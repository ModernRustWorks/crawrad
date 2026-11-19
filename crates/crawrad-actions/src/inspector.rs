//! What an agent can do to the **OBJECT INSPECTOR** — open the window behind
//! a property, close it, and fold a property open in place.
//!
//! # Why a family of its own, and not a corner of `LayoutCommand`
//!
//! [`LayoutCommand`](super::LayoutCommand) answers *where the panels are* and
//! [`ExplorerCommand`](super::ExplorerCommand) answers *what is open inside
//! the Explorer*. This answers *what is open inside the Object Inspector*, and
//! the three never travel together: resetting the layout must not close the
//! items editor, and opening the items editor must not move a panel. An agent
//! reading an [`Effect`](super::Effect) to find out what is about to happen
//! reads the family and already knows — this one writes nothing to disk,
//! launches no process and does not touch the document.
//!
//! # Why it is in the catalogue at all
//!
//! The rule of the house: *does an agent get to do this with a tool, without a
//! mouse?* Three of the eleven property kinds do not fit on a row —
//! `Multiline`, `Collection` and the visual picker of `Align` — and the only
//! way to reach them is a button that opens something. Before this module the
//! egui shell opened those windows by reaching into its own fields
//! (`ide.abrir_editor_items(&id)`), which is exactly the kind of capability
//! that never reaches the protocol, because nobody notices it is missing.
//!
//! # What this family does NOT decide
//!
//! **Which** window opens. The catalogue says *"open whatever edits this
//! property"*; the shell owns the table that turns a property into a window,
//! for the reason written at `crawrad-ide/src/inspector.rs:959-969` — the
//! registry is data, and a descriptor carrying the name of a dialog of the
//! shell would put the interface inside the data (ADR 0005).
//!
//! And it does not decide what the window **writes**. Everything a collection
//! editor or a colour picker produces goes back through
//! `component.set_property`, the same door the rows and the protocol already
//! use.

use crawrad_ir::NodeId;
use crawrad_note::note;

#[note("inspector-inspectorcommand")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectorCommand {
    /// Open whatever edits this property of this component.
    ///
    /// Both halves are needed and neither is redundant: the **key** says which
    /// property, and the **node** says whose. The selection is not enough —
    /// the gesture that opens the window is a click on a row, and clicking a
    /// row must not depend on the selection still being what it was when the
    /// list was drawn.
    OpenEditor { node: NodeId, key: String },
    /// Open the dialog of the child collection — the `Columns` of a `Grid`
    /// The same dialog a double-click on the component opens.
    OpenCollection { node: NodeId },
    /// Show the Events tab of the selected component.
    ShowEvents,
    /// Show the Properties tab with the cursor in the Name field.
    EditName,
    /// Close whatever is open, writing nothing.
    ///
    /// It exists as a command of its own rather than as "open nothing",
    /// because `Esc` and `Cancel` are a real gesture with a real meaning —
    /// *discard* — and an agent that opened a window has to be able to get out
    /// of it without guessing an argument.
    CloseEditor,
    /// Unfold the sub-rows of a property, or fold them away.
    ///
    /// **Toggle**, and not `Open`/`Close`, for the same reason
    /// [`ExplorerCommand::ToggleFolder`](super::ExplorerCommand::ToggleFolder)
    /// is: the user's gesture is one click on one chevron that goes both ways,
    /// and two commands over it would be two things to keep in step.
    ///
    /// No node here, and that is deliberate: `Margins` left open on one
    /// component stays open on the next one, which is what the Object
    /// Inspector of a classic RAD IDE does. Folding is a preference about a *property*,
    /// not a fact about a component.
    ToggleGroup { key: String },
    /// Fold a whole CATEGORY away — `Appearance`, `Layout` — or open it back.
    ///
    /// # Why it is not [`ToggleGroup`](Self::ToggleGroup)
    ///
    /// They look alike on screen and name different things. A **group** is one
    /// property that has sub-rows (`Margins` unfolds into four numbers); a
    /// **category** is the heading that gathers unrelated properties, and it
    /// comes from `crawrad_registry::Category`, not from a property key. Folding
    /// `Layout` hides `Rect`, `Anchors`, `Align` and `Margins` at once — and
    /// `Margins` keeps its own fold, underneath, for when the category opens
    /// again.
    ///
    /// Sharing one command would mean a single namespace where a category
    /// named like a property silently fights with it.
    ///
    /// # Why the catalogue owns it at all
    ///
    /// The egui shell folds with `egui::CollapsingHeader`, which keeps the
    /// state inside egui's own memory — nobody outside the mouse can reach it.
    /// That is exactly the capability that never arrives at the protocol
    /// because no one notices it is missing (the rule at the top of this
    /// module). Here the fold is an action, so an agent can open `Events`
    /// before reading it.
    ///
    /// No node, for the same reason as [`ToggleGroup`](Self::ToggleGroup):
    /// `Advanced` left closed stays closed on the next component. Folding is a
    /// preference about the PANEL, not a fact about a component — and it is
    /// what the classic RAD Object Inspector does.
    ToggleCategory { label: String },
    /// Move the splitter between the names and the values.
    ///
    /// The width is of the NAME column, in logical pixels, as in the classic
    /// RAD Object Inspector: widening the dock gives the values the room, and
    /// the names stay where the eye left them. The shell clamps it to what the
    /// panel can hold, so an agent may send any number. Like a fold, it is a
    /// preference about the panel, not a fact about a component — no node.
    /// Whole pixels: a column edge between two pixels draws blurred.
    SetNameWidth { width: i32 },
    /// Choose the file a `Picture` property shows — the `…` of its row.
    ///
    /// With no `file`, the shell opens the file dialog filtered to images; with
    /// one, it reads that file, so an agent sets a picture without a window.
    /// Either way the picture is written with `component.set_property`: one
    /// step of undo, the same door as typing a value.
    PickPicture {
        node: NodeId,
        key: String,
        file: Option<String>,
    },
    /// Choose the font a `Font` property shows — the `…` of its row.
    ///
    /// With no `font`, the shell opens the native font dialog; with
    /// one, it reads that formatted font, so an agent sets a font without a window.
    /// Either way the font is written with `component.set_property`: one
    /// step of undo, the same door as typing a value.
    PickFont {
        node: NodeId,
        key: String,
        font: Option<String>,
    },
}

#[cfg(test)]
#[path = "test/inspector/test.rs"]
mod tests;
