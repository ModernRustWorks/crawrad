#[test]
fn drawer_drawing_registered() {
    let d = crate::components::desenho_de("Drawer");
    assert_eq!(d.class_name(), "Drawer");
}
