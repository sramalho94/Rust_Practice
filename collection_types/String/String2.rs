/** A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.

&str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>. **/

// FILL in the blanks
fn main() {  
  let mut s: String = String::from("hello, world");

  let slice1: &str = &s; // or s.as_str()
  assert_eq!(slice1, "hello, world");

  let slice2: &str = &s[..=4];
  assert_eq!(slice2, "hello");

  let slice3: &mut String = &mut s; 
  slice3.push('!');
  assert_eq!(slice3, "hello, world!");

  println!("Success!");
}

