//! # crawrad-runtime-mock — the mock backend
//!
//! A [`Runtime`] that does not draw anywhere: records what it received and
//! returns the form as **text**.
//!
//! It exists for a single reason, and it is an architectural one: as long as there is
//! only one real runtime, the [`crawrad_runtime::Runtime`] boundary leaks without
//! anyone noticing — some Win32 detail creeps up to the designer and is only
//! discovered the day the second runtime arrives. The mock is continuous
//! proof that the boundary remains honest (ADR 0002).
//!
//! As a bonus, it makes the **entire vertical slice testable without a window**:
//! document + designer + registry + runtime, in CI, without a GUI.

// This crate has no `unsafe`, and the compiler now guarantees it. CrawRAD
// does not have A SINGLE line of unsafe code: the FFI adapter was removed in ADR 0013 — the rest of the system is safe Rust, and this boundary is as
// important as that of ADR 0002.
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

use crawrad_ir::{Document, NodeId, Rect};
use crawrad_note::note;
use crawrad_runtime::{Runtime, RuntimeError, RuntimeEvent, Scale, ViewportPx};

#[note("mockctl")]
#[derive(Debug, Clone, PartialEq)]
pub struct MockCtl {
    pub class: String,
    pub rect: Rect,
    pub text: String,
    pub visible: bool,
    pub enabled: bool,
}

#[derive(Debug, Default)]
pub struct MockRuntime {
    ctls: BTreeMap<NodeId, MockCtl>,
    ordem: Vec<NodeId>,
    sel: Vec<NodeId>,
    viewport: Option<ViewportPx>,
    scale: f32,
    /// Everything that crossed the boundary, for tests to inspect.
    ///
    /// **Remains in Portuguese, and this is an intentional decision, not an oversight.** Our rules require
    /// writing in English what the user READS, and no one reads this: the sole
    /// reader is the suite (`tests/fatia_vertical.rs:379` compares
    /// `"update(1 tocados)"` character by character). Translating would touch a literal
    /// that serves as a test contract, in a batch that was only supposed to translate
    /// user-facing UI. If the log ever appears somewhere, it becomes a message and
    /// falls under the same rule as the rest.
    pub log: Vec<String>,
    /// Events that the test wants to inject as if coming from the user.
    pendentes: Vec<RuntimeEvent>,
    aberto: bool,
    marquee: Option<Rect>,
    guias: Vec<crawrad_runtime::GuideLine>,
    badges: Vec<(NodeId, String)>,
    modo: crawrad_runtime::Mode,
}

impl MockRuntime {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            ..Default::default()
        }
    }

    #[note("mockruntime-injetar")]
    pub fn injetar(&mut self, e: RuntimeEvent) {
        self.pendentes.push(e);
    }

    pub fn ctl(&self, id: &str) -> Option<&MockCtl> {
        self.ctls.get(id)
    }

    pub fn quantidade(&self) -> usize {
        self.ctls.len()
    }

    pub fn selecao(&self) -> &[NodeId] {
        &self.sel
    }

    pub fn marquee(&self) -> Option<Rect> {
        self.marquee
    }

    pub fn guias(&self) -> &[crawrad_runtime::GuideLine] {
        &self.guias
    }

    pub fn badges(&self) -> &[(NodeId, String)] {
        &self.badges
    }

    #[note("mockruntime-modo")]
    pub fn modo(&self) -> crawrad_runtime::Mode {
        self.modo
    }

    #[note("mockruntime-render")]
    pub fn render(&self) -> String {
        const CEL: f32 = 8.0;
        let (mut cols, mut linhas) = (40usize, 12usize);
        for c in self.ctls.values() {
            cols = cols.max(((c.rect.x + c.rect.w) / CEL).ceil() as usize + 1);
            linhas = linhas.max(((c.rect.y + c.rect.h) / CEL).ceil() as usize + 1);
        }
        let mut tela = vec![vec![b'.'; cols]; linhas];

        for id in &self.ordem {
            let Some(c) = self.ctls.get(id) else { continue };
            if !c.visible {
                continue;
            }
            let x0 = (c.rect.x / CEL).round() as isize;
            let y0 = (c.rect.y / CEL).round() as isize;
            let x1 = ((c.rect.x + c.rect.w) / CEL).round() as isize - 1;
            let y1 = ((c.rect.y + c.rect.h) / CEL).round() as isize - 1;
            let selecionado = self.sel.contains(id);

            for y in y0..=y1 {
                for x in x0..=x1 {
                    if y < 0 || x < 0 || y as usize >= linhas || x as usize >= cols {
                        continue;
                    }
                    let borda = y == y0 || y == y1 || x == x0 || x == x1;
                    tela[y as usize][x as usize] = match (borda, selecionado) {
                        (true, true) => b'#',
                        (true, false) => b'+',
                        (false, _) => b' ',
                    };
                }
            }
            // label inside the component
            let rotulo = if c.text.is_empty() {
                id.clone()
            } else {
                c.text.replace('&', "")
            };
            let ty = ((y0 + y1) / 2).max(0) as usize;
            let tx = (x0 + 1).max(0) as usize;
            if ty < linhas {
                for (i, ch) in rotulo.bytes().enumerate() {
                    let px = tx + i;
                    if px >= cols || px as isize >= x1 {
                        break;
                    }
                    tela[ty][px] = ch;
                }
            }
        }

        tela.into_iter()
            .map(|l| String::from_utf8_lossy(&l).trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn sincronizar(&mut self, doc: &Document, id: &NodeId) {
        let Some(n) = doc.node(id) else { return };
        let mut ctl = MockCtl {
            class: n.class.clone(),
            rect: n.rect().unwrap_or(Rect::new(0.0, 0.0, 8.0, 8.0)),
            text: String::new(),
            visible: true,
            enabled: true,
        };
        // Only what the contract delivers to THIS target. `win32.*` property does not
        // arrive here — and that is precisely what needs to be proven.
        for (chave, valor) in n.props_for("mock") {
            match (chave.to_string().as_str(), valor) {
                ("text", crawrad_ir::Value::Str(s)) => ctl.text = s.clone(),
                ("visible", crawrad_ir::Value::Bool(b)) => ctl.visible = *b,
                ("enabled", crawrad_ir::Value::Bool(b)) => ctl.enabled = *b,
                _ => {}
            }
        }
        self.ctls.insert(id.clone(), ctl);
        if !self.ordem.contains(id) {
            self.ordem.push(id.clone());
        }
    }
}

impl Runtime for MockRuntime {
    fn target(&self) -> &'static str {
        "mock"
    }

    #[note("mockruntime-classes")]
    fn classes(&self) -> &'static [&'static str] {
        &[]
    }

    fn open(&mut self, doc: &Document) -> Result<(), RuntimeError> {
        self.ctls.clear();
        self.ordem.clear();
        let raiz = doc
            .node(&doc.root)
            .ok_or_else(|| RuntimeError::Backend("the document has no root".into()))?;
        for id in raiz.children.clone() {
            self.sincronizar(doc, &id);
        }
        self.aberto = true;
        self.log.push(format!("open({} nodes)", self.ctls.len()));
        Ok(())
    }

    fn update(&mut self, doc: &Document, touched: &BTreeSet<NodeId>) -> Result<(), RuntimeError> {
        if !self.aberto {
            return Err(RuntimeError::NotStarted);
        }
        let conhecidos: Vec<NodeId> = self.ordem.clone();
        for id in conhecidos {
            if doc.node(&id).is_none() {
                self.ctls.remove(&id);
                self.ordem.retain(|o| o != &id);
            }
        }
        let raiz = doc
            .node(&doc.root)
            .ok_or_else(|| RuntimeError::Backend("the document has no root".into()))?;
        for id in raiz.children.clone() {
            if touched.contains(&id) || !self.ctls.contains_key(&id) {
                self.sincronizar(doc, &id);
            }
        }
        self.log.push(format!("update({} tocados)", touched.len()));
        Ok(())
    }

    fn set_viewport(&mut self, vp: ViewportPx, scale: Scale) {
        self.viewport = Some(vp);
        self.scale = scale.0;
        self.log
            .push(format!("viewport({}x{} @{})", vp.w, vp.h, scale.0));
    }

    fn set_selection(&mut self, sel: &[NodeId]) {
        self.sel = sel.to_vec();
        self.log.push(format!("selection({})", sel.join(",")));
    }

    fn set_marquee(&mut self, rect: Option<Rect>) {
        self.marquee = rect.filter(|r| r.w > 0.0 || r.h > 0.0);
        self.log.push(match self.marquee {
            Some(r) => format!("marquee({}x{})", r.w, r.h),
            None => "marquee(nenhum)".into(),
        });
    }

    fn set_badges(&mut self, badges: &[(NodeId, String)]) {
        self.badges = badges.to_vec();
    }

    fn set_grid(&mut self, grid: f32) {
        self.log.push(format!("grid({grid})"));
    }

    fn set_mode(&mut self, mode: crawrad_runtime::Mode) {
        self.modo = mode;
    }

    fn set_guides(&mut self, guias: &[crawrad_runtime::GuideLine]) {
        self.guias = guias.to_vec();
    }

    fn drain_events(&mut self) -> Vec<RuntimeEvent> {
        std::mem::take(&mut self.pendentes)
    }

    fn close(&mut self) {
        self.ctls.clear();
        self.ordem.clear();
        self.aberto = false;
        self.log.push("close".into());
    }
}
