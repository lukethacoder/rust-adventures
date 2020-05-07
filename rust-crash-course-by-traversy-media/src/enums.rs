// Enums are types which have a few definite values

enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right
}

fn move_avatar(m: Movement) {
  // Perform action depending on info
  // match works similarly to switch
  match m {
    Movement::Up => println!("Moving up"),
    Movement::Down => println!("Moving down"),
    Movement::Left => println!("Moving left"),
    Movement::Right => println!("Moving right")
  }
}

pub fn run() {
  let avatar1 = Movement::Left;
  let avatar2 = Movement::Up;
  let avatar3 = Movement::Right;
  let avatar4 = Movement::Down;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}