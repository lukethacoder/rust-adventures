// Arrays - Fixed list where elements are the same data type
use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // Get single val
  println!("Single val: {}", numbers[0]);

  // Get array length
  println!("array length {}", numbers.len());

  // Arrays are stack allocated
  // println!("array occupies {} bytes", std::mem::size_of_val(&numbers));
  // if we import with `use std::mem;` we can shorthand the above
  println!("array occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("slice: {:?}", slice);


}