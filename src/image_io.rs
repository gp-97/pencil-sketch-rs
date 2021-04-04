use photon_rs::{
    native::{open_image, save_image},
    PhotonImage,
};

pub fn read(path: &str) -> PhotonImage {
    let img = open_image(path).unwrap_or_else(|_err| {
        eprintln!("[ERROR]: File not found. Check filename and/or filepath");
        std::process::exit(1);
    });
    img
}

pub fn write(img: PhotonImage, path: &str) {
    save_image(img, path);
}
