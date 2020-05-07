// Vectors - resizeable arrays
use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 20;

  // Add on to Vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // Get single val
  println!("Single val: {}", numbers[0]);

  // Get Vector length
  println!("Vector length {}", numbers.len());

  // Vectors are stack allocated
  // println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
  // if we import with `use std::mem;` we can shorthand the above
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate values (similar to .map())
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("Numbers Vec: {:?}", numbers);

}