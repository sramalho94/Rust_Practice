// A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen.

// Fill in the blank to make the code work, `split` MUST be used
fn main() {
  let num: Option<i32> = Some(4);
  let split: i32 = 5;
  match num {
      Some(x) if x < split => assert!(x < split),
      Some(x) => assert!(x >= split),
      None => (),
  }

  println!("Success!");
}
