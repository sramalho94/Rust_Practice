
// FILL in the blank and FIX errors
fn main() {
  let s = String::from("hello, 世界");
  let slice1 = s[0]; //tips: `h` only takes 1 byte in UTF8 format
  assert_eq!(slice1, "h");

  let slice2 = &s[3..5]; // Tips: `中`  takes 3 bytes in UTF8 format
  assert_eq!(slice2, "世");
  
  // Iterate through all chars in s
  for (i, c) in s.__ {
      if i == 7 {
          assert_eq!(c, '世')
      }
  }

  println!("Success!");
}
