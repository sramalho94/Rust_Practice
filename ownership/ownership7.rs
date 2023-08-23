fn main() {
  let t: (String,String) = (String::from("hello"), String::from("world"));

  // Fill in the blanks
  let (s1, s2) = t.clone();

  println!("{:?}, {:?}, {:?}", s1, s2, t);
}