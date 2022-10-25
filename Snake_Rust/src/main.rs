extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

struct App {
  gl: GlGraphics,
  snake: Snake,
  food: Food,
}

impl App {
  fn render(&mut self, args: &RenderArgs) {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    self.gl.draw(args.viewport(), |_c, gl| {
      graphics::clear(BLACK, gl)
    });

    self.snake.render(&mut self.gl, args);
    self.food.render(&mut self.gl, args);
  }
}

struct Snake {
  pos_x: i32,
  pos_y: i32,
}

impl Snake {
  fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {

    let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    let square = graphics::rectangle::square(self.pos_x as f64 + 1.0, self.pos_y as f64 + 1.0, 18_f64);

    gl.draw(args.viewport(), |c, gl| {
      let transform = c.transform;

      graphics::rectangle(red, square, transform, gl)
    })
  }
}

struct Food {
  pos_x: i32,
  pos_y: i32,
}

impl Food {
  fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {

    let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

    let square = graphics::rectangle::square(self.pos_x as f64, self.pos_y as f64, 20_f64);

    gl.draw(args.viewport(), |c, gl| {
      let transform = c.transform;

      graphics::rectangle(green, square, transform, gl)
    })
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
    snake: Snake { pos_x: 50, pos_y: 100 },
    food: Food { pos_x: 140, pos_y: 100 },
  };

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
      app.render(&r);
    }
  }
}