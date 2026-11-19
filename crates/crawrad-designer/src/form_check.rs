//! **The form checker**: what is wrong with a form, read from the
//! document alone — no window, no framework.
//!
//! Each rule looks at the [`Document`] and the [`Registry`] and says what it
//! found as a [`Finding`]: the controls involved (the first one carries the
//! mark on the canvas), the property behind it (the Inspector's ▲), a line of
//! text and, when the fix is obvious, a [`Fix`]. Overlapping siblings have no
//! fix: which one should move is the user's call.

use crawrad_doc::{Command, Transaction};
use crawrad_ir::{Color, Document, Node, NodeId, Rect, Value};
use crawrad_registry::Registry;

/// What a rule checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    Overlap,
    OutsideParent,
    Contrast,
    TabOrder,
    NoOnClick,
    /// A Label beside an input control has no FocusControl set.
    LabelLink,
}

impl Rule {
    /// The panel's name for the rule.
    pub fn label(self) -> &'static str {
        match self {
            Self::Overlap => "Overlap",
            Self::OutsideParent => "Outside parent",
            Self::Contrast => "Contrast",
            Self::TabOrder => "Tab order",
            Self::NoOnClick => "No OnClick",
            Self::LabelLink => "FocusControl",
        }
    }

    /// Controls that hide each other are errors; the rest are warnings.
    pub fn is_error(self) -> bool {
        matches!(self, Self::Overlap | Self::OutsideParent)
    }
}

/// The one-click fix of a finding.
#[derive(Debug, Clone, PartialEq)]
pub enum Fix {
    /// Move (and shrink, if it is bigger) the control into its parent.
    FitInside { node: NodeId, rect: Rect },
    /// Give the text this colour, which passes WCAG AA.
    FontColor { node: NodeId, color: Color },
    /// Number these siblings 0, 1, 2… in this order.
    Renumber(Vec<NodeId>),
    /// Create the handler and wire it (the Inspector's double-click).
    AddHandler {
        node: NodeId,
        event: String,
        handler: String,
    },
    /// Link this label to the control beside it.
    LinkLabel {
        node: NodeId,
        target: NodeId,
        target_name: String,
    },
}

impl Fix {
    /// The fix's button.
    pub fn label(&self) -> String {
        match self {
            Self::FitInside { .. } => "Fit inside".into(),
            Self::FontColor { color, .. } => format!("Use {}", hex(*color)),
            Self::Renumber(_) => "Renumber".into(),
            Self::AddHandler { .. } => "Add handler".into(),
            Self::LinkLabel { target_name, .. } => format!("Link to {target_name}"),
        }
    }

    /// The change, as one undo step. `None` for [`Fix::AddHandler`], which
    /// writes code too and goes through the `event.bind` action.
    pub fn transaction(&self, doc: &Document) -> Option<Transaction> {
        let mut tx = Transaction::user(format!("Fix: {}", self.label()));
        match self {
            Self::FitInside { node, rect } => tx.push(Command::SetRect {
                node: node.clone(),
                rect: *rect,
            }),
            Self::FontColor { node, color } => {
                let mut font = doc
                    .node(node)?
                    .props
                    .get("font")
                    .and_then(Value::as_font)
                    .cloned()
                    .unwrap_or_default();
                font.color = *color;
                tx.push(set(node, "font", Value::Font(font)));
                tx.push(set(node, "parent_font", Value::Bool(false)));
            }
            Self::Renumber(ids) => {
                for (i, id) in ids.iter().enumerate() {
                    tx.push(set(id, "tab_order", Value::Int(i as i64)));
                }
            }
            Self::AddHandler { .. } => return None,
            Self::LinkLabel { node, target, .. } => {
                tx.push(set(node, "focus_control", Value::Ref(target.clone())));
            }
        }
        Some(tx)
    }
}

fn set(node: &str, key: &str, value: Value) -> Command {
    Command::SetProp {
        node: node.into(),
        key: key.into(),
        value: Some(value),
    }
}

/// One thing wrong with the form.
#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    pub rule: Rule,
    /// The controls involved; the canvas mark sits on the first.
    pub nodes: Vec<NodeId>,
    /// The property behind it, marked in the Inspector.
    pub prop: Option<&'static str>,
    pub text: String,
    pub fix: Option<Fix>,
}

/// The colours a control without its own falls back to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub text: Color,
    pub surface: Color,
}

/// Everything wrong with `doc`, errors first, in form order within a rule.
pub fn check(doc: &Document, reg: &Registry, theme: Theme) -> Vec<Finding> {
    let c = Checker { doc, reg, theme };
    let mut out = Vec::new();
    for id in doc.in_paint_order() {
        let Some(n) = doc.node(&id) else { continue };
        c.outside(n, &mut out);
        c.contrast(n, &mut out);
        c.click(n, &mut out);
        c.label_link(n, &mut out);
    }
    for parent in std::iter::once(doc.root.clone()).chain(doc.in_paint_order()) {
        let Some(p) = doc.node(&parent) else { continue };
        c.overlaps(p, &mut out);
        c.tab_order(p, &mut out);
    }
    out.sort_by_key(|f| (!f.rule.is_error(), f.rule));
    out
}

struct Checker<'a> {
    doc: &'a Document,
    reg: &'a Registry,
    theme: Theme,
}

impl Checker<'_> {
    /// Drawn on the form at a place of its own: visual, shown, not laid out
    /// by `Align`, not a page or item its owner places.
    fn placed(&self, n: &Node) -> bool {
        let Some(desc) = self.reg.get(&n.class) else {
            return false;
        };
        let aligned = matches!(n.props.get("align"), Some(Value::Enum(a)) if a != "None");
        let hidden = matches!(n.props.get("visible"), Some(Value::Bool(false)));
        let owned = self
            .doc
            .parent_of(&n.id)
            .and_then(|p| self.doc.node(p))
            .and_then(|p| self.reg.get(&p.class))
            .is_some_and(|p| p.item_class() == Some(n.class.as_str()));
        desc.visual && !desc.derived_geometry && !aligned && !hidden && !owned && n.rect().is_some()
    }

    fn outside(&self, n: &Node, out: &mut Vec<Finding>) {
        let (Some(r), Some(parent)) = (n.rect(), self.doc.parent_of(&n.id)) else {
            return;
        };
        let Some(p) = self.doc.node(parent) else {
            return;
        };
        let Some(area) = p.rect() else { return };
        if !self.placed(n) {
            return;
        }
        let past = [
            (-r.x, "left"),
            (-r.y, "top"),
            (r.x + r.w - area.w, "right"),
            (r.y + r.h - area.h, "bottom"),
        ];
        let Some((by, edge)) = past
            .into_iter()
            .filter(|(by, _)| *by >= 1.0)
            .max_by(|a, b| a.0.total_cmp(&b.0))
        else {
            return;
        };
        let (w, h) = (r.w.min(area.w), r.h.min(area.h));
        let rect = Rect::new(r.x.clamp(0.0, area.w - w), r.y.clamp(0.0, area.h - h), w, h);
        out.push(Finding {
            rule: Rule::OutsideParent,
            nodes: vec![n.id.clone(), p.id.clone()],
            prop: Some("rect"),
            text: format!(
                "{} ends {} px past the {edge} edge of {}",
                n.name(),
                by.round(),
                p.name()
            ),
            fix: Some(Fix::FitInside {
                node: n.id.clone(),
                rect,
            }),
        });
    }

    fn overlaps(&self, p: &Node, out: &mut Vec<Finding>) {
        let kids: Vec<(&Node, Rect)> = p
            .children
            .iter()
            .filter_map(|c| self.doc.node(c))
            .filter(|n| self.placed(n))
            .filter_map(|n| Some((n, n.rect()?)))
            .collect();
        for (i, (a, ra)) in kids.iter().enumerate() {
            for (b, rb) in &kids[i + 1..] {
                let w = (ra.x + ra.w).min(rb.x + rb.w) - ra.x.max(rb.x);
                let h = (ra.y + ra.h).min(rb.y + rb.h) - ra.y.max(rb.y);
                if w < 1.0 || h < 1.0 {
                    continue;
                }
                out.push(Finding {
                    rule: Rule::Overlap,
                    nodes: vec![b.id.clone(), a.id.clone()],
                    prop: Some("rect"),
                    text: format!(
                        "{} and {} overlap by {} \u{00d7} {} px",
                        a.name(),
                        b.name(),
                        w.round(),
                        h.round()
                    ),
                    fix: None,
                });
            }
        }
    }

    /// The colour `key` resolves to: its own, unless `parent_key` says the
    /// parent's; alpha 0 is the theme's.
    fn resolved(
        &self,
        id: &str,
        pick: &dyn Fn(&Node) -> Option<Color>,
        parent_key: &str,
    ) -> Option<Color> {
        let mut at = id.to_string();
        for _ in 0..64 {
            let n = self.doc.node(&at)?;
            let inherits = !matches!(n.props.get(parent_key), Some(Value::Bool(false)));
            let own = pick(n).filter(|c| c.a > 0);
            match (own, self.doc.parent_of(&at)) {
                (Some(c), _) if !inherits || at == self.doc.root => return Some(c),
                (_, Some(p)) if inherits => at = p.clone(),
                _ => return own,
            }
        }
        None
    }

    fn contrast(&self, n: &Node, out: &mut Vec<Finding>) {
        let has_font = self
            .reg
            .get(&n.class)
            .is_some_and(|d| d.prop("font").is_some());
        if !has_font || n.text().is_none_or(|t| t.trim().is_empty()) || !self.placed(n) {
            return;
        }
        let font_color = |n: &Node| {
            n.props
                .get("font")
                .and_then(Value::as_font)
                .map(|f| f.color)
        };
        let back = |n: &Node| match n.props.get("color") {
            Some(Value::Color(c)) => Some(*c),
            _ => None,
        };
        let fg = self.resolved(&n.id, &font_color, "parent_font");
        let bg = self.resolved(&n.id, &back, "parent_color");
        if fg.is_none() && bg.is_none() {
            return; // the theme's own pair, which the theme answers for
        }
        let (fg, bg) = (
            fg.unwrap_or(self.theme.text),
            bg.unwrap_or(self.theme.surface),
        );
        let font = n.props.get("font").and_then(Value::as_font);
        let size = font.map_or(0.0, |f| f.size);
        let large = size >= 18.0 || (size >= 14.0 && font.is_some_and(|f| f.bold));
        let need = if large { 3.0 } else { 4.5 };
        let ratio = contrast(fg, bg);
        if ratio >= need {
            return;
        }
        out.push(Finding {
            rule: Rule::Contrast,
            nodes: vec![n.id.clone()],
            prop: Some("font"),
            text: format!(
                "{} text {} on {} is {:.1}:1 \u{2014} WCAG AA needs {need}:1",
                n.name(),
                hex(fg),
                hex(bg),
                ratio
            ),
            fix: Some(Fix::FontColor {
                node: n.id.clone(),
                color: passing(fg, bg, need),
            }),
        });
    }

    fn click(&self, n: &Node, out: &mut Vec<Finding>) {
        let is_button = matches!(n.class.as_str(), "Button" | "BitBtn" | "SpeedButton");
        let is_item = n.class == "MenuItem" && n.children.is_empty() && n.text() != Some("-");
        let handled = n
            .events
            .get("on_click")
            .is_some_and(|h| !h.trim().is_empty());
        let acted = matches!(n.props.get("action"), Some(Value::Ref(a)) if !a.is_empty());
        let modal = matches!(n.props.get("modal_result"), Some(Value::ModalResult(m)) if *m != crawrad_ir::ModalResult::None);
        if !(is_button || is_item) || handled || acted || modal {
            return;
        }
        let text = match is_item {
            true => format!("{} has no OnClick", self.menu_path(n)),
            false => format!("{} does nothing when clicked", n.name()),
        };
        out.push(Finding {
            rule: Rule::NoOnClick,
            nodes: vec![n.id.clone()],
            prop: None,
            text,
            fix: Some(Fix::AddHandler {
                node: n.id.clone(),
                event: "on_click".into(),
                handler: crawrad_ir::default_handler_name(n.name(), "on_click"),
            }),
        });
    }

    /// `MainMenu1 › File › Export`.
    fn menu_path(&self, n: &Node) -> String {
        let mut parts = vec![n.text().unwrap_or(n.name()).to_string()];
        for a in self.doc.ancestors(&n.id) {
            let Some(a) = self.doc.node(&a).filter(|_| a != self.doc.root) else {
                break;
            };
            parts.push(match a.class.as_str() {
                "MenuItem" => a.text().unwrap_or(a.name()).to_string(),
                _ => a.name().to_string(),
            });
        }
        parts.reverse();
        parts.join(" \u{203a} ")
    }

    fn label_link(&self, n: &Node, out: &mut Vec<Finding>) {
        if n.class != "Label" || !self.placed(n) {
            return;
        }
        let unset = match n.props.get("focus_control") {
            None => true,
            Some(Value::Ref(s)) => s.trim().is_empty(),
            _ => false,
        };
        if !unset {
            return;
        }
        let Some(parent_id) = self.doc.parent_of(&n.id) else {
            return;
        };
        let Some(parent) = self.doc.node(parent_id) else {
            return;
        };
        let Some(l) = n.rect() else {
            return;
        };
        let mut best: Option<(&Node, f32)> = None;
        for child_id in &parent.children {
            if child_id == &n.id {
                continue;
            }
            let Some(candidate) = self.doc.node(child_id) else {
                continue;
            };
            if !self.placed(candidate) {
                continue;
            }
            let Some(desc) = self.reg.get(&candidate.class) else {
                continue;
            };
            if !desc.answers_to(crawrad_registry::props::WIN_CONTROL) {
                continue;
            }
            if !desc.events.iter().any(|e| e.name == "on_change") {
                continue;
            }
            if desc.prop("checked").is_some() {
                continue;
            }
            let Some(t) = candidate.rect() else {
                continue;
            };
            let right = t.x >= l.x + l.w - 4.0
                && t.x - (l.x + l.w) <= 24.0
                && ((l.y + l.h / 2.0) - (t.y + t.h / 2.0)).abs() <= l.h.max(t.h) / 2.0;
            let below =
                (t.x - l.x).abs() <= 8.0 && t.y >= l.y + l.h - 2.0 && t.y - (l.y + l.h) <= 12.0;
            let gap = match (right, below) {
                (true, true) => (t.x - (l.x + l.w)).min(t.y - (l.y + l.h)),
                (true, false) => t.x - (l.x + l.w),
                (false, true) => t.y - (l.y + l.h),
                (false, false) => continue,
            };
            if best.is_none_or(|(_, best_gap)| gap < best_gap) {
                best = Some((candidate, gap));
            }
        }
        let Some((target, _)) = best else {
            return;
        };
        out.push(Finding {
            rule: Rule::LabelLink,
            nodes: vec![n.id.clone(), target.id.clone()],
            prop: Some("focus_control"),
            text: format!(
                "{} labels {} but its FocusControl is empty",
                n.name(),
                target.name()
            ),
            fix: Some(Fix::LinkLabel {
                node: n.id.clone(),
                target: target.id.clone(),
                target_name: target.name().to_string(),
            }),
        });
    }

    fn tab_order(&self, p: &Node, out: &mut Vec<Finding>) {
        let mut stops: Vec<(i64, &Node)> = p
            .children
            .iter()
            .filter_map(|c| self.doc.node(c))
            .filter_map(|n| Some((n.props.get("tab_order")?.as_int().unwrap_or(0), n)))
            .collect();
        stops.sort_by_key(|(t, _)| *t);
        let mut dup: Vec<(i64, Vec<&str>)> = Vec::new();
        for w in stops.windows(2) {
            if w[0].0 == w[1].0 {
                match dup.last_mut() {
                    Some((t, names)) if *t == w[0].0 => names.push(w[1].1.name()),
                    _ => dup.push((w[0].0, vec![w[0].1.name(), w[1].1.name()])),
                }
            }
        }
        // Inside a container a gap is harmless: the IDE numbers new controls
        // form-wide, so only the form's own sequence is held to 0, 1, 2…
        let unused: Vec<i64> = match p.id == self.doc.root {
            true => (0..stops.len() as i64)
                .filter(|i| !stops.iter().any(|(t, _)| t == i))
                .collect(),
            false => Vec::new(),
        };
        if dup.is_empty() && unused.is_empty() {
            return;
        }
        let mut parts: Vec<String> = dup
            .iter()
            .map(|(t, names)| format!("{} {} TabOrder {t}", and_list(names), both(names.len())))
            .collect();
        if !unused.is_empty() {
            let list: Vec<String> = unused.iter().map(i64::to_string).collect();
            let verb = if unused.len() == 1 { "is" } else { "are" };
            parts.push(format!("{} {verb} unused", and_list(&list)));
        }
        // The mark sits on the second of the first tie (the one that came
        // late); the others tied follow, so the canvas can colour them.
        let tied: Vec<NodeId> = stops
            .iter()
            .filter(|(t, _)| dup.iter().any(|(d, _)| d == t))
            .map(|(_, n)| n.id.clone())
            .collect();
        let mark = tied.get(1).or(tied.first()).cloned();
        let mut nodes: Vec<NodeId> = mark.clone().into_iter().collect();
        nodes.extend(tied.iter().filter(|id| Some(*id) != mark.as_ref()).cloned());
        if nodes.is_empty() {
            nodes.extend(stops.first().map(|(_, n)| n.id.clone()));
        }
        out.push(Finding {
            rule: Rule::TabOrder,
            nodes,
            prop: Some("tab_order"),
            text: parts.join("; "),
            fix: Some(Fix::Renumber(
                stops.iter().map(|(_, n)| n.id.clone()).collect(),
            )),
        });
    }
}

fn both(n: usize) -> &'static str {
    if n == 2 { "both have" } else { "all have" }
}

fn and_list<S: AsRef<str>>(items: &[S]) -> String {
    match items {
        [] => String::new(),
        [one] => one.as_ref().to_string(),
        [rest @ .., last] => {
            let rest: Vec<&str> = rest.iter().map(AsRef::as_ref).collect();
            format!("{} and {}", rest.join(", "), last.as_ref())
        }
    }
}

/// `#5B5F63`.
pub fn hex(c: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b)
}

fn luminance(c: Color) -> f64 {
    let lin = |v: u8| {
        let s = f64::from(v) / 255.0;
        if s <= 0.039_28 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(c.r) + 0.7152 * lin(c.g) + 0.0722 * lin(c.b)
}

/// The WCAG contrast ratio of two colours, 1 to 21.
pub fn contrast(a: Color, b: Color) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// The colour nearest `fg` — moved toward white or black, whichever the
/// background leaves room for — that reaches `need` against `bg`.
fn passing(fg: Color, bg: Color, need: f64) -> Color {
    let lighter = contrast(Color::rgb(255, 255, 255), bg) >= contrast(Color::rgb(0, 0, 0), bg);
    let to = if lighter { 255.0 } else { 0.0 };
    let mix = |v: u8, t: f64| (f64::from(v) + (to - f64::from(v)) * t).round() as u8;
    for step in 1..=100 {
        let t = f64::from(step) / 100.0;
        let c = Color::rgb(mix(fg.r, t), mix(fg.g, t), mix(fg.b, t));
        if contrast(c, bg) >= need {
            return c;
        }
    }
    Color::rgb(to as u8, to as u8, to as u8)
}

#[cfg(test)]
#[path = "test/form_check/test.rs"]
mod tests;
