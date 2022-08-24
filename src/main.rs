use rfd::FileDialog;

fn main() {
    let model = FileDialog::new()
        .add_filter("3D Model file", &["vrm", "glb", "gltf"])
        .pick_file();


    println!("{}", model.unwrap().display());
}
