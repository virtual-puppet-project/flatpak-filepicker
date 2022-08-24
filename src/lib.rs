use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
struct PortalDialog;

#[methods]
impl PortalDialog {
    fn new(_o: &Reference) -> Self {
        PortalDialog
    }

    #[export]
    fn execute(&self, _o: &Reference) -> GodotString {
        let dialog = rfd::FileDialog::new()
            .add_filter("3D Model file", &["vrm", "glb", "gltf"])
            .pick_file();

        GodotString::from_str(format!("{}", dialog.unwrap().display()))
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<PortalDialog>();
}

godot_init!(init);
