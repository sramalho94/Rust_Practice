
// Use at least two approaches to make it work.
// DON'T add/remove any code line.
trait MyTrait {
  fn f(&self) -> Self;
}

impl MyTrait for u32 {
  fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
  fn f(&self) -> Self { self.clone() }
}

fn my_function(x: Box<dyn MyTrait>)  {
  x.f()
}

fn main() {
  my_function(Box::new(13_u32));
  my_function(Box::new(String::from("abc")));

  println!("Success!");
}
