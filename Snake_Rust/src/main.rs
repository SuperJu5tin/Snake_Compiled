extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use std::{thread, time};
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;


struct App {
  gl: GlGraphics,
  snake: Snake,
  food: Food,
  score: i32,
  is_playing: bool,
}

impl App {

  fn add_snake(&mut self) {
    // println!("{}", self.snake.is_gamestart);
    if self.snake.is_gamestart {
      self.snake.pos.insert(0, vec![self.snake.pos[0][0] + self.snake.direction_list[0][0], self.snake.pos[0][1] + self.snake.direction_list[0][1]]);
      let mut is_food = false;
      for i in &self.snake.pos {
        let i: Vec<i32> = i.to_vec();
        if i == self.food.pos {
          is_food = true
        }
      }
      if is_food {
        self.food.randomize_food();
        self.score+=1
      } else {
        self.snake.pos.remove(self.snake.pos.len() - 1);
      }
    }
  }
  fn render(&mut self, args: &RenderArgs) {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    self.gl.draw(args.viewport(), |_c, gl| {
      graphics::clear(BLACK, gl)
    });

    self.snake.render(&mut self.gl, args);
    self.food.render(&mut self.gl, args);

    thread::sleep(time::Duration::from_millis(100));
  }
  fn check_snake(&mut self) {
    if self.snake.pos.iter().filter(|&n| *n == self.snake.pos[0]).count() == 2 {
      self.is_playing = false;
      println!("thanks for playing");
      println!("Score: {}", self.score );
    }
    for i in &self.snake.pos {
      let i: Vec<i32> = i.to_vec();
      if 0 > i[0] || i[0] > 480 || 0 > i[1] || i[1] > 360 {
        self.is_playing = false;
        println!("thanks for playing");
        println!("Score: {}", self.score );
      }
    }
  }
}

struct Snake {
  pos: Vec<Vec<i32>>,
  direction_list: Vec<Vec<i32>>,
  is_gamestart : bool,
  checks: Vec<bool>,
}

impl Snake {
  fn pressed(&mut self, btn: &Button) {
    if !self.is_gamestart {
      match btn {
        &Button::Keyboard(Key::Up) => {
          self.direction_list.insert(0, vec![0, -10]);
          self.is_gamestart = true
        },
        &Button::Keyboard(Key::Down) => {
          self.direction_list.insert(0, vec![0, 10]);
          self.is_gamestart = true
        },
        &Button::Keyboard(Key::Left) => {
          self.direction_list.insert(0, vec![-10, 0]);
          self.is_gamestart = true
        },
        &Button::Keyboard(Key::Right) => {
          self.direction_list.insert(0, vec![10, 0]);
          self.is_gamestart = true
        },
        _ => ()
      }
    }
    match btn {
      &Button::Keyboard(Key::Up) => self.check_dirrection(vec![0, -10]),
      &Button::Keyboard(Key::Down) => self.check_dirrection(vec![0, 10]),
      &Button::Keyboard(Key::Left) => self.check_dirrection(vec![-10, 0]),
      &Button::Keyboard(Key::Right) => self.check_dirrection(vec![10, 0]),
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
  fn check_duplication(&mut self, direction: Vec<i32>) {
    if self.direction_list[0] != vec![0, 0] && direction != self.direction_list[0]{
      self.checks[0] = true;
    } else {self.checks[0] = false };
  } 
  fn check_back_onself(&mut self, direction: Vec<i32>) {
    let right: Vec<i32> = vec![10, 0];
    let left: Vec<i32> = vec![-10, 0];
    let down: Vec<i32> = vec![0, 10];
    let up: Vec<i32> = vec![0, -10];
    if self.direction_list[0] == right && direction == left {self.checks[1] = false}
    if self.direction_list[0] == left && direction == right {self.checks[1] = false}
    if self.direction_list[0] == up && direction == down {self.checks[1] = false}
    if self.direction_list[0] == down && direction == up {self.checks[1] = false}
    // match &self.direction_list[0] {
    //   right => if direction == vec![-10, 0] {},
    //   left => if direction == vec![10, 0] {self.checks[1] = false},
    //   down => if direction == vec![0, -10] {self.checks[1] = false},
    //   up => if direction == vec![0, 10] {self.checks[1] = false},
    //   _ => self.checks[1] = true
    // }
  }
  fn check_dirrection(&mut self, direction: Vec<i32>) {
    self.check_duplication(direction.to_vec());
    self.check_back_onself(direction.to_vec());
    if self.checks[0] && self.checks[1] {
      self.direction_list.insert(0, direction)
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
  fn randomize_food(&mut self) {
    self.pos = vec![rand::thread_rng().gen_range(0, 48) * 10, rand::thread_rng().gen_range(0, 36) * 10]
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
      direction_list: vec![vec![0, 0]],
      is_gamestart: false,
      checks: vec![false, true],
    },
    food: Food { pos: vec![320, 180] },
    score: 0,
    is_playing: true,
  };

  let mut events = Events::new(EventSettings::new()).ups(20);
  while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
      app.check_snake();
      app.add_snake();
      app.render(&r);
    }
    if let Some(k) = e.button_args() {
      if k.state == ButtonState::Press {
        app.snake.pressed(&k.button);
      }
    }
    if let Some(_u) = e.update_args() {
      
      if !app.is_playing {
        break;
      }
    }
  }
}