use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
struct PortalDialog;

#[methods]
impl PortalDialog {
    fn new(_o: &Reference) -> Self {
        PortalDialog
    }

    #[method]
    fn get_file(&self) -> GodotString {
        match rfd::FileDialog::new().pick_file() {
            Some(path) => GodotString::from_str(format!("{}", path.display())),
            None => GodotString::new(),
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<PortalDialog>();
}

godot_init!(init);
