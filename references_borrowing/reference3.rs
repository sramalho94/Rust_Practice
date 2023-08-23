// Fix the error

fn main(){
  let mut s = String::from("hello, ");

  borrow_objects(&s);

  println!("Success!");
}

fn borrow_objects(s: &String){}