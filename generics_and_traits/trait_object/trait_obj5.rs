
// Use at least two approaches to make it work.
// DON'T add/remove any code line.
trait MyTrait {
  fn f(&self) -> Self;
}

impl MyTrait for u32 {
  fn f(&self) -> u32 { 42 }
}

impl MyTrait for String {
  fn f(&self) -> Self { self.clone() }
}

fn my_function<T: MyTrait>(x: T)  {
  x.f();
}

fn main() {
  my_function(13_u32);
  my_function(String::from("abc"));

  println!("Success!");
}
