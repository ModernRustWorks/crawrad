//! Phase 1 vertical slice, proven **without opening a window**.
//!
//! Registry → document → designer → runtime, the same path that the IDE
//! traverses when you drag a component. What the IDE adds on top of
//! this is just drawing: if this test passes, the RAD logic stands.
//!
//! The human gate of the phase remains the 60 recorded seconds. This is the
//! machine gate, which runs on every commit.

use crawrad_designer::Designer;
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::{Command, Doc, Transaction};
use crawrad_ir::{Document, Rect, Value};
use crawrad_registry::Registry;
use crawrad_runtime::{Runtime, RuntimeEvent};
use crawrad_runtime_mock::MockRuntime;

/// The IDE in miniature: the same pieces, without the egui shell.
struct Ide {
    doc: Doc,
    designer: Designer,
    registry: Registry,
    rt: MockRuntime,
    /// What the shell's status bar would say. Exists since `Lock Controls`:
    /// a rejected gesture must PRODUCE a phrase, and a test that does not read it
    /// would not be measuring what the user receives.
    status: Option<String>,
    /// `Preview` enabled. Both shells store exactly this boolean —
    /// see `crawrad-ide/src/ide_app.rs` (Makepad) and `main.rs` (egui) — because
    /// `Efeito` is a TOGGLE, and whatever toggles needs to know where it starts from.
    vivo: bool,
}

impl Ide {
    fn nova() -> Self {
        let mut doc = Document::new("form_main", "Form");
        doc.node_mut("form_main")
            .unwrap()
            .set_rect(Rect::new(0.0, 0.0, 320.0, 160.0));
        let mut rt = MockRuntime::new();
        rt.open(&doc).unwrap();
        Self {
            doc: Doc::new(doc),
            designer: Designer::default(),
            registry: Registry::builtin(),
            rt,
            status: None,
            vivo: false,
        }
    }

    /// The mutation path, in the same shape as the shell: lock gate first,
    /// `apply` later.
    ///
    /// `recusa` is called HERE, in the applier, and not in each producer — it is the
    /// same line that the shell's `Ide::aplicar` has. Two shells, **one**
    /// rule: that is what making `recusa` public upholds, and that is why
    /// this harness can prove locked `Align ▸ Left` without rewriting
    /// any decision.
    fn aplicar(&mut self, tx: Transaction) {
        if self.designer.refuses(self.doc.ir(), &tx) {
            self.status = Some(crawrad_designer::LOCKED_WARNING.to_string());
            return;
        }
        let ap = self.doc.apply(tx).expect("valid transaction");
        self.rt.update(self.doc.ir(), &ap.touched).unwrap();
    }

    /// `Ctrl+Z`, and it does NOT go through the applier — as in the shell.
    ///
    /// Purposely mirrors the shell's `Ide::desfazer`: if one day someone
    /// unifies both paths by routing undo through `aplicar`, this is where
    /// the change lands, and the locked `Ctrl+Z` test fails with it. A test
    /// calling `doc.undo()` directly would not catch this refactoring — it
    /// would measure `crawrad-doc`, and the defect would be in the shell.
    fn desfazer(&mut self) {
        let ap = self.doc.undo().expect("undo");
        self.rt.update(self.doc.ir(), &ap.touched).unwrap();
    }

    fn refazer(&mut self) {
        let ap = self.doc.redo().expect("redo");
        self.rt.update(self.doc.ir(), &ap.touched).unwrap();
    }

    /// What clicking a menu item does: gets the action by id from the catalog,
    /// executes, and applies the `Efeito` — the entire [ADR 0005] path.
    fn executar(&mut self, id: &str) {
        let acao = crawrad_actions::by_id(id).expect("action in catalogue");
        let area = Clipboard::default();
        let efeito = {
            let c = crawrad_actions::Context {
                doc: Some(&self.doc),
                registry: &self.registry,
                clipboard: &area,
                args: crawrad_actions::Args::none(),
                project: crawrad_actions::Project::default(),
                author: crawrad_actions::Author::User,
                editor_dirty: false,
                text: Default::default(),
            };
            assert!(
                acao.enabled.allows(&c),
                "{id} should be enabled — the menu would show it grayed out"
            );
            (acao.execute)(&c)
        };
        match efeito.designer {
            Some(crawrad_actions::DesignerMode::ToggleLock) => {
                self.designer.locked = !self.designer.locked;
            }
            Some(crawrad_actions::DesignerMode::ToggleLive) => {
                self.vivo = !self.vivo;
                self.rt.set_mode(if self.vivo {
                    crawrad_runtime::Mode::Live
                } else {
                    crawrad_runtime::Mode::Design
                });
            }
            None => {}
        }
        if let Some(tx) = efeito.transaction {
            self.aplicar(tx);
        }
        if let Some(sel) = efeito.selection {
            self.doc.select(sel);
        }
    }

    /// What the shell does when you choose from the palette and click the form.
    fn soltar(&mut self, class: &str, x: f32, y: f32) -> String {
        let (w, h) = self.registry.get(class).unwrap().default_size;
        let rect = self.designer.drop_rect(x, y, w, h);
        let id = self
            .registry
            .suggest_id(class, |i| self.doc.ir().node(i).is_some());
        let node = self.registry.instantiate(class, id.clone(), rect).unwrap();
        let raiz = self.doc.ir().root.clone();
        let indice = self.doc.ir().node(&raiz).unwrap().children.len();
        self.aplicar(
            Transaction::user(format!("add {class}")).with(Command::AddNode {
                parent: raiz,
                node,
                index: indice,
            }),
        );
        self.doc.select_one(id.clone());
        self.rt.set_selection(self.doc.selection());
        id
    }

    /// A complete mouse gesture, as it comes from the design surface.
    fn gesto(&mut self, de: (f32, f32), ate: (f32, f32)) {
        self.rt.injetar(RuntimeEvent::PointerDown {
            x: de.0,
            y: de.1,
            ctrl: false,
        });
        self.rt.injetar(RuntimeEvent::PointerMove {
            x: ate.0,
            y: ate.1,
            free: false,
        });
        self.rt
            .injetar(RuntimeEvent::PointerUp { x: ate.0, y: ate.1 });
        self.bombear();
    }

    /// The SAME gesture, but with the intermediate frames that a real
    /// mouse produces.
    ///
    /// [`Ide::gesto`] fires **one** `PointerMove`, and therefore it measured for
    /// a long time a drag that no one does: the hand crosses the screen across
    /// dozens of frames, and each one becomes a transaction. It is rule 9
    /// applied to the gesture rather than to the document — being born as the user is born applies
    /// to what they do, not just what they create.
    fn gesto_continuo(&mut self, de: (f32, f32), ate: (f32, f32), quadros: usize) {
        assert!(quadros >= 1);
        self.rt.injetar(RuntimeEvent::PointerDown {
            x: de.0,
            y: de.1,
            ctrl: false,
        });
        for i in 1..=quadros {
            let t = i as f32 / quadros as f32;
            self.rt.injetar(RuntimeEvent::PointerMove {
                x: de.0 + (ate.0 - de.0) * t,
                y: de.1 + (ate.1 - de.1) * t,
                free: false,
            });
        }
        self.rt
            .injetar(RuntimeEvent::PointerUp { x: ate.0, y: ate.1 });
        self.bombear();
    }

    fn bombear(&mut self) {
        for ev in self.rt.drain_events() {
            let sel: Vec<_> = self.doc.selection().to_vec();
            let out = match ev {
                RuntimeEvent::PointerDown { x, y, ctrl } => {
                    self.designer.pointer_down(self.doc.ir(), &sel, x, y, ctrl)
                }
                RuntimeEvent::PointerMove { x, y, .. } => {
                    self.designer.pointer_move(self.doc.ir(), x, y)
                }
                RuntimeEvent::PointerUp { .. } => self.designer.pointer_up(self.doc.ir()),
                RuntimeEvent::PointerDblClick { .. } => crawrad_designer::Outcome::default(),
                RuntimeEvent::Cancel => self.designer.cancel(),
            };
            if let Some(nova) = out.select {
                self.doc.select(nova);
                self.rt.set_selection(self.doc.selection());
            }
            if let Some(m) = out.marquee {
                self.rt.set_marquee(Some(m));
            }
            if let Some(tx) = out.tx {
                self.aplicar(tx);
            }
        }
    }

    fn rect(&self, id: &str) -> Rect {
        self.doc.ir().node(id).unwrap().rect().unwrap()
    }
}

#[test]
fn o_ciclo_inteiro_do_rad_sem_abrir_janela() {
    let mut ide = Ide::nova();

    // 1. three components coming from the palette
    let lbl = ide.soltar("Label", 16.0, 16.0);
    let edt = ide.soltar("Edit", 16.0, 40.0);
    let btn = ide.soltar("Button", 16.0, 80.0);
    assert_eq!(ide.rt.quantidade(), 3, "the runtime received all three");

    // 2. default text came from the descriptor, not from IDE code
    assert_eq!(
        ide.rt.ctl(&btn).unwrap().text,
        btn,
        "born with its own name"
    );

    // 3. drag the button (from the center, away from handles)
    let antes = ide.rect(&btn);
    ide.gesto((64.0, 96.0), (144.0, 96.0));
    assert_eq!(ide.rect(&btn).x, antes.x + 80.0);
    assert_eq!(
        ide.rect(&btn).y,
        antes.y,
        "dragging horizontally does not affect y"
    );
    assert_eq!(
        ide.rt.ctl(&btn).unwrap().rect,
        ide.rect(&btn),
        "the runtime matched the document"
    );

    // 4. edit property via inspector
    ide.aplicar(Transaction::user("caption").with(Command::SetProp {
        node: btn.clone(),
        key: "text".into(),
        value: Some(Value::Str("Save".into())),
    }));
    assert_eq!(ide.rt.ctl(&btn).unwrap().text, "Save");

    // 5. bind an event
    ide.aplicar(Transaction::user("event").with(Command::SetEvent {
        node: btn.clone(),
        event: "on_click".into(),
        handler: Some("btn_1Click".into()),
    }));

    // 6. undoing three times dismantles what was done, in order
    ide.doc.undo().unwrap(); // event
    ide.doc.undo().unwrap(); // caption
    assert_eq!(ide.doc.ir().node(&btn).unwrap().text(), Some(btn.as_str()));
    ide.doc.undo().unwrap(); // drag
    assert_eq!(ide.rect(&btn), antes);

    // 7. saving and reopening preserves everything
    let json = ide.doc.ir().to_json().unwrap();
    let relido = Document::from_json(&json).unwrap();
    assert_eq!(
        &relido,
        ide.doc.ir(),
        "the .crawform loses nothing along the way"
    );
    assert!(relido.node(&lbl).is_some() && relido.node(&edt).is_some());
}

#[test]
fn o_fantoche_desenha_o_formulario_como_o_documento_manda() {
    let mut ide = Ide::nova();
    ide.soltar("Label", 16.0, 16.0);
    ide.soltar("Button", 16.0, 64.0);
    ide.doc.clear_selection();
    ide.rt.set_selection(&[]);

    let tela = ide.rt.render();
    let esperado = "\
........................................
........................................
..+++++++++++++.........................
..+label1     +.........................
..+++++++++++++.........................
........................................
........................................
........................................
..++++++++++++..........................
..+button1   +..........................
..+          +..........................
..++++++++++++..........................
........................................";
    // normalizes trailing whitespace: what matters is the drawing
    let n = |s: &str| {
        s.lines()
            .map(|l| l.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(n(&tela), n(esperado), "\n--- got ---\n{tela}\n");
}

#[test]
fn selecao_aparece_no_desenho() {
    let mut ide = Ide::nova();
    let btn = ide.soltar("Button", 16.0, 16.0);
    ide.doc.select_one(btn);
    ide.rt.set_selection(ide.doc.selection());
    assert!(
        ide.rt.render().contains('#'),
        "selected component is drawn with double border"
    );
}

#[test]
fn propriedade_de_outro_alvo_nao_atravessa_a_fronteira() {
    // Proof of ADR 0002: `win32.PasswordChar` exists in the document, but the
    // `mock` runtime never sees it. If someone ever "simplifies" the contract,
    // this test fails.
    let mut ide = Ide::nova();
    let edt = ide.soltar("Edit", 16.0, 16.0);
    ide.aplicar(Transaction::user("password").with(Command::SetProp {
        node: edt.clone(),
        key: "win32.PasswordChar".into(),
        value: Some(Value::Str("*".into())),
    }));

    let no = ide.doc.ir().node(&edt).unwrap();
    assert!(
        no.props.contains_key("win32.PasswordChar"),
        "the document keeps the specific property"
    );
    let vistas: Vec<String> = no.props_for("mock").map(|(k, _)| k.to_string()).collect();
    assert!(
        !vistas.iter().any(|k| k.starts_with("win32.")),
        "but the mock runtime receives nothing from win32: {vistas:?}"
    );
    assert!(
        vistas.contains(&"text".to_string()),
        "the common core passes"
    );
}

#[test]
fn remover_componente_some_da_tela() {
    let mut ide = Ide::nova();
    let btn = ide.soltar("Button", 16.0, 16.0);
    assert_eq!(ide.rt.quantidade(), 1);

    ide.aplicar(Transaction::user("remove").with(Command::RemoveNode { node: btn.clone() }));
    assert_eq!(ide.rt.quantidade(), 0, "the runtime removed the control");

    // and undo brings it back
    let ap = ide.doc.undo().unwrap();
    ide.rt.update(ide.doc.ir(), &ap.touched).unwrap();
    assert_eq!(ide.rt.quantidade(), 1);
    assert_eq!(
        ide.rt.ctl(&btn).unwrap().text,
        btn,
        "born with its own name"
    );
}

#[test]
fn a_fronteira_recebe_so_o_que_mudou() {
    let mut ide = Ide::nova();
    ide.soltar("Button", 16.0, 16.0);
    let b = ide.soltar("Button", 16.0, 64.0);
    ide.rt.log.clear();

    ide.aplicar(Transaction::user("move only one").with(Command::SetRect {
        node: b.clone(),
        rect: Rect::new(100.0, 64.0, 96.0, 32.0),
    }));
    assert_eq!(
        ide.rt.log,
        vec!["update(1 tocados)".to_string()],
        "updating a single component must not cost the entire form"
    );
}

#[test]
fn com_o_lock_ligado_o_mesmo_gesto_para_de_mover_e_o_arquivo_nao_muda() {
    // `Edit ▸ Lock Controls` measured in the user path, rather than in a
    // hand-assembled `Document`: the component is born via the palette's `instantiate()`
    // and the gesture enters as a pointer event, as in the IDE.
    //
    // The check comes first on purpose. A test that merely asserted "locked
    // does not move" would pass with gestures broken for everyone.
    let mut ide = Ide::nova();
    let btn = ide.soltar("Button", 16.0, 16.0);
    let inicial = ide.rect(&btn);
    ide.gesto((48.0, 32.0), (128.0, 32.0));
    let solto = ide.rect(&btn);
    assert_ne!(solto.x, inicial.x, "unlocked, the gesture moves");

    // Through the CATALOG, as the menu item does — not by writing to the field.
    ide.executar("edit.lock_controls");
    assert!(ide.designer.locked, "the menu action must enable the lock");

    // The form as it goes to disk, BEFORE the rejected gesture.
    let antes = ide.doc.ir().to_json().unwrap();
    let revisao = ide.doc.revision();

    ide.gesto((160.0, 32.0), (240.0, 32.0));
    assert_eq!(
        ide.rect(&btn),
        solto,
        "locked, the same gesture does not move"
    );
    assert_eq!(
        ide.rt.ctl(&btn).unwrap().rect,
        solto,
        "and the runtime received no movement"
    );
    assert_eq!(
        ide.doc.revision(),
        revisao,
        "a rejected gesture cannot become an undo step"
    );
    assert_eq!(
        ide.doc.ir().to_json().unwrap(),
        antes,
        "the lock is designer state: the .crawform comes out byte-for-byte identical, and \
         so does the generated code"
    );

    // And what lock does NOT lock remains valid: the marquee on the form's
    // background still selects, which is what keeps the Object Inspector alive.
    ide.doc.select(Vec::new());
    ide.gesto((300.0, 150.0), (8.0, 8.0));
    assert_eq!(
        ide.doc.selection(),
        std::slice::from_ref(&btn),
        "locked, selection remains allowed"
    );
}

#[test]
fn travado_o_align_do_menu_tambem_para_e_diz_por_que() {
    // The flaw that failed the first version of the command, measured across
    // the whole path: palette → document → `Edit` menu → catalog → `Efeito` →
    // applier. `Align ▸ Left` sits two items above `Lock controls`
    // itself, in the SAME menu, and it moved the entire form while the banner
    // stated "controls locked".
    let mut ide = Ide::nova();
    let a = ide.soltar("Button", 16.0, 16.0);
    let b = ide.soltar("Button", 120.0, 80.0);
    ide.doc.select(vec![a.clone(), b.clone()]);
    assert_ne!(
        ide.rect(&a).x,
        ide.rect(&b).x,
        "both must be misaligned, otherwise Align produces nothing and \
         the test would pass without measuring anything"
    );

    // Locks FIRST, and then attempts to align.
    ide.executar("edit.lock_controls");
    let antes = ide.doc.ir().to_json().unwrap();
    let revisao = ide.doc.revision();
    ide.status = None;

    ide.executar("align.left");
    assert_eq!(
        ide.status.as_deref(),
        Some(crawrad_designer::LOCKED_WARNING),
        "the rejected command must explain, otherwise the banner lies and the menu \
         seems broken"
    );
    assert_eq!(ide.doc.revision(), revisao);
    assert_eq!(
        ide.doc.ir().to_json().unwrap(),
        antes,
        "not a single `SetRect` entered the document"
    );

    // Unlocks and repeats: this is the contrast that proves the transaction existed and
    // would move — without it, a broken `align.left` would pass this test.
    ide.executar("edit.lock_controls");
    assert!(!ide.designer.locked);
    ide.executar("align.left");
    assert_eq!(
        ide.rect(&b).x,
        ide.rect(&a).x,
        "unlocked, the same command aligns"
    );
}

#[test]
fn travado_o_ctrl_z_continua_desfazendo() {
    // **A lock that freezes Ctrl+Z is worse than no lock at all**, and this test
    // exists because the gate was placed in the applier — the same place where the
    // user's transaction passes. If one day `desfazer` is routed through there
    // "to unify", undo dies when lock is enabled, and the symptom would be a
    // person locking controls and losing the escape route from the mistake they just
    // made.
    //
    // Today it bypasses: `Doc::undo` does not go through `aplicar`. The bypass is
    // recorded as a decision on both sides; this is what makes the decision
    // stop depending on someone having read the comment.
    let mut ide = Ide::nova();
    let id = ide.soltar("Button", 20.0, 20.0);
    let inicial = ide.rect(&id);

    // Moves UNLOCKED — this is the damage the user will want to undo.
    ide.gesto((25.0, 25.0), (95.0, 65.0));
    let movido = ide.rect(&id);
    assert_ne!(movido, inicial, "the unlocked gesture had to move");

    // Now locks, as someone who finished the layout.
    ide.executar("edit.lock_controls");
    assert!(ide.designer.locked, "the command had to enable the lock");

    // And regrets it. Ctrl+Z must work regardless.
    ide.desfazer();
    assert_eq!(
        ide.rect(&id),
        inicial,
        "with the lock enabled Ctrl+Z stopped undoing — the gate swallowed the \
         history, and the user was trapped in the damage they wanted to undo"
    );

    // And redo as well, otherwise the pair is half-broken.
    ide.refazer();
    assert_eq!(ide.rect(&id), movido, "redo broke as well");

    // Contrast: the lock remains valid after all this. Without this line,
    // the test would pass with `lock_controls` having no effect at all.
    ide.gesto((100.0, 70.0), (160.0, 120.0));
    assert_eq!(
        ide.rect(&id),
        movido,
        "the lock had to remain in effect after undo"
    );
}

#[test]
fn um_arrasto_e_um_ctrl_z() {
    // **Two drags wiped the user's entire history**, and the path
    // there was short: the IDE applies a transaction per drag FRAME, each
    // pushed an inverse, and `max_undo` is 100. A drag measured on screen
    // took the document from `rev 0` to `rev 70`. Two of them evicted from the stack
    // even the `AddNode` that created the component.
    //
    // The symptom the user saw first was sillier and equally bad: a
    // `Ctrl+Z` undid ONE FRAME — the button did not move visibly, and
    // one had to hold the shortcut for the gesture to rewind.
    //
    // The doc-comment on `pointer_move` already promised "history merges everything on
    // `pointer_up`". It described behavior that did not exist.
    let mut ide = Ide::nova();
    let id = ide.soltar("Button", 20.0, 20.0);
    let inicial = ide.rect(&id);
    let antes = ide.doc.revision();

    // Twenty frames, which is an actual short drag.
    ide.gesto_continuo((25.0, 25.0), (185.0, 105.0), 20);
    let movido = ide.rect(&id);
    assert_ne!(movido, inicial, "the drag had to move");
    assert!(
        ide.doc.revision() > antes,
        "the drag had to change the document"
    );

    // ONE. The entire gesture is undone, because it was a single gesture the user
    // performed — they did not drag seventy times.
    ide.desfazer();
    assert_eq!(
        ide.rect(&id),
        inicial,
        "one Ctrl+Z had to undo the entire drag, but it only undid a single frame"
    );

    // And redo is its pair: a single one restores the entire gesture.
    ide.refazer();
    assert_eq!(ide.rect(&id), movido, "one Ctrl+Y had to redo the gesture");
}

#[test]
fn dois_arrastos_sao_dois_passos_e_nao_cento_e_quarenta() {
    // The contrast of the previous test, and it is the half that matters: a
    // history that merged EVERYTHING into one step would pass there and be equally
    // broken. A separate gesture is a separate step.
    //
    // And here is where the real damage appeared. `max_undo` is 100; at 70 revisions
    // per drag, the second gesture already evicted the `AddNode` that created the
    // component — the user lost the entire history in two mouse
    // movements, without any warning.
    let mut ide = Ide::nova();
    let id = ide.soltar("Button", 20.0, 20.0);
    let inicial = ide.rect(&id);

    ide.gesto_continuo((25.0, 25.0), (105.0, 25.0), 20);
    let primeiro = ide.rect(&id);
    ide.gesto_continuo((110.0, 30.0), (110.0, 110.0), 20);
    let segundo = ide.rect(&id);
    assert_ne!(primeiro, inicial);
    assert_ne!(segundo, primeiro);

    ide.desfazer();
    assert_eq!(
        ide.rect(&id),
        primeiro,
        "the first Ctrl+Z undoes the 2nd gesture"
    );
    ide.desfazer();
    assert_eq!(
        ide.rect(&id),
        inicial,
        "the second Ctrl+Z undoes the 1st gesture"
    );

    // And `AddNode` remains on the stack: it was what history was losing.
    ide.desfazer();
    assert!(
        ide.doc.ir().node(&id).is_none(),
        "the third Ctrl+Z had to undo the component creation — that was \
         precisely what drags were pushing out of the history"
    );
}

#[test]
fn a_seta_do_teclado_continua_sendo_um_passo_por_toque() {
    // The gesture stamp belongs to DRAGGING, and not to everything that modifies a rectangle.
    // A key is a discrete act: pressing an arrow three times expects three
    // undo steps, and grouping them would fix dragging by breaking the keyboard.
    let mut ide = Ide::nova();
    let id = ide.soltar("Button", 20.0, 20.0);
    let inicial = ide.rect(&id);

    for _ in 0..3 {
        let sel = ide.doc.selection().to_vec();
        if let Some(tx) = ide.designer.nudge(ide.doc.ir(), &sel, 1.0, 0.0) {
            ide.aplicar(tx);
        }
    }
    let depois = ide.rect(&id);
    assert_ne!(depois, inicial, "arrow keys had to move");

    ide.desfazer();
    assert_ne!(
        ide.rect(&id),
        inicial,
        "one Ctrl+Z after three arrows reverted everything: gesture stamp leaked \
         to the keyboard"
    );
    ide.desfazer();
    ide.desfazer();
    assert_eq!(ide.rect(&id), inicial, "three arrows, three undos");
}

#[test]
fn o_preview_do_menu_alterna_o_modo_do_runtime() {
    // The Makepad shell's `Preview` **never worked** — the owner reported
    // it like that, in those exact words. And the reason was not broken plumbing:
    // `Preview` was a status BADGE (`ide_app.rs`, `text: "Preview"`), never
    // a button. There was no action in the catalog to enable live mode.
    //
    // This test measures the full path of [ADR 0005] — catalog → `Efeito` →
    // runtime — without opening a window, and is what prevents the badge from returning: a
    // `view.toggle_live` that does not reach the adapter fails here.
    let mut ide = Ide::nova();
    ide.soltar("Button", 16.0, 16.0);
    assert_eq!(
        ide.rt.modo(),
        crawrad_runtime::Mode::Design,
        "the IDE opens in design — that is the state of a classic RAD IDE when loading the form"
    );

    // Through the CATALOG, as clicking the button does. `executar` already requires that the
    // action is ENABLED: without a document it is grayed out, and here is where
    // a loose `habilitada` would show up.
    ide.executar("view.toggle_live");
    assert!(ide.vivo, "the menu action must enable live mode");
    assert_eq!(
        ide.rt.modo(),
        crawrad_runtime::Mode::Live,
        "the shell toggled the boolean but did not notify the adapter — on screen the \
         form would still have ruler, handles and grid, which is exactly the \
         owner's complaint about Preview"
    );

    // And toggles back. A toggle that only goes one way is half a solution: in a classic RAD IDE
    // `F9`/`Stop` restores the artboard, and without this half the user enables
    // preview and gets stuck in it.
    ide.executar("view.toggle_live");
    assert!(!ide.vivo, "the second click must return to design");
    assert_eq!(
        ide.rt.modo(),
        crawrad_runtime::Mode::Design,
        "the runtime stayed live forever — ruler, handles and grid do not return"
    );
}
