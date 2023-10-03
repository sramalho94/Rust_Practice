
// Question: how many heap allocations are happening here?
// Your answer: 2
fn main() {  
  // Create a String type based on `&str`
  // The type of string literals is `&str`
  let s: String = String::from("hello, world!"); // 1

 // Create a slice point to String `s`
  let slice: &str = &s;

 // Create a String type based on the recently created slice
  let s: String = slice.to_string(); // 2

  assert_eq!(s, "hello, world!");

  println!("Success!");
}
