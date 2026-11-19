//! The centre's texts live in `crawrad_project::source` (pure: no window);
//! [`Center`] is the host's view, under the name the shell's call-sites use.
pub use crawrad_project::source::*;
pub type Center = crawrad_host::ViewCenter;

pub fn to_center_view(center: Center) -> CenterView {
    match center {
        Center::Code | Center::History => CenterView::Code,
        Center::FormSource
        | Center::Design
        | Center::Sizes
        | Center::Translations
        | Center::Welcome => CenterView::FormSource,
    }
}
