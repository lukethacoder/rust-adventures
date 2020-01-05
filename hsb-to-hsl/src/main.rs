use std::io;

fn main() {
    let mut hsb_h = String::new(); // make a mutable string variable
    println!("Enter 'h' value");
    io::stdin().read_line(&mut hsb_h); //to get input from the user
    let hsb_h: i64 = hsb_h.trim().parse().unwrap(); // performing shadowing to convert the type

    let mut hsb_s = String::new(); // make a mutable string variable
    println!("Enter 's' value");
    io::stdin().read_line(&mut hsb_s); //to get input from the user
    let hsb_s: i64 = hsb_s.trim().parse().unwrap(); // performing shadowing to convert the type
    let mut hsb_b = String::new(); // make a mutable string variable
    println!("Enter 'b' value");
    io::stdin().read_line(&mut hsb_b); //to get input from the user
    let hsb_b: i64 = hsb_b.trim().parse().unwrap(); // performing shadowing to convert the type

    let string_this: String = hsbToHsl(hsb_h, hsb_s, hsb_b);

    println!("value {}", string_this);
}

fn hsbToHsl(hsb_h: i64, hsb_s: i64, hsb_b: i64) -> String {
    let hsl_lightness: i64;
    let hsl_saturation: i64;

    let mut string_to_return: String;

    if hsb_b == 0 {
        string_to_return = String::from("hsl(0, 0, 0)");
    } else {
        hsl_lightness = (hsb_b / 2) * (2 - (hsb_s / 100));
        let divide_by_this: i64;

        if hsl_lightness < 50 {
            divide_by_this = hsl_lightness * 2
        } else {
            divide_by_this = (200 - hsl_lightness) * 2
        }

        hsl_saturation = (hsb_b * hsb_s) / divide_by_this;
        string_to_return = String::from(format!(
            "hsl( {}, {}, {} )",
            hsb_h, hsl_saturation, hsl_lightness
        ));
    }

    return string_to_return;
}
