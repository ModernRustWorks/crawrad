use crate::{ComponentDesc, Props};
use crawrad_note::note;

#[note("components-mask-edit-mask-edit")]
pub fn mask_edit() -> ComponentDesc {
    let mut d = crate::components::edit::edit();
    d.class = "MaskEdit";
    d.display = "MaskEdit";
    d.base_name = "mask_edit";
    d.palette = "Additional";
    d.prefix = "msk";
    for p in d.props.iter_mut().filter(|p| p.key == "text") {
        p.bindable = false;
    }
    d.props.push(Props::edit_mask(""));
    d
}
