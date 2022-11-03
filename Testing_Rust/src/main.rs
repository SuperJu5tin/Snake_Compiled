fn main() {
  struct Foo {
    stuff: Vec<Vec<i32>>
  }

  impl Foo {
    fn print(&self, num: usize, num2: usize) {
      println!("{}", self.stuff[num][num2])
    }
    fn add(&mut self, vec: i32) {
      self.stuff.push(vec![vec])
    }
  }

  let mut foo = Foo {
    stuff: vec![vec![21]]
  };

  foo.print(0, 0);
  foo.add(22);
  foo.print(1, 0);
  

  // let mut arr: Vec<Vec<i32>> = Vec::new();
  // arr.push(vec![21]);
  // println!("item {}", arr[0][0]);
}
