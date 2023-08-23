fn main(){
  println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
  match tp {
    1 => {
      // TO DO
    }
    _ => {
      // TO DO
    }
  };

  // Rather than returning a None, we use a diverging function instead

  never_return_fn()
}

// IMPLEMENT this function in THREE WAYS

fn never_return_fn() -> ! {
  // panic!()
  // unimplemented!()
  todo!()
}