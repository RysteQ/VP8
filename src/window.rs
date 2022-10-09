use piston_window::{self, Event, clear, rectangle, PistonWindow, WindowSettings};

macro_rules! pixel_coordinates {
    ($x: expr, $y: expr) => {
        [($x * 4) as f64, ($y * 4) as f64, ($x + 4) as f64, ($y + 4) as f64]
    };
}

macro_rules! pixel_index {
    ($x: expr, $y: expr) => {
        $x + $y * 128
    };
}

pub struct Window {
    window: PistonWindow,
    data_to_render: [u8; 16384],
}

impl Window {
    pub fn init() -> Window {
        Window {
            window: WindowSettings::new("Hello World!", [512; 2]).build().unwrap(),
            data_to_render: [0; 16384]
        }
    }

    pub fn set_screen_memory_data(&mut self, data: [u8; 16384]) {
        self.data_to_render = data;
    }

    pub fn update(&mut self, e: Event) {
        self.window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            
            for x in 0..128 {
                for y in 0..128 {
                    let rgb_data: [f32; 4] = Window::convert_bytes_to_colours(self.data_to_render[pixel_index!(x, y)]);

                    rectangle(rgb_data, pixel_coordinates!(x, y), c.transform, g);
                }
            }
        });
    }

    pub fn get_window_next(&mut self) -> Option<Event> {
        return self.window.next()
    }

    fn convert_bytes_to_colours(byte_to_analyze: u8) -> [f32; 4] {
        let colour_data: [[f32; 4]; 16] = [
            [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 1.0, 1.0], 
            [0.5, 0.0, 0.5, 1.0], [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0], [1.0, 1.0, 0.0, 1.0],
            [1.0, 0.647, 0.0, 1.0], [0.647, 0.1647, 0.1647, 1.0], [1.0, 0.8, 0.8, 1.0], [0.4117, 0.4117, 0.4117, 1.0],
            [0.5, 0.5, 0.5, 1.0], [0.0, 0.5, 0.0, 1.0], [0.0, 0.0, 0.5, 1.0], [0.25, 0.25, 0.25, 1.0]
        ];

        return colour_data[(byte_to_analyze & 0b00001111) as usize];
    }
}