fn main(){
  let x: i32 = 5;
  let y = &x;

  // Modify this line only
  assert_eq!(5, *y);

  println!("Success!");
}