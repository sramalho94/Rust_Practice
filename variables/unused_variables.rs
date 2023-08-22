// Fix the warning below with
// * only one solution
//  ** two distinct solutions
// Note: none of the solutions is to remove the line let x= 1


fn main(){
  let x: i32 = 1;

  println!("value of x is {}", x);
  assert_eq!(x, 1);
}
