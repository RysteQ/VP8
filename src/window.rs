macro_rules! get_pixel {
    ($x: expr, $y: expr) => { 
        ($x + $y * 128) as usize
    };
}

pub mod window {
    use image::{self, DynamicImage, RgbImage, Pixel, Rgb, ImageBuffer, GenericImageView, GenericImage};
    use show_image::{self, create_window, WindowProxy};
    
    pub struct Window {
        window: WindowProxy,
        image_data: [Colours; 16384],
        image: RgbImage,
        key_event_result: char
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Colours {
        Black,
        Gray,
        White,
        Red,
        Green,
        Blue,
        Cyan,
        Orange,
        Brown,
        Gold,
        Yellow,
        DarkBlue,
        Silver
    }

    impl Window {
        pub fn init() -> Window {
            Window {
                window: create_window("Virtual Processor 8", Default::default()).unwrap(),
                image_data: [Colours::White; 16384],
                image: RgbImage::new(128, 128),
                key_event_result: 'A'
            }
        }

        pub fn refresh(&self, screen_data: [char; 16384]) {

        }

        pub fn get_key_press(&self) -> char {
            self.key_event_result
        }

        fn convert_colour_data_to_image(&mut self, data_to_convert: [u8; 16384]) {
            let colours: [(Colours, [u8; 3]); 12] = self.colours();

            for x in 0..128 {
                for y in 0..128 {
                    let current_pixel: u8 = data_to_convert[get_pixel!(x, y)];
                    self.change_colour(colours[current_pixel as usize].0, get_pixel!(x, y));
                }
            }
        }

        fn construct_image(&mut self) {
            let colour_values: [(Colours, [u8; 3]); 12] = self.colours();

            for x in 0..128 {
                for y in 0..128 {
                    for index in 0..colour_values.len() {
                        if self.image_data[index] == colour_values[index].0 {
                            self.image.put_pixel(x, y, image::Rgb(colour_values[index].1));
                        }
                    }
                }
            }
        }

        fn change_colour(&mut self, colour_to_change_to: Colours, index: usize) {
            self.image_data[index] = colour_to_change_to;
        }

        fn colours(&self) -> [(Colours, [u8; 3]); 12] {
            let colour_values: [(Colours, [u8; 3]); 12] = [
                (Colours::Black, [0, 0, 0]), (Colours::Gray, [127, 127, 127]), (Colours::White, [255, 255, 255]),
                (Colours::Red, [255, 0, 0]), (Colours::Green, [0, 255, 0]), (Colours::Blue, [0, 0, 255]),
                (Colours::Cyan, [0, 255, 255]), (Colours::Orange, [255, 165, 0]), (Colours::Brown, [165, 42, 42]),
                (Colours::Gold, [255, 215, 0]), (Colours::Yellow, [255, 255, 0]), (Colours::DarkBlue, [0, 0, 139])
            ];

            colour_values
        }
    }
}