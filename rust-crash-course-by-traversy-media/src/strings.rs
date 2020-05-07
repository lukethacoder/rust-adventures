// Primative str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data.

pub fn run() {
  // Immutable str
  // let helloImut = "Hello";

  // Growable
  let mut hello = String::from("Hello ");
  println!("Length: {}", hello.len());

  // Push char
  hello.push('\u{1F525}');

  // Push string
  hello.push_str(" longer string");

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Check if empty
  println!("is empty: {}", hello.is_empty());

  // Contains
  println!("does contain: {}", hello.contains("longer"));

  // Replace
  println!("Put out the fire {}", hello.replace("\u{1F525}", "\u{1F9EF}"));

  // println!("{}", hello);

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('h');
  s.push('i');
  println!("{}", s);
  println!("{}", s.len());

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  // Unicode char capacity
  let mut emojis = String::with_capacity(10);
  emojis.push('\u{1F34E}');
  emojis.push('\u{1F34C}');
  println!("{}", emojis);
  println!("{}", emojis.len());
  assert_eq!(8, emojis.len());

}