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

enum Direction{
    Right, Left, Up, Down
}

struct Snake{
    pos_x:i32,
    pos_y:i32,
    direction: Direction,
}

impl Snake{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;
        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20_f64);
        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;
            graphics::rectangle(RED, square, transform, gl)
        });
    }
}

fn main(){
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
         [200, 200]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game{
        gl: GlGraphics::new(opengl),
        snake: Snake {pos_x: 9, pos_y: 9, dir: Direction::Right},
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            game.render(&r);
        }
    }

}