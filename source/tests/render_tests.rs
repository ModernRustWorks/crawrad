use crawrad_component::canvas::{Canvas, Paint};
use crawrad_components_source::{additional, dialogs, standard, system, win32};

#[test]
fn all_reference_components_paint_without_panic() {
    let mut canvas = Canvas::new(400.0, 400.0);

    // Standard
    standard::Button::default().paint(&mut canvas);
    standard::Label::default().paint(&mut canvas);
    standard::Panel::default().paint(&mut canvas);
    standard::Edit::default().paint(&mut canvas);
    standard::CheckBox::default().paint(&mut canvas);
    standard::RadioButton::default().paint(&mut canvas);
    standard::RadioGroup::default().paint(&mut canvas);
    standard::GroupBox::default().paint(&mut canvas);
    standard::Memo::default().paint(&mut canvas);
    standard::ComboBox::default().paint(&mut canvas);
    standard::ListBox::default().paint(&mut canvas);
    standard::PageControl::default().paint(&mut canvas);
    standard::ScrollBar::default().paint(&mut canvas);
    standard::ActionList::default().paint(&mut canvas);
    standard::Action::default().paint(&mut canvas);
    standard::MainMenu::default().paint(&mut canvas);
    standard::PopupMenu::default().paint(&mut canvas);

    // Additional
    additional::BitBtn::default().paint(&mut canvas);
    additional::Image::default().paint(&mut canvas);
    additional::PaintBox::default().paint(&mut canvas);
    additional::ScrollBox::default().paint(&mut canvas);
    additional::Shape::default().paint(&mut canvas);
    additional::SpeedButton::default().paint(&mut canvas);
    additional::SpinEdit::default().paint(&mut canvas);
    additional::Toggle::default().paint(&mut canvas);
    additional::ColorBox::default().paint(&mut canvas);
    additional::LinkLabel::default().paint(&mut canvas);
    additional::MaskEdit::default().paint(&mut canvas);
    additional::Bevel::default().paint(&mut canvas);
    additional::Badge::default().paint(&mut canvas);
    additional::Card::default().paint(&mut canvas);
    additional::Drawer::default().paint(&mut canvas);
    additional::CategoryPanel::default().paint(&mut canvas);
    additional::Chips::default().paint(&mut canvas);
    additional::Breadcrumb::default().paint(&mut canvas);
    additional::Pagination::default().paint(&mut canvas);
    additional::MultiSelect::default().paint(&mut canvas);
    additional::Splitter::default().paint(&mut canvas);

    // Win32
    win32::DateTimePicker::default().paint(&mut canvas);
    win32::ImageList::default().paint(&mut canvas);
    win32::ProgressBar::default().paint(&mut canvas);
    win32::StatusBar::default().paint(&mut canvas);
    win32::ToolBar::default().paint(&mut canvas);
    win32::TrackBar::default().paint(&mut canvas);
    win32::TreeView::default().paint(&mut canvas);
    win32::ListView::default().paint(&mut canvas);

    // System
    system::Timer::default().paint(&mut canvas);

    // Dialogs
    dialogs::OpenDialog::default().paint(&mut canvas);
    dialogs::SaveDialog::default().paint(&mut canvas);
    dialogs::MessageDialog::default().paint(&mut canvas);

    assert!(
        !canvas.commands().is_empty(),
        "The reference components must emit drawing commands to the 2D Canvas"
    );
}
