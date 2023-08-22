// Modify `assert_eq!` to make it work

fn main(){
  let x: u32 = 5;

  assert_eq!("u32".to_string(), type_of(&x));

  println!("Success!");
}

// Get the type of given variable, return a string representation of the type

fn type_of<T>(_: &T) -> String {
  format!("{}", std::any::type_name::<T>())
}