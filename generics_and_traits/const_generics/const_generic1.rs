// <T, const N: usize> is part of the struct type, it means Array<i32, 3> and Array<i32, 4> are different types.

struct Array<T, const N: usize> {
  data : [T; N]
}

fn main() {
  let arrays: [Array<i32, 3>; 3] = [
      Array{
          data: [1, 2, 3],
      },
      Array {
          data: [1, 2, 3],
      },
      Array {
          data: [4, 5, 6]
      }
  ];

  let floats: [Array<f64, 2>; 3] = [
    Array { data: [1.0, 2.0]},
    Array { data: [3.0, 4.0]},
    Array { data: [5.0, 6.0]},
  ];

  println!("Success!");
}
