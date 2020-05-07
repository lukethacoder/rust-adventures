// Tuples group together values of different types
// Max 12 elements - to be rendered in a println!()

pub fn run() {
  let person: (&str, &str, i8) = ("luke", "aus", 13);

  println!("{} is from {} and is {}", person.0, person.1, person.2)
}