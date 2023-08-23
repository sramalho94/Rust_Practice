fn main(){
  // FILL in the blank

  let b = false;

  let _v = match b {
    true => 1,
    // Diverging functions can also be used in match expression to replace a val
    false => {
      println!("Success!");
      panic!("We have no value for `false` but we can panic"); 
    }
  };

  println!("Exercise Failed if printing out this line!");
}