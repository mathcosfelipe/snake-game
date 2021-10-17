extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

struct Game{
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        use graphics;
        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl|{
          graphics::clear(GREEN, gl);  
        });
    }
}

struct Snake{
    pos_x:i32,
    pos_y:i32,
}

impl Snake{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;
        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = graphics::rectangle::square(self.pos_x as f64, self.pos_y as f64, 20_f64);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
         [200, 200]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let game = Game{
        gl: GlGraphics::new(opengl)
    }

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            app.render(&r);
        }
    }

}