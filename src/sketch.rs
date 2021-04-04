use photon_rs::{
    channels::invert, conv::gaussian_blur, monochrome::grayscale_human_corrected,
    multiple::blend, PhotonImage,
};

fn dodge(img_inverted: &mut PhotonImage, img_grayscale: &PhotonImage) -> PhotonImage {
    let img_inv_v8: Vec<u8> = img_inverted.get_raw_pixels();
    let img_gray_v8: Vec<u8> = img_grayscale.get_raw_pixels();
    let end = img_inv_v8.len();

    let mut res_img = Vec::<u8>::new();

    for i in (0..end).step_by(4) {
        let r_inv = img_inv_v8[i] as u32;
        let g_inv = img_inv_v8[i + 1] as u32;
        let b_inv = img_inv_v8[i + 2] as u32;

        let mut r_gray = img_gray_v8[i] as u32;
        let mut g_gray = img_gray_v8[i + 1] as u32;
        let mut b_gray = img_gray_v8[i + 2] as u32;

        if r_gray == 255 {
            r_gray = 254;
        }
        if g_gray == 255 {
            g_gray = 254;
        }
        if b_gray == 255 {
            b_gray = 254;
        }

        let mut r_res = (r_inv * 255) / (255 - r_gray);
        if r_res > 255 {
            r_res = 255;
        }
        if r_gray == 255 {
            r_res = 255;
        }
        let mut g_res = (g_inv * 255) / (255 - g_gray);
        if g_res > 255 {
            g_res = 255;
        }
        if g_gray == 255 {
            g_res = 255;
        }
        let mut b_res = (b_inv * 255) / (255 - b_gray);
        if b_res > 255 {
            b_res = 255;
        }
        if b_gray == 255 {
            b_res = 255;
        }

        res_img.push(r_res as u8);
        res_img.push(g_res as u8);
        res_img.push(b_res as u8);
        res_img.push(img_gray_v8[i + 3]);
    }
    PhotonImage::new(
        res_img,
        img_grayscale.get_width(),
        img_grayscale.get_height(),
    )
}

pub fn sketch(img: &mut PhotonImage, radius: i32) -> PhotonImage {
    grayscale_human_corrected(img);
    let bw_img = img.clone();
    invert(img);
    gaussian_blur(img, radius);
    let mut ret = dodge(img, &bw_img);
    blend(&mut ret, &bw_img, "lighten");
    ret
}
