extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

enum Direction {
  Right, Left, Up, Down
}

struct App {
  gl: GlGraphics,
  rotation: f64,
  pos_x; i32,
  pos_y: i32,
}

impl App {
  fn render(&mut self, args: &RenderArgs) {
    use graphics::*;
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    let square = graphics::rectangle::square(self.pos_x as f64, self.pos_y as f64, 20_f64;

    self.gl.draw(args.viewport(), |c, gl| {
      clear(BLACK, gl);
      let transform = c
        .transform
        .trans(self.pos_x, self.pos_y)
        .rot_rad(rotation)
        .trans(-25.0, -25.0);
      rectangle(GREEN, square, transform, gl);
    });

    self.snake.render(&mut self.gl, arg);
  }
  fn update(&mut self, args: &UpdateArgs) {
    match self.dir {
      self.pos_x += 1;
      self.pos_y += 1;
    }
  }
}

fn main() {
  let opengl = OpenGL::V3_2;
  let mut window: GlutinWindow = WindowSettings::new("snake", [200, 200])
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
  
  let mut app = App {
    gl: GlGraphics::new(opengl),
    snake: Snake { pos_x: 0, pos_y: 0, 10 }
    rotation: 0.0,
    pos_x: args.window_size[0] / 2.0,
    pos_y: args.window_size[1] / 2.0
  };

  let mut events = Events::new(EventSettings::new()).ups(8);
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      app.render(&args);
    }

    if let Some(args) = e.update_args() {
      app.update(&args);
    }
  }
}