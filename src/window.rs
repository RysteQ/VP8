pub mod window {
    extern crate glutin_window;
    extern crate graphics;
    extern crate opengl_graphics;
    extern crate piston;

    use piston::input::RenderArgs;
    use graphics::{Context, rectangle, Transformed};
    use opengl_graphics::{GlGraphics, OpenGL};
    use graphics::clear;

    pub struct Window {
        gl: GlGraphics,
        data_to_render: [[f32; 4]; 16384],
    }

    impl Window {
        pub fn init(open_gl: OpenGL) -> Window {
            Window {
                gl: GlGraphics::new(open_gl),
                data_to_render: [[1.0; 4]; 16384]
            }
        }

        pub fn set_screen_memory_data(&mut self, data: [u8; 16384]) {
            for i in 0..16384 {
                self.data_to_render[i] = Window::convert_bytes_to_colours(data[i], i as u16 + 256);
            }
        }

        pub fn update(&mut self, args: RenderArgs) {
            self.gl.draw(args.viewport(), |_c: Context, gl: &mut GlGraphics| {
                clear([0.0, 0.0, 0.0, 1.0], gl);

                for x in 0..128 {
                    for y in 0..128 {
                        let transform = _c
                            .transform
                            .trans(x as f64, y as f64)
                            .rot_rad(0.0)
                            .trans(0.0, 0.0);

                        rectangle(self.data_to_render[x + y * 128], rectangle::square(x as f64, y as f64, 4.0), transform, gl);
                    }
                }
            });
        }

        fn convert_bytes_to_colours(byte_to_analyze: u8, memory_location: u16) -> [f32; 4] {
            let colour_data: [[f32; 4]; 16] = [
                [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 1.0, 1.0], 
                [0.5, 0.0, 0.5, 1.0], [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0], [1.0, 1.0, 0.0, 1.0],
                [1.0, 0.647, 0.0, 1.0], [0.647, 0.1647, 0.1647, 1.0], [1.0, 0.8, 0.8, 1.0], [0.4117, 0.4117, 0.4117, 1.0],
                [0.5, 0.5, 0.5, 1.0], [0.0, 0.5, 0.0, 1.0], [0.0, 0.0, 0.5, 1.0], [0.25, 0.25, 0.25, 1.0]
            ];

            if byte_to_analyze < 16 {
                return colour_data[byte_to_analyze as usize]
            } else {
                return colour_data[(byte_to_analyze & 0b00001111) as usize];
            }
        }
    }
}