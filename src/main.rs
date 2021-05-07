use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const LIGHT: [f32; 4] = [(240.0/255.0), (217.0/255.0), (181.0/255.0), 1.0];
        const DARK: [f32; 4] = [(181.0/255.0), (136.0/255.0), (99.0/255.0), 1.0];

        let square = rectangle::square(0.0, 0.0, 100.0);

        self.gl.draw(args.viewport(), |c, gl| {

            
            for i in 1..9{
                for j in 1..9{
                    if (j + i) % 2 == 0{
                        let transform = c
                        .transform
                        .trans((j - 1) as f64 * 100.0, (i - 1) as f64 * 100.0);

                        rectangle(LIGHT, square, transform, gl);
                    }
                    else{
                        let transform = c
                        .transform
                        .trans((j - 1) as f64 * 100.0, (i - 1) as f64 * 100.0);

                        rectangle(DARK, square, transform, gl);
                    }
                }
            }

        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Chess", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}