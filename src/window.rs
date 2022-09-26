use image::{self, DynamicImage};
use show_image;

pub fn init() {
    
}

pub fn refresh(data: [u8; 16384]) {

}

pub fn get_key_press() -> char {
    let mut to_return: char = 'a';

    to_return
}

fn convert_colour_data_to_image(data_to_convert: [u8; 16384]) -> DynamicImage {
    let mut to_return: DynamicImage = DynamicImage::new_rgb8(128, 128);

    to_return
}