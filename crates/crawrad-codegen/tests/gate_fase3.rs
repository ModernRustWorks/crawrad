//! The Phase 3 gate, proven end-to-end and headless.
//!
//! Generator unit tests check the output **text**. This test checks the
//! only thing the user sees: the generated project **compiles**, and
//! clicking the button they drew **executes the function they wrote**.
//!
//! The same applies to what is not clicked: the `Timer` tick, the form's
//! `OnCreate` — which must run once and AFTER designer values — and
//! `OnClose`, which must reach the handler on the frame close was requested.
//!
//! Marked `#[ignore]` intentionally — it calls `cargo` from within `cargo`,
//! downloads `eframe` on the first run, and takes minutes. Run whenever you want
//! the proof:
//!
//! ```text
//! cargo test -p crawrad-codegen --test gate_fase3 -- --ignored --nocapture
//! ```
//!
//! ## NETWORK: this gate may need it
//!
//! Since ADR 0018,
//! project creation is done by `cargo init` and dependencies are added by
//! `cargo add` — and `cargo add` queries the crates.io index. The gate tries
//! `--offline` first, which resolves against the index already downloaded on the machine,
//! and falls back to online only when the local cache misses. On a machine that
//! already compiled this workspace (which depends on `eframe 0.36`), the offline
//! path works; on a clean machine without network the gate **fails**, and fails
//! explicitly stating so. Note that `cargo test` in the next step would also
//! need to download `eframe`, so offline never got very far here anyway.

// The rule shared by this gate and `gate_sem_avisos`. It used to live here as
// a copy, and the copy diverged in `4959652`: this file replaced `contains` with
// equality while the sibling lagged behind. Why it is a single function, rather
// than two proven separately, is documented in `comum/mod.rs`.
mod comum;

use comum::manifesto_intocado;
use crawrad_codegen::Gerador as _;
use crawrad_ir::{Document, Node, Rect};
use std::path::{Path, PathBuf};
use std::process::Command;

/// The minimal form that satisfies the gate: a field and a button wired to an
/// event. It is what results from dragging two components and typing an `OnClick`.
///
/// Since `Timer`, it also includes a **running non-visual component**: a second field
/// and a 100 ms timer wired to `OnTimer`. Run-time behavior is something no other
/// component of ours generates, and this gate is the only place that proves it the
/// way that matters — by compiling and running.
///
/// And since the page guard fix that did not compose, it includes **nested tabs**
/// (see [`abas_aninhadas`]): a button at the bottom of a `PageControl` that lives
/// inside a `TabSheet`. That it disappears when the outer tab is closed is not a
/// question of emitted text — it means the pixel is not painted and the click
/// finds nothing, and this is the only place where that can be measured.
///
/// And since FORM events, it wires `OnCreate` and `OnClose` on the root. They are
/// not a matter of emitted text either: what matters for `OnCreate` is ORDER —
/// designer values first, the handler second —, and for `OnClose` that the close
/// request reaches the handler exactly once.
fn documento(com_clique: bool) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").unwrap();
        raiz.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        raiz.set_name("form_main");
        raiz.set_text("Cadastro");
        // ROOT EVENTS. They remain wired in BOTH documents — with click
        // and without —, because step 4 of the gate disables `on_click` to
        // prove that the click test is valid, and mixing concerns there would
        // make failures ambiguous about which one broke.
        raiz.events
            .insert("on_create".into(), "form_mainCreate".into());
        raiz.events
            .insert("on_close".into(), "form_mainClose".into());
    }

    // DESIGNER text. This is not decoration: it is half the proof that
    // `OnCreate` runs AFTER design values — the handler concatenates, and
    // inverted order would wipe this word out.
    let mut edt = Node::new("edt1", "Edit");
    edt.set_name("edt_nome");
    edt.set_text("designer");
    edt.set_rect(Rect::new(24.0, 48.0, 280.0, 24.0));
    d.add_child("form_main", edt).unwrap();

    // Where `OnClose` counts close requests. Same reason as the timer
    // counter: the handler author does not add fields to the generated
    // struct, so test state lives where application state lives.
    let mut fec = Node::new("edt3", "Edit");
    fec.set_name("edt_fechou");
    fec.set_text("");
    fec.set_rect(Rect::new(150.0, 144.0, 96.0, 24.0));
    d.add_child("form_main", fec).unwrap();

    let mut btn = Node::new("btn1", "Button");
    btn.set_name("btn_ok");
    btn.set_text("&OK");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    if com_clique {
        btn.events.insert("on_click".into(), "btn_okClick".into());
    }
    d.add_child("form_main", btn).unwrap();

    // The tick counter. Needs to be a form component: whoever writes the
    // handler cannot add fields to the generated struct, so test state lives
    // where application state lives.
    let mut tks = Node::new("edt2", "Edit");
    tks.set_name("edt_tiques");
    tks.set_text("");
    tks.set_rect(Rect::new(24.0, 144.0, 96.0, 24.0));
    d.add_child("form_main", tks).unwrap();

    let mut tmr = Node::new("tmr1", "Timer");
    tmr.set_name("tmr_tique");
    tmr.set_rect(Rect::new(400.0, 8.0, 28.0, 28.0));
    tmr.props
        .insert("interval".into(), crawrad_ir::Value::Int(100));
    tmr.events
        .insert("on_timer".into(), "tmr_tiqueTimer".into());
    d.add_child("form_main", tmr).unwrap();

    abas_aninhadas(&mut d, com_clique);

    d.validate().unwrap();
    d
}

/// **A `PageControl` inside a `TabSheet`**, with a button at the bottom.
///
/// The generator used to emit a guard for ONE page only — the closest one —,
/// so this button was guarded under `if self.pc_interno.active_page == 0` and
/// nothing else. With the OUTER tab closed, it kept being drawn and remained
/// clickable over whatever was on the other tab. The guard now composes the
/// condition of all ancestors, which is what `mod prova` tests verify — via
/// clicks, not emitted text.
///
/// Coordinates are relative to parent, as in the document. Added up, the button
/// lands at (40, 240) on the form; `mod prova` recalculates this.
fn abas_aninhadas(d: &mut Document, com_clique: bool) {
    let mut novo = |pai: &str, id: &str, classe: &str, nome: &str, texto: &str, r: Rect| {
        let mut n = Node::new(id, classe);
        n.set_name(nome);
        if !texto.is_empty() {
            n.set_text(texto);
        }
        n.set_rect(r);
        d.add_child(pai, n).unwrap();
    };

    novo(
        "form_main",
        "pc1",
        "PageControl",
        "pc_externo",
        "",
        Rect::new(24.0, 176.0, 400.0, 120.0),
    );
    novo(
        "pc1",
        "ts1",
        "TabSheet",
        "ts_externa_aberta",
        "Outer A",
        Rect::new(0.0, 24.0, 400.0, 96.0),
    );
    novo(
        "pc1",
        "ts2",
        "TabSheet",
        "ts_externa_fechada",
        "Outer B",
        Rect::new(0.0, 24.0, 400.0, 96.0),
    );
    novo(
        "ts1",
        "pc2",
        "PageControl",
        "pc_interno",
        "",
        Rect::new(8.0, 8.0, 300.0, 80.0),
    );
    novo(
        "pc2",
        "ts3",
        "TabSheet",
        "ts_interna_aberta",
        "Inner A",
        Rect::new(0.0, 24.0, 300.0, 56.0),
    );
    novo(
        "pc2",
        "ts4",
        "TabSheet",
        "ts_interna_fechada",
        "Inner B",
        Rect::new(0.0, 24.0, 300.0, 56.0),
    );

    let mut btn = Node::new("btn2", "Button");
    btn.set_name("btn_aninhado");
    btn.set_text("Deep");
    btn.set_rect(Rect::new(8.0, 8.0, 96.0, 24.0));
    if com_clique {
        btn.events
            .insert("on_click".into(), "btn_aninhadoClick".into());
    }
    d.add_child("ts3", btn).unwrap();
}

/// The user file: the handwritten handler body plus the proof.
///
/// `mod prova` drives egui headlessly and injects the click at the coordinates
/// saved by the **designer** — whether a click occurred is decided by the
/// generated file's `ui.put(...).clicked()`, not the test.
const ARQUIVO_DO_USUARIO: &str = r##"//! Arquivo do usuário — escrito à mão; o CrawRAD nunca o sobrescreve.
#![allow(non_snake_case)]

use crate::form_main_designer::FormMain;

impl FormMain {
    /// `btn_ok.on_click`
    pub fn btn_okClick(&mut self) {
        self.edt_nome.text = "clicou!".into();
    }

    /// `tmr_tique.on_timer`
    pub fn tmr_tiqueTimer(&mut self) {
        let n: u32 = self.edt_tiques.text.parse().unwrap_or(0);
        self.edt_tiques.text = (n + 1).to_string();
    }

    /// `btn_aninhado.on_click` — o botão do fundo das abas aninhadas.
    pub fn btn_aninhadoClick(&mut self) {
        self.edt_nome.text = "deep!".into();
    }

    /// `form_main.on_create` — CONCATENA de propósito.
    ///
    /// Somar em vez de atribuir é o que faz este handler medir a ORDEM: se o
    /// `new()` chamasse o `OnCreate` antes de os campos nascerem com os
    /// valores do designer, a palavra do designer não estaria aqui para ser
    /// concatenada — e ainda seria sobrescrita logo depois.
    pub fn form_mainCreate(&mut self) {
        self.edt_nome.text = format!("{} + created", self.edt_nome.text);
    }

    /// `form_main.on_close` — conta os pedidos de fechamento.
    ///
    /// Contar em vez de marcar é o que separa "avisou uma vez" de "avisa em
    /// todo quadro depois do pedido".
    pub fn form_mainClose(&mut self) {
        let n: u32 = self.edt_fechou.text.parse().unwrap_or(0);
        self.edt_fechou.text = (n + 1).to_string();
    }
}

#[cfg(test)]
mod prova {
    use super::*;
    use eframe::egui;

    fn clique(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// Um quadro de verdade do egui, sem janela. O formulário ocupa
    /// exatamente o tamanho que o designer deu a ele, então a origem cai em
    /// zero e o crescimento das âncoras também: a coordenada do código
    /// gerado é a coordenada da tela.
    fn quadro(ctx: &egui::Context, form: &mut FormMain, eventos: Vec<egui::Event>) {
        quadro_com(ctx, form, eventos, None, false);
    }

    /// Um quadro em que o gerenciador de janelas PEDIU o fechamento — o X, o
    /// Alt+F4, a barra de tarefas. É assim que o egui conta isso: um evento de
    /// viewport na entrada do quadro, e não um método a chamar.
    fn quadro_fechando(ctx: &egui::Context, form: &mut FormMain) {
        quadro_com(ctx, form, vec![], None, true);
    }

    /// O mesmo quadro, com o relógio posto na mão.
    ///
    /// `RawInput::time` é o relógio que o `ui.input(|i| i.time)` do código
    /// gerado lê. Pondo-o aqui, o teste do timer não espera segundo nenhum e
    /// não depende da velocidade da máquina — que é justamente o defeito que
    /// um timer contado em quadros teria.
    fn quadro_em(ctx: &egui::Context, form: &mut FormMain, tempo: f64) {
        quadro_com(ctx, form, vec![], Some(tempo), false);
    }

    fn quadro_com(
        ctx: &egui::Context,
        form: &mut FormMain,
        eventos: Vec<egui::Event>,
        tempo: Option<f64>,
        fechando: bool,
    ) {
        let mut viewport = egui::ViewportInfo::default();
        if fechando {
            viewport.events.push(egui::ViewportEvent::Close);
        }
        let entrada = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            viewports: std::iter::once((egui::ViewportId::ROOT, viewport)).collect(),
            events: eventos,
            time: tempo,
            ..Default::default()
        };
        ctx.begin_pass(entrada);
        let alvo = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(FormMain::SIZE[0], FormMain::SIZE[1]),
        );
        egui::Area::new(egui::Id::new("prova"))
            .fixed_pos(egui::Pos2::ZERO)
            .show(ctx, |ui| {
                let mut filho = ui.new_child(egui::UiBuilder::new().max_rect(alvo));
                form.show(&mut filho);
            });
        // Sem GPU ninguém consome o atlas de fontes; reconhecemos e
        // descartamos de propósito, senão o egui reclama que ninguém pintou.
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
    }

    fn clicar(form: &mut FormMain, alvo: egui::Pos2) {
        let ctx = egui::Context::default();
        quadro(&ctx, form, vec![]);
        quadro(&ctx, form, vec![egui::Event::PointerMoved(alvo)]);
        quadro(
            &ctx,
            form,
            vec![egui::Event::PointerMoved(alvo), clique(alvo, true)],
        );
        quadro(&ctx, form, vec![clique(alvo, false)]);
    }

    /// O gate: o clique no botão do designer executa o código do usuário.
    #[test]
    fn o_clique_no_botao_executa_o_handler_escrito_a_mao() {
        let mut form = FormMain::default();
        // btn_ok está em (24, 96) com 96x32 no designer; o centro é o alvo.
        clicar(&mut form, egui::pos2(24.0 + 48.0, 96.0 + 16.0));
        assert_eq!(form.edt_nome.text, "clicou!");
    }

    /// Sem clique nenhum, nada acontece — senão o teste de cima passaria
    /// mesmo com o campo nascendo preenchido.
    #[test]
    fn sem_clique_o_handler_nao_roda() {
        let ctx = egui::Context::default();
        let mut form = FormMain::default();
        quadro(&ctx, &mut form, vec![]);
        quadro(&ctx, &mut form, vec![]);
        assert_ne!(form.edt_nome.text, "clicou!");
    }

    /// Clicar fora do botão também não roda: o que decide é a geometria que
    /// o designer gravou, não um clique qualquer.
    #[test]
    fn clique_fora_do_botao_nao_roda() {
        let mut form = FormMain::default();
        clicar(&mut form, egui::pos2(500.0, 300.0));
        assert_ne!(form.edt_nome.text, "clicou!");
    }

    /// O timer do designer chama o handler escrito à mão, uma vez por
    /// intervalo. É o gate do clique aplicado ao que não se clica.
    #[test]
    fn o_timer_chama_o_handler_uma_vez_por_intervalo() {
        let ctx = egui::Context::default();
        let mut form = FormMain::default();

        // O primeiro quadro só começa a contar — o `SetTimer` do Windows
        // agenda o primeiro tique para daqui a `Interval`, e não para já.
        quadro_em(&ctx, &mut form, 0.0);
        assert_eq!(form.edt_tiques.text, "");
        quadro_em(&ctx, &mut form, 0.10);
        assert_eq!(form.edt_tiques.text, "1");
        quadro_em(&ctx, &mut form, 0.15);
        assert_eq!(form.edt_tiques.text, "1", "before the deadline, nothing");
        quadro_em(&ctx, &mut form, 0.20);
        assert_eq!(form.edt_tiques.text, "2");
    }

    /// **A regra do acúmulo, provada no aplicativo de verdade.**
    ///
    /// Cinco segundos sem desenhar, com intervalo de 100 ms: cinquenta tiques
    /// perdidos. O handler roda UMA vez — é o que o `WM_TIMER` do Windows faz,
    /// e por isso é o que o `TTimer` da IDE RAD clássica faz.
    #[test]
    fn o_tique_atrasado_e_descartado_e_nao_enfileirado() {
        let ctx = egui::Context::default();
        let mut form = FormMain::default();
        quadro_em(&ctx, &mut form, 0.0);
        quadro_em(&ctx, &mut form, 5.0);
        assert_eq!(form.edt_tiques.text, "1", "fifty lost become one");
        quadro_em(&ctx, &mut form, 5.05);
        assert_eq!(
            form.edt_tiques.text, "1",
            "and the next frame pays no late tick"
        );
    }

    /// O botão do fundo das abas ANINHADAS. A coordenada é a soma do
    /// documento: pc_externo (24,176) + ts_externa (0,24) + pc_interno (8,8)
    /// + ts_interna (0,24) + o botão (8,8) = (40,240), 96x24.
    const BOTAO_ANINHADO: egui::Pos2 = egui::pos2(40.0 + 48.0, 240.0 + 12.0);

    /// Com as DUAS abas abertas, o clique no botão do fundo roda o handler.
    #[test]
    fn o_clique_no_botao_das_abas_aninhadas_executa_o_handler() {
        let mut form = FormMain::default();
        clicar(&mut form, BOTAO_ANINHADO);
        assert_eq!(form.edt_nome.text, "deep!");
    }

    /// **A guarda que não compunha.** A aba EXTERNA fechada tem de esconder o
    /// botão que mora no `PageControl` de dentro dela — e esconder quer dizer
    /// que o pixel não é pintado e o clique não encontra nada. O gerador
    /// escrevia só `if self.pc_interno.active_page == 0`, e este clique
    /// disparava o handler com a aba externa fechada.
    #[test]
    fn com_a_aba_externa_fechada_o_botao_do_fundo_nao_recebe_clique() {
        let mut form = FormMain::default();
        form.pc_externo.active_page = 1;
        clicar(&mut form, BOTAO_ANINHADO);
        assert_ne!(
            form.edt_nome.text, "deep!",
            "the outer tab is closed: nothing inside should respond"
        );
    }

    /// E a aba interna fechada continua escondendo, com a externa aberta.
    /// Este é o caso de um nível só — o que já funcionava —, e ele fica aqui
    /// para o teste de cima não passar por um conserto que apagou a guarda.
    #[test]
    fn com_a_aba_interna_fechada_o_botao_do_fundo_nao_recebe_clique() {
        let mut form = FormMain::default();
        form.pc_interno.active_page = 1;
        clicar(&mut form, BOTAO_ANINHADO);
        assert_ne!(form.edt_nome.text, "deep!");
    }

    /// **O `OnCreate` roda, roda uma vez, e roda DEPOIS do designer.**
    ///
    /// Uma asserção prende as três coisas: o handler concatena, então
    /// "designer + created" só sai se os valores do desenho estavam no lugar
    /// antes de ele rodar, e se ele rodou exatamente uma vez. Ordem trocada
    /// daria "created"; sem chamada nenhuma, "designer"; duas chamadas,
    /// "designer + created + created".
    #[test]
    fn o_on_create_roda_uma_vez_depois_dos_valores_do_designer() {
        let form = FormMain::new();
        assert_eq!(form.edt_nome.text, "designer + created");
    }

    /// E o `default()` continua sendo só o desenho: é ele que o `new()`
    /// monta antes de chamar o handler.
    #[test]
    fn o_default_e_o_desenho_e_mais_nada() {
        let form = FormMain::default();
        assert_eq!(form.edt_nome.text, "designer");
    }

    /// Desenhar quadro nenhum cria formulário: o `OnCreate` não é coisa do
    /// `show()`, e por isso quadro nenhum o chama de novo.
    #[test]
    fn desenhar_nao_chama_o_on_create_de_novo() {
        let ctx = egui::Context::default();
        let mut form = FormMain::new();
        quadro(&ctx, &mut form, vec![]);
        quadro(&ctx, &mut form, vec![]);
        assert_eq!(form.edt_nome.text, "designer + created");
    }

    /// **O pedido de fechamento chama o `OnClose`, uma vez.**
    #[test]
    fn o_pedido_de_fechamento_chama_o_on_close_uma_vez() {
        let ctx = egui::Context::default();
        let mut form = FormMain::default();
        quadro(&ctx, &mut form, vec![]);
        assert_eq!(form.edt_fechou.text, "", "a normal frame does not close anything");
        quadro_fechando(&ctx, &mut form);
        assert_eq!(form.edt_fechou.text, "1");
        quadro(&ctx, &mut form, vec![]);
        assert_eq!(
            form.edt_fechou.text, "1",
            "the request belongs to the frame it arrived in, not a lasting state"
        );
    }

    /// Timer desligado não chama nada. Sem isto, o teste de cima passaria
    /// mesmo com um timer que dispara em todo quadro.
    #[test]
    fn timer_desligado_nao_chama_o_handler() {
        let ctx = egui::Context::default();
        let mut form = FormMain::default();
        form.tmr_tique.enabled = false;
        for i in 0..10 {
            quadro_em(&ctx, &mut form, i as f64);
        }
        assert_eq!(form.edt_tiques.text, "");
    }
}
"##;

fn gerar_em(destino: &Path, com_clique: bool) {
    let arqs = crawrad_codegen::GeradorRxp
        .gerar(
            &documento(com_clique),
            "gate_fase3",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generate the project");
    crawrad_codegen::escrever(destino, &arqs).expect("write the project");
}

/// A `cargo` invocation inside the generated project — or inside the group,
/// when `cargo new` is being called.
///
/// **The project IS a member of a group**, and that is what this gate proves.
/// It was previously stated here that it had an empty `[workspace]` and was a
/// member of nothing: that was true under the old flow, where the generator
/// wrote `Cargo.toml`. Since ADR 0018 the manifest comes from `cargo new` and
/// the generator writes none (`crates/crawrad-codegen/src/rxp.rs`, the plan is
/// only `src/*.rs`); the package is resolved by [`RAIZ_DO_GRUPO`], which lists
/// it in `members`.
///
/// What remains valid is the reason for `env_remove`: environment variables
/// from the OUTER cargo — ours, running this test — would make the inner cargo
/// compile in the wrong `target/` and with different flags.
fn cargo(destino: &Path, args: &[&str]) -> (bool, String) {
    let saida = Command::new(env!("CARGO"))
        .args(args)
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .current_dir(destino)
        .output()
        .expect("call cargo inside the generated project");
    let texto = format!(
        "{}{}",
        String::from_utf8_lossy(&saida.stdout),
        String::from_utf8_lossy(&saida.stderr)
    );
    (saida.status.success(), texto)
}

/// Runs `cargo test` inside the generated project.
fn testar(destino: &Path) -> (bool, String) {
    cargo(destino, &["test"])
}

/// The "user repository": an already existing Cargo workspace, with what is
/// inherited from it.
///
/// **This is the difficult scenario of ADR 0018, and it is simulated intentionally.**
/// When the folder is inside a workspace, `cargo init` (measured on 1.95) does two
/// things: registers the new package in `workspace.members` **in the manifest above**
/// and writes the lower manifest inheriting from it. Letting that hit our `Cargo.toml`
/// would pollute the repository on each run; with a mock workspace in the way,
/// Cargo stops at it — searching for the **closest** ancestor — and clutter stays
/// within `target/`, which is disposable.
///
/// And the scenario remains realistic: it is exactly what happens when creating a
/// test app inside an existing repository.
const REPOSITORIO_DE_MENTIRA: &str = "# Um workspace que já existe, para o `cargo` do gate cair dentro dele.\n\
     [workspace]\n\
     members = []\n\
     resolver = \"2\"\n\
     \n\
     [workspace.package]\n\
     edition = \"2024\"\n\
     \n\
     [workspace.lints.clippy]\n\
     all = \"warn\"\n";

/// The GROUP root, written before `cargo new` — as the IDE does.
///
/// The structure is what matters, not the exact bytes: `[workspace]` without
/// `[workspace.package]`, with the member already listed. That structure makes
/// `cargo new` stop here instead of climbing up to the outer repository, and
/// is what the generated project must be able to open.
const RAIZ_DO_GRUPO: &str = r#"[workspace]
members = ["app"]
resolver = "2"
default-members = ["app"]
"#;

/// Is the draft **intact**, or is only a fragment left?
///
/// ## The bug this function exists to prevent from returning
///
/// `if destino.join("Cargo.toml").exists() { return; }` used to live here,
/// and that line failed CI seven times in a row — always in 0.03 s, before
/// asserting anything:
///
/// ```text
/// o `cargo new` falhou:
/// error: destination `D:\a\CrawRAD\CrawRAD\target\gate-fase3\app` already exists
/// ```
///
/// The draft lives in `target/`, and `target/` is what `Swatinem/rust-cache`
/// restores at the start of each run. **What it restores is not what it saved:**
/// the action prunes the tree before saving, leaving the `app/` directory in
/// place without `Cargo.toml` inside it. The single-file guard saw the missing
/// manifest and concluded "there is no project"; `cargo new` saw the existing
/// folder and concluded "there already is". Both were right, and the gate died
/// in the middle.
///
/// **Reproduced on this machine's disk:** deleting `app/Cargo.toml` while keeping
/// `app/` causes the old gate to fail with the message above, same line, same 0.03 s.
/// Locally it never failed because nobody prunes anyone's `target/` — it is either
/// intact or absent.
///
/// ## The rule that remained
///
/// **One file does not prove a project.** The guard checks the entire set, and
/// the set is *everything [`preparar_projeto`] writes* — the four pieces, in the
/// order they are created:
///
/// 1. the outer repository manifest (`target/gate-fase3/Cargo.toml`);
/// 2. the group root (`target/gate-fase3/grupo/Cargo.toml`);
/// 3. the member's `src/`, created by `cargo new`;
/// 4. `eframe` in the member manifest, added by `cargo add`.
///
/// `eframe` is the last piece to arrive, so finding it indicates that setup
/// finished completely. If any piece is missing, the draft is wiped and recreated
/// from scratch. A half-baked state cannot be reused.
///
/// **Piece 1 was missing in the first version of this guard**, and the omission
/// was silently broken in the worst way: deleting only `target/gate-fase3/Cargo.toml`,
/// the gate passed green while the file remained absent — meaning the three-level
/// scenario of ADR 0018, the reason the mock repository exists, **silently stopped
/// happening**. A guard that does not check everything it writes stops guarding
/// one piece at a time.
///
/// ## Both checks are the SAME check
///
/// The two workspace manifests are checked by CONTENT, and by EQUALITY
/// ([`manifesto_intocado`]), not by `.exists()`. There used to be an asymmetric
/// guard here — `.exists()` on the group root, `contains("eframe")` on the member —,
/// and asymmetry has a price. **Measured on this machine**, with `grupo/Cargo.toml`
/// present with **zero bytes**:
///
/// ```text
/// error: failed to parse manifest at `...\grupo\Cargo.toml`
/// Caused by:
///   manifest is missing either a `[package]` or a `[workspace]`
/// ```
///
/// Every cargo invocation inside the member halts on this — `metadata`, `add`, `test` —,
/// because the member cannot open without the root resolving it. And `.exists()`
/// reported "intact", so **nothing rewrote the file and every subsequent run died
/// identically**: it is the same dead-gate-that-never-heals this function came to
/// eliminate, one piece over. (Curiosity measured alongside: `cargo new` itself
/// survives an empty file — it only WARNS `invalid workspace configuration` and
/// creates the folder anyway. What dies is the next step.)
///
/// Cache pruning deletes rather than truncating, so this state does not come
/// from it; it comes from Ctrl-C between writes — the same scenario the list below
/// already cited as justification.
///
/// ## Why the draft remains inside `target/`
///
/// That was the alternative considered: moving it out of `target/` would hide it
/// from the cache and make the defect vanish. It stays, for three reasons plus one:
///
/// - `target/` is the only folder the repository already ignores and `cargo clean`
///   already wipes away. A draft in `%TEMP%` would survive `cargo clean` and
///   nobody would know it is there.
/// - the group's shared `target/` (`target/gate-fase3/grupo/target/`) is what
///   makes the second run take seconds instead of two minutes, and it is not
///   inside `app/` — deleting the draft **does not** cold-start the build.
///   Measured: `app/` has only `Cargo.toml` and `src/`, and the group's `target/`
///   remains there after `remove_dir_all`. (This path used to say
///   `target/gate-fase3/target/`, which was true while the gate had two levels;
///   the third level entered in the same commit that wrote this line and left it
///   pointing to a non-existent folder.)
/// - running away from the cache would fix the symptom. The fragile guard would
///   remain fragile, and the next thing touching `target/` — an intermediate
///   `cargo clean`, a Ctrl-C interrupt between `cargo new` and `cargo add` —
///   would hit the same dead gate for the same reason.
///
/// And the reason worth all others: a gate that only works because nobody touches
/// its draft is a gate that depends on luck.
fn rascunho_intacto(repositorio: &Path, grupo: &Path, destino: &Path) -> bool {
    if !manifesto_intocado(repositorio, REPOSITORIO_DE_MENTIRA)
        || !manifesto_intocado(grupo, RAIZ_DO_GRUPO)
    {
        return false;
    }
    if !destino.join("src").is_dir() {
        return false;
    }
    std::fs::read_to_string(destino.join("Cargo.toml"))
        .is_ok_and(|m| m.contains("eframe") && m.contains("crawrad-runtime"))
}

/// Scorch earth before `cargo new`: wipes the partial draft and rewrites both
/// workspace manifests.
///
/// Separated from [`preparar_projeto`] because **this half does not call cargo**,
/// and can therefore be tested in seconds rather than in a multi-minute gate.
/// See [`o_terreno_refeito_nao_guarda_pedaco_do_rascunho_velho`].
///
/// `remove_dir_all` is the guard's counterpart, not a detail: without it,
/// [`rascunho_intacto`] can report "partial" all it wants while `cargo new`
/// keeps responding `destination ... already exists` — exactly how CI died
/// seven times. A guard that detects without fixing fixes nothing.
///
/// Order matters: the group root must be on disk BEFORE `cargo new`. Without it,
/// cargo climbs up to the mock repository, registers the package in its `members`,
/// and writes the member manifest inheriting from it — and when the group root
/// appeared next, the newly created project would fail to open.
fn refazer_o_terreno(repositorio: &Path, grupo: &Path, destino: &Path) {
    if destino.exists() {
        std::fs::remove_dir_all(destino).expect("delete the half draft");
    }
    std::fs::create_dir_all(grupo).expect("create the group root");
    std::fs::write(repositorio.join("Cargo.toml"), REPOSITORIO_DE_MENTIRA)
        .expect("the mock workspace");
    std::fs::write(grupo.join("Cargo.toml"), RAIZ_DO_GRUPO).expect("the group root");
}

/// What the IDE does before generating: group root, `cargo new`, `cargo add`.
///
/// **The order is that of the IDE, which the gate must reproduce.** The old
/// flow used to live here — `cargo init` at destination followed by an
/// `isolar_workspace` patching up the inheritance cargo just wrote. That flow
/// was removed from the IDE, but the gate kept testing it: the most valuable
/// test in the repository stayed green measuring a path the product no longer
/// takes. That is rule 8 one level above normal — not a test that doesn't touch
/// code, but a test touching the wrong code.
///
/// Setup is simulated only in the outer `Cargo.toml`; the rest is real `cargo`,
/// which decides whether having the group root in the right place suffices for
/// the project to open.
///
/// Only sets up what does not exist yet: `cargo new` rejects directories that
/// already have a package, and the gate generates multiple times in the same
/// destination intentionally to keep `target/` warm. When the draft is partial,
/// it is **wiped** and rebuilt — see [`rascunho_intacto`].
fn preparar_projeto(repositorio: &Path, destino: &Path) {
    let grupo = destino
        .parent()
        .expect("the destination is a member of a group");
    if rascunho_intacto(repositorio, grupo, destino) {
        return;
    }
    refazer_o_terreno(repositorio, grupo, destino);

    // `--vcs none`: a `.git` in here would be a repo inside our own repo.
    let nome = destino
        .file_name()
        .and_then(|n| n.to_str())
        .expect("the name");
    let (ok, texto) = cargo(
        grupo,
        &[
            "new",
            "--bin",
            "--vcs",
            "none",
            "--name",
            "gate_fase3",
            nome,
        ],
    );
    assert!(ok, "`cargo new` failed:\n{texto}");

    // The member must OPEN on its own, which was the missing check here.
    // The defect it catches doesn't appear when compiling our code: it appears
    // the first time someone opens the project the IDE claimed to have created.
    let (ok, texto) = cargo(destino, &["metadata", "--no-deps", "--format-version", "1"]);
    assert!(
        ok,
        "the newly created project does not open — the group root did not resolve it:\n{texto}"
    );

    // `cargo add` hits the index. Offline first: on a machine that has already
    // compiled this workspace, the `eframe 0.36` index is cached and the gate
    // runs without network. Without cache, it falls back to downloading.
    let (ok, offline) = cargo(destino, &["add", "eframe@0.36", "egui@0.36", "--offline"]);
    if !ok {
        let (ok, online) = cargo(destino, &["add", "eframe@0.36", "egui@0.36"]);
        assert!(
            ok,
            "`cargo add` failed offline AND online — without the crates.io index \
             this gate cannot set up the project.\n--- offline ---\n{offline}\
             \n--- online ---\n{online}"
        );
    }

    // Same dependency the IDE writes for an egui form (`dependencias_do_projeto`):
    // the generated `main` reports a startup failure through it.
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("../crawrad-runtime");
    let runtime = runtime.to_str().expect("the runtime path");
    let (ok, texto) = cargo(destino, &["add", "--path", runtime, "--offline"]);
    assert!(ok, "the `cargo add` of crawrad-runtime failed:\n{texto}");
}

#[test]
#[ignore = "builds a real project; run with --ignored"]
fn o_projeto_gerado_compila_e_o_clique_executa_o_handler() {
    // Three levels, not two: the user repository, the GROUP inside it, and
    // the member. Previously `repositorio == grupo`, so both manifests hit
    // the SAME file — `REPOSITORIO_DE_MENTIRA` was written and overwritten
    // by the group root one line later. The difficult scenario of ADR 0018
    // was documented in twenty lines and never occurred: what stopped `cargo new`
    // was always the group root, and the outer repository was never an ancestor
    // of anything.
    let repositorio: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-fase3");
    let destino = repositorio.join("grupo").join("app");

    // 0. The IDE creates the project via cargo: group root, `new`, `add`.
    preparar_projeto(&repositorio, &destino);

    // 1. The designer generates the project. The `main.rs` from `cargo init`
    //    is overwritten here — it is `Dono::Gerador`, which is how codegen
    //    behaves over a folder that is already a crate.
    gerar_em(&destino, true);

    // 2. The user writes the handler body. That the generator does not
    //    overwrite this file is already proven in unit tests; here it just
    //    needs to exist with real code inside.
    std::fs::write(destino.join("src/form_main.rs"), ARQUIVO_DO_USUARIO)
        .expect("write the user file");

    // 3. Compiles and runs.
    let (ok, texto) = testar(&destino);
    assert!(
        ok,
        "the generated project should compile and pass:\n{texto}"
    );
    assert!(
        texto.contains("13 passed"),
        "the thirteen tests should run:\n{texto}"
    );

    // 4. Proof that the proof holds: removing the event binding in the
    //    document causes the same click to stop executing the handler. Without
    //    this, step 3 could be passing by accident.
    gerar_em(&destino, false);
    let (ok, texto) = testar(&destino);
    assert!(
        !ok,
        "without on_click in the document the click could not execute anything:\n{texto}"
    );

    // 5. Leaves the directory green, for anyone inspecting later.
    gerar_em(&destino, true);
    let (ok, _) = testar(&destino);
    assert!(ok, "rewiring the event, it should pass again");
}

// ===========================================================================
// THE GATE'S GATE
//
// The gate above is `#[ignore]` and takes minutes. The two tests down here
// run in milliseconds in the normal suite, and exist for a reason measured
// and phrased by this fix's reviewer: **the fix had no gate**. Undoing the
// entire guard — returning to `if destino.join("Cargo.toml").exists() { return; }`
// and removing `remove_dir_all` —, with an intact disk both gates stayed GREEN.
// Nothing failed. A fix that is only a fix as long as nobody reverts it is a
// comment, not code.
//
// They do not call cargo and do not touch `target/`: they construct a mock draft
// in a temporary directory and ask the guard. The guard is what is under test,
// and the guard is a disk function.
// ===========================================================================

/// An INTACT draft on disk, just as [`preparar_projeto`] leaves it: the two
/// workspace manifests, the member's `src/`, and `eframe` added by `cargo add`.
/// Returns `(repositorio, grupo, destino)`.
///
/// `grupo/target/` is included because it makes the second run cheap, and
/// because the second test verifies that it survives.
fn rascunho_de_mentira(raiz: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let repositorio = raiz.to_path_buf();
    let grupo = repositorio.join("grupo");
    let destino = grupo.join("app");
    std::fs::create_dir_all(destino.join("src")).expect("the member's `src/`");
    std::fs::create_dir_all(grupo.join("target")).expect("the group's `target/`");
    std::fs::write(repositorio.join("Cargo.toml"), REPOSITORIO_DE_MENTIRA).unwrap();
    std::fs::write(grupo.join("Cargo.toml"), RAIZ_DO_GRUPO).unwrap();
    std::fs::write(
        destino.join("Cargo.toml"),
        "[package]\nname = \"gate_fase3\"\nedition = \"2024\"\n\n\
         [dependencies]\neframe = \"0.36\"\negui = \"0.36\"\n\
         crawrad-runtime = { path = \"../crawrad-runtime\" }\n",
    )
    .unwrap();
    std::fs::write(destino.join("src/main.rs"), "fn main() {}\n").unwrap();
    (repositorio, grupo, destino)
}

/// **Every piece that [`preparar_projeto`] writes, if missing or partial,
/// rejects the draft.**
///
/// The table is the enumeration of states in which cache pruning — or a Ctrl-C
/// mid-setup — can leave the disk. The criterion for each row is the same:
/// **if cargo could not continue from there, the guard must report that it is
/// not intact**, because the price of error is not slowness, but the gate dying
/// before the first assertion and remaining dead in every subsequent run.
///
/// ## Mutation testing, state by state
///
/// Each line here exercises a different part of the guard; none is here for
/// pretty symmetry. Reverting the guard to what it was before this fix
/// (`.exists()` on the group root and `eframe` in the member), these fail:
///
/// - `without repository manifest` and `empty repository manifest` — the
///   old guard NEVER checked for it. It is the fourth artifact, from the
///   three-level scenario of ADR 0018, and without it `cargo new` climbs back
///   up to the real repository.
/// - `empty group root` — the old `.exists()` returned true for a zero-byte
///   file, and cargo then died with `failed to parse manifest`.
/// - `group root with OTHER content` and `repository manifest with OTHER content` —
///   the `contains("[workspace]")` that came after `.exists()`
///   also returned true because the section is there. Swapping `==` in
///   [`manifesto_intocado`] back to `contains` makes these TWO lines, and only
///   these two, return `true`.
///
/// **The second of those two was added on 07/09/2026, and it was missing.** The
/// review measured the tenth state: weakening **only** the repository
/// check — the first half of [`rascunho_intacto`] — left the whole suite green.
/// The two lines that already checked the repository (`sem o manifesto` and
/// `manifesto vazio`) don't catch this: unreadable files and empty files also
/// fail `contains`. Only a manifest that OPENS and is not ours separates the
/// two questions — and [`manifesto_intocado`]'s docs always stated the state
/// comes from editing EITHER of the two constants, while the table listed only
/// one. The symmetric line is the guard catching up with its own documentation.
///
/// And the other lines exercise the NEW guard: removing any of the four
/// checks makes the corresponding line return `true`.
#[test]
fn o_guarda_reprova_todo_rascunho_pela_metade() {
    // (the state, what pruning did, is the draft still usable?)
    type Poda = fn(&Path, &Path, &Path);
    let casos: &[(&str, Poda, bool)] = &[
        ("intact", |_, _, _| {}, true),
        (
            "without repository manifest",
            |repo, _, _| std::fs::remove_file(repo.join("Cargo.toml")).unwrap(),
            false,
        ),
        (
            "empty repository manifest",
            |repo, _, _| std::fs::write(repo.join("Cargo.toml"), "").unwrap(),
            false,
        ),
        // The TENTH state, and the symmetric line to the one below. A `[workspace]`
        // that OPENS in place of the mock repository, but is not ours: without
        // `[workspace.package]` that the member inherits, `cargo new` writes a
        // manifest with `edition.workspace = true` that fails to resolve, and the gate
        // dies on every subsequent run without self-healing. This line is why the
        // repository check must also test for equality.
        (
            "repository manifest with OTHER content",
            |repo, _, _| {
                std::fs::write(repo.join("Cargo.toml"), "[workspace]\nmembers = []\n").unwrap()
            },
            false,
        ),
        (
            "without group root",
            |_, grupo, _| std::fs::remove_file(grupo.join("Cargo.toml")).unwrap(),
            false,
        ),
        (
            "empty group root",
            |_, grupo, _| std::fs::write(grupo.join("Cargo.toml"), "").unwrap(),
            false,
        ),
        // The ninth state. It and the tenth above are the two that do not come
        // from pruning or Ctrl-C: they come from someone editing either of the two
        // constants and leaving the previous version's draft on disk. A `[workspace]`
        // that OPENS and is not ours — without `members`, `cargo new` registers
        // the member again and the gate dies.
        (
            "group root with OTHER content",
            |_, grupo, _| {
                std::fs::write(grupo.join("Cargo.toml"), "[workspace]\nmembers = []\n").unwrap()
            },
            false,
        ),
        (
            "without member manifest",
            |_, _, dest| std::fs::remove_file(dest.join("Cargo.toml")).unwrap(),
            false,
        ),
        (
            "member without `cargo add`",
            |_, _, dest| {
                std::fs::write(
                    dest.join("Cargo.toml"),
                    "[package]\nname = \"gate_fase3\"\n\n[dependencies]\n",
                )
                .unwrap()
            },
            false,
        ),
        (
            "without member `src/`",
            |_, _, dest| std::fs::remove_dir_all(dest.join("src")).unwrap(),
            false,
        ),
    ];

    // Runs the ENTIRE table before complaining, intentionally: whoever wants
    // to check mutation testing needs to see ALL states mutation produces,
    // not just the first.
    let base = std::env::temp_dir().join("crawrad_gate_fase3_guarda");
    let _ = std::fs::remove_dir_all(&base);
    let mut errados: Vec<&str> = Vec::new();
    for (i, (estado, podar, serve)) in casos.iter().enumerate() {
        let raiz = base.join(format!("caso{i}"));
        let (repositorio, grupo, destino) = rascunho_de_mentira(&raiz);
        podar(&repositorio, &grupo, &destino);
        if rascunho_intacto(&repositorio, &grupo, &destino) != *serve {
            errados.push(estado);
        }
    }
    assert!(
        errados.is_empty(),
        "the guard answered the opposite of what these disks deserve: {errados:?}"
    );
    let _ = std::fs::remove_dir_all(&base);
}

/// **The guard detects, and the rebuilt terrain fixes.** The two halves form a
/// single guarantee, and this is what `cargo new` enforces.
///
/// Mutation proof: removing `remove_dir_all` from [`refazer_o_terreno`], this
/// test fails on the first assertion — exactly the state where `cargo new`
/// returns `destination ... already exists` and the gate dies in 0.03 s, which
/// is how CI died seven times.
///
/// The second assertion is the flip side of the coin: wiping the draft **cannot**
/// take down the group's `target/`, which is what makes the subsequent run
/// take seconds. It is a sibling of `app/`, not a child.
#[test]
fn o_terreno_refeito_nao_guarda_pedaco_do_rascunho_velho() {
    let raiz = std::env::temp_dir().join("crawrad_gate_fase3_terreno");
    let _ = std::fs::remove_dir_all(&raiz);
    let (repositorio, grupo, destino) = rascunho_de_mentira(&raiz);

    // The pruning that broke CI, plus an intruder file to prove that the
    // directory vanishes completely, rather than piece by piece.
    std::fs::remove_file(destino.join("Cargo.toml")).unwrap();
    std::fs::write(
        destino.join("src/sobra.rs"),
        "// leftover from previous run",
    )
    .unwrap();
    assert!(!rascunho_intacto(&repositorio, &grupo, &destino));

    refazer_o_terreno(&repositorio, &grupo, &destino);

    assert!(
        !destino.exists(),
        "`cargo new` rejects existing destination: member folder should have disappeared"
    );
    assert!(
        grupo.join("target").is_dir(),
        "deleting the draft must not invalidate the group build"
    );
    assert_eq!(
        std::fs::read_to_string(repositorio.join("Cargo.toml")).unwrap(),
        REPOSITORIO_DE_MENTIRA,
        "the outer repository is what enables the three-level scenario"
    );
    assert_eq!(
        std::fs::read_to_string(grupo.join("Cargo.toml")).unwrap(),
        RAIZ_DO_GRUPO,
        "the group root must be on disk BEFORE `cargo new`"
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

/// One form of a three-level chain: `name` inherits `inherits`, and
/// overrides the click of A's `btn_ok` with `handler` when there is one.
fn chain_form(name: &str, inherits: Option<&str>, handler: Option<&str>) -> Document {
    let mut doc = Document::new(name, "Form");
    {
        let root = doc.node_mut(name).unwrap();
        root.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        root.set_name(name);
    }
    doc.set_inherits(inherits);
    if inherits.is_none() || handler.is_some() {
        let mut btn = Node::new("btn_ok", "Button");
        btn.set_name("btn_ok");
        btn.set_rect(Rect::new(20.0, 20.0, 120.0, 32.0));
        if let Some(h) = handler {
            btn.events.insert("on_click".into(), h.into());
        }
        doc.add_child(name, btn).unwrap();
    }
    doc
}

/// Writes one generated form, keeping the shared `main.rs` out.
fn write_form(destino: &Path, files: &[crawrad_codegen::Arquivo]) {
    for a in files {
        if a.caminho != Path::new("src/main.rs") {
            std::fs::write(destino.join(&a.caminho), &a.conteudo).unwrap();
        }
    }
}

/// Appends a line to the end of a handler's body in a form's user file.
fn append_to_handler(destino: &Path, form: &str, handler: &str, line: &str) {
    let path = destino.join(format!("src/{form}.rs"));
    let text = std::fs::read_to_string(&path).unwrap();
    let start = text
        .find(&format!("pub fn {handler}(&mut self) {{"))
        .unwrap_or_else(|| panic!("{handler} in {form}.rs:\n{text}"));
    let end = start + text[start..].find("\n    }").unwrap();
    let text = format!("{}\n        {line}{}", &text[..end], &text[end..]);
    std::fs::write(&path, text).unwrap();
}

/// **Three levels**: C over B over A. The most derived override runs,
/// and each `self.base.<handler>()` reaches the level above, down to A.
#[test]
#[ignore = "compila um projeto de verdade; rode com --ignored"]
fn a_three_level_override_reaches_the_first_base() {
    let repositorio: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-three-levels");
    let destino = repositorio.join("grupo").join("app");
    preparar_projeto(&repositorio, &destino);

    let reg = crawrad_registry::Registry::builtin();
    let generator = crawrad_codegen::targets::egui::GeradorRxp;
    let a = chain_form("form_a", None, Some("btn_ok_click"));
    let b = chain_form("form_b", Some("form_a"), Some("btn_b_click"));
    let c = chain_form("form_c", Some("form_b"), Some("btn_c_click"));

    let run = |c: &Document, c_handler: Option<&str>| {
        let gen_a = generator.generate_with_bases(&a, "app", &reg, &[], true);
        write_form(&destino, &gen_a.expect("A"));
        let gen_b = generator.generate_with_bases(&b, "app", &reg, std::slice::from_ref(&a), true);
        write_form(&destino, &gen_b.expect("B"));
        let bases = [b.clone(), a.clone()];
        let gen_c = generator.generate_with_bases(c, "app", &reg, &bases, false);
        write_form(&destino, &gen_c.expect("C"));
        append_to_handler(&destino, "form_a", "btn_ok_click", "println!(\"a\");");
        append_to_handler(&destino, "form_b", "btn_b_click", "println!(\"b\");");
        if let Some(h) = c_handler {
            append_to_handler(&destino, "form_c", h, "println!(\"c\");");
        }
        std::fs::write(destino.join("src/main.rs"), THREE_LEVEL_MAIN).unwrap();
        let (ok, out) = cargo(&destino, &["run", "-q"]);
        assert!(ok, "the three-level project must run:\n{out}");
        out.lines()
            .map(str::trim)
            .filter(|l| l.len() == 1)
            .collect::<Vec<_>>()
            .join("")
    };

    assert_eq!(
        run(&c, Some("btn_c_click")),
        "abc",
        "C overrides and calls B, which calls A"
    );
    let c_plain = chain_form("form_c", Some("form_b"), None);
    assert_eq!(
        run(&c_plain, None),
        "ab",
        "C inherits B's override untouched"
    );
}

const THREE_LEVEL_MAIN: &str = "\
#![allow(non_snake_case)]
use eframe::egui;

mod form_a;
#[path = \"form_a.designer.rs\"]
pub mod form_a_designer;
mod form_b;
#[path = \"form_b.designer.rs\"]
pub mod form_b_designer;
mod form_c;
#[path = \"form_c.designer.rs\"]
pub mod form_c_designer;

fn main() {
    let ctx = egui::Context::default();
    let mut form = form_c_designer::FormC::default();
    ctx.run_ui(egui::RawInput::default(), |ui| form.show(ui)).textures_delta.clear();
    let center = egui::pos2(80.0, 36.0);
    for pressed in [true, false] {
        let mut raw_input = egui::RawInput::default();
        raw_input.events.push(egui::Event::PointerMoved(center));
        raw_input.events.push(egui::Event::PointerButton {
            pos: center,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        });
        ctx.run_ui(raw_input, |ui| form.show(ui)).textures_delta.clear();
    }
}
";

#[test]
#[ignore = "builds a real project; run with --ignored"]
fn o_formulario_derivado_com_override_de_evento_compila() {
    let repositorio: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-fase3-override");
    let destino = repositorio.join("grupo").join("app");

    preparar_projeto(&repositorio, &destino);

    let reg = crawrad_registry::Registry::builtin();
    let gerador = crawrad_codegen::targets::egui::GeradorRxp;

    let mut base_doc = Document::new("FormBase", "Form");
    {
        let root = base_doc.node_mut("FormBase").unwrap();
        root.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        root.set_name("FormBase");
    }
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.set_rect(Rect::new(20.0, 20.0, 120.0, 32.0));
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("FormBase", btn).unwrap();

    let arqs_base = gerador
        .generate_with_base(&base_doc, "app", &reg, None, true)
        .expect("generate base");
    for a in &arqs_base {
        if a.caminho != Path::new("src/main.rs") {
            std::fs::write(destino.join(&a.caminho), &a.conteudo).unwrap();
        }
    }

    let base_user_path = destino.join("src/FormBase.rs");
    let base_user_content = std::fs::read_to_string(&base_user_path).unwrap();
    let base_with_handler = base_user_content.replace(
        "pub fn btn_ok_click(&mut self) {\n        \n    }",
        "pub fn btn_ok_click(&mut self) {\n        println!(\"base click\");\n    }",
    );
    std::fs::write(&base_user_path, base_with_handler).unwrap();

    let mut derived_doc = Document::new("FormDerived", "Form");
    {
        let root = derived_doc.node_mut("FormDerived").unwrap();
        root.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        root.set_name("FormDerived");
    }
    derived_doc.set_inherits(Some("FormBase"));
    let mut override_btn = Node::new("btn_ok", "Button");
    override_btn.set_name("btn_ok");
    override_btn
        .events
        .insert("on_click".into(), "btn_derived_click".into());
    derived_doc.add_child("FormDerived", override_btn).unwrap();

    let arqs_derived = gerador
        .generate_with_base(&derived_doc, "app", &reg, Some(&base_doc), false)
        .expect("generate derived");
    for a in &arqs_derived {
        if a.caminho != Path::new("src/main.rs") {
            std::fs::write(destino.join(&a.caminho), &a.conteudo).unwrap();
        }
    }

    let derived_user_path = destino.join("src/FormDerived.rs");
    let derived_user_content = std::fs::read_to_string(&derived_user_path).unwrap();
    let derived_with_handler = derived_user_content.replace(
        "pub fn btn_derived_click(&mut self) {\n        self.base.btn_ok_click();\n    }",
        "pub fn btn_derived_click(&mut self) {\n        self.base.btn_ok_click();\n        println!(\"derived click\");\n    }",
    );
    std::fs::write(&derived_user_path, derived_with_handler).unwrap();

    let main_path = destino.join("src/main.rs");
    let main_content = "\
use eframe::egui;

mod FormBase;
#[path = \"FormBase.designer.rs\"]
pub mod FormBase_designer;
mod FormDerived;
#[path = \"FormDerived.designer.rs\"]
pub mod FormDerived_designer;

fn main() {
    let ctx = egui::Context::default();
    let mut form = FormDerived_designer::FormDerived::default();

    let raw_input = egui::RawInput::default();
    ctx.run_ui(raw_input, |ui| form.show(ui)).textures_delta.clear();

    let center = egui::pos2(80.0, 36.0);
    let mut raw_input = egui::RawInput::default();
    raw_input.events.push(egui::Event::PointerMoved(center));
    raw_input.events.push(egui::Event::PointerButton {
        pos: center,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::default(),
    });
    ctx.run_ui(raw_input, |ui| form.show(ui)).textures_delta.clear();

    let mut raw_input = egui::RawInput::default();
    raw_input.events.push(egui::Event::PointerButton {
        pos: center,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::default(),
    });
    ctx.run_ui(raw_input, |ui| form.show(ui)).textures_delta.clear();
}
";
    std::fs::write(&main_path, main_content).unwrap();

    let (ok, texto) = cargo(&destino, &["run", "-q"]);
    assert!(
        ok,
        "project with visual inheritance and event override should run:\n{texto}"
    );
    let base_idx = texto.find("base click").expect("base click in stdout");
    let derived_idx = texto
        .find("derived click")
        .expect("derived click in stdout");
    assert!(
        base_idx < derived_idx,
        "base click must be called before derived click: {texto}"
    );

    // Scenario 2: derived form does NOT override btn_ok
    let mut derived_doc_no_override = Document::new("FormDerived", "Form");
    {
        let root = derived_doc_no_override.node_mut("FormDerived").unwrap();
        root.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        root.set_name("FormDerived");
    }
    derived_doc_no_override.set_inherits(Some("FormBase"));

    let arqs_derived2 = gerador
        .generate_with_base(
            &derived_doc_no_override,
            "app",
            &reg,
            Some(&base_doc),
            false,
        )
        .expect("generate derived without override");
    for a in &arqs_derived2 {
        if a.caminho != Path::new("src/main.rs") {
            std::fs::write(destino.join(&a.caminho), &a.conteudo).unwrap();
        }
    }

    let (ok2, texto2) = cargo(&destino, &["run", "-q"]);
    assert!(
        ok2,
        "project with inheritance without override should run:\n{texto2}"
    );
    assert!(
        texto2.contains("base click"),
        "base click must be executed in default dispatch: {texto2}"
    );
    assert!(
        !texto2.contains("derived click"),
        "derived click must not be called when there is no override: {texto2}"
    );
}
