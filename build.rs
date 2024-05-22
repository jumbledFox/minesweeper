// This is used to generate the pixel arrays for the program icon
// Too bad macroquad on linux doesn't support icons :|
// It does however appear when running a windows build through wine!! :3

use std::{env, fs::File, path::Path, io::Write};

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("icon_data.rs");
    let mut f = File::create(&dest_path).expect("Failed to create file");

    for (name, path, size) in [
        ("SMALL",  "resources/icon_small.png",  1024),
        ("MEDIUM", "resources/icon_medium.png", 4096),
        ("BIG",    "resources/icon_big.png",    16384),
    ] {
        let img = image::open(path).expect("Failed to open image");
        let img_bytes = img.as_bytes();
        write!(f, "pub const ICON_{}: [u8; {:?}] = {:?};", name, size, img_bytes).expect("Failed to write into image");
    }
}