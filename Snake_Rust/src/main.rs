extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng::*;


struct App {
  gl: GlGraphics,
  snake: Snake,
  food: Food,
}

impl App {
  fn add_snake(&mut self) {
    self.snake.pos.insert(0, vec![self.snake.pos[0][0] + self.snake.direction_list[0][0], self.snake.pos[0][1] + self.snake.direction_list[0][1]]);
    let mut is_food = false;
    for i in &self.snake.pos {
      println!("{}", i[0]);
      let i: Vec<i32> = i.to_vec();
      if i == self.food.pos {
        is_food = true
      }
    }
    if is_food {
      self.food.randomize()
    } else {
      self.snake.pos.remove(self.snake.pos.len() - 1);
    }
  }
  fn render(&mut self, args: &RenderArgs) {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    self.gl.draw(args.viewport(), |_c, gl| {
      graphics::clear(BLACK, gl)
    });

    self.add_snake();

    self.snake.render(&mut self.gl, args);
    self.food.render(&mut self.gl, args);
  }
}

struct Snake {
  pos: Vec<Vec<i32>>,
  direction_list: Vec<Vec<i32>>,
  direction: Vec<i32>
}

impl Snake {
  
  fn pressed(&mut self, btn: &Button) {
    match btn {
      &Button::Keyboard(Key::Up) => self.direction_list.insert(0, vec![0, -10]),
      &Button::Keyboard(Key::Down) => self.direction_list.insert(0, vec![0, 10]),
      &Button::Keyboard(Key::Left) => self.direction_list.insert(0, vec![-10, 0]),
      &Button::Keyboard(Key::Right) => self.direction_list.insert(0, vec![10, 0]),
      _ => ()
    }
  }
  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {

    let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    for i in &self.pos {
      let i: Vec<i32> = i.to_vec();
      let square = graphics::rectangle::square(i[0] as f64 + 1.0, i[1] as f64 + 1.0, 9_f64);
      gl.draw(args.viewport(), |c, gl| {
        let transform = c.transform;

        graphics::rectangle(red, square, transform, gl)
      })
    }
  } 
}

struct Food {
  pos: Vec<i32>,
}

impl Food {
  fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {

    let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

    let square = graphics::rectangle::square(self.pos[0] as f64, self.pos[1] as f64, 10_f64);

    gl.draw(args.viewport(), |c, gl| {
      let transform = c.transform;

      graphics::rectangle(green, square, transform, gl)
    })
  }
  fn randomize(&self) {
    use rand::Rng;
    self.pos = vec![mul(rand::thread_rng(), 10), mul(rand::thread_rng(), 10)];
  }
}

fn main() {
  let opengl = OpenGL::V3_2;
  let mut window: GlutinWindow = WindowSettings::new("snake", [480, 360])
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  let mut app = App {
    gl: GlGraphics::new(opengl),
    snake: Snake {
      pos: vec![vec![160, 180]],
      direction_list: vec![vec![10, 0]],
      direction: vec![10, 0]
    },
    food: Food { pos: vec![32, 18]},
  };

  let mut events = Events::new(EventSettings::new()).ups(10);
  while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
      app.render(&r);
    }
    if let Some(k) = e.button_args() {
      if k.state == ButtonState::Press {
        app.snake.pressed(&k.button);
      }
    }
  }
}