// Solve it in two ways
// DON'T let `println!` work

fn main(){
  never_return();

  println!("Failed!");
}
  fn never_return() -> ! {
    // Implement this fucntion, don't modify the fn signatures
    panic!()
  }
