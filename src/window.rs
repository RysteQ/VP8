macro_rules! get_pixel {
    ($x: expr, $y: expr) => {
        (x + y * 128) as usize
    };
}

pub mod window {
    use image::{self, DynamicImage, RgbImage, Pixel, Rgb, ImageBuffer, GenericImageView, GenericImage};
    use show_image::{self, create_window, WindowProxy};
    
    pub struct Window {
        window: WindowProxy,
        image: RgbImage,
        key_event_result: char
    }

    impl Window {
        pub fn init() -> Window {
            Window {
                window: create_window("Virtual Processor 8", Default::default()).unwrap(),
                image: RgbImage::new(128, 128),
                key_event_result: 'A'
            }
        }

        pub fn refresh(&self, screen_data: [char; 16384]) {

        }

        pub fn get_key_press(&self) -> char {
            self.key_event_result
        }

        fn convert_colour_data_to_image(&mut self, data_to_convert: [u8; 16384]) -> [u8; 49152] {
            let mut to_return: [u8; 49152];

            for x in 0..128 {
                for y in 0..128 {
                    let current_pixel: u8 = data_to_convert[get_pixel!(x, y)];

                    // I need to think this through on how to implement this
                }
            }

            to_return
        }
    }
}