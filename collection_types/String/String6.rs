/* A String is made up of three components: a pointer to some bytes, a length, and a capacity.

The pointer points to an internal buffer String uses to store its data. The length is the number of bytes currently stored in the buffer( always stored on the heap ), and the capacity is the size of the buffer in bytes. As such, the length will always be less than or equal to the capacity. */

// Modify the code below to print out: 
// 25
// 25
// 25
// Here, there’s no need to allocate more memory inside the loop.
fn main() {
  let mut s: String = String::new();

  println!("{}", s.capacity());

  for _ in 0..2 {
      s.push_str("hello");
      println!("{}", s.capacity());
  }

  println!("Success!");
}
