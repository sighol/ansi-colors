fn main() {
    print!("Text colors: ");
    for i in 30..=37 {
        print!("\x1b[{i}m {i} \x1b[0m");
    }
    for i in 90..=97 {
        print!("\x1b[{i}m  {i} \x1b[0m");
    }
    print!("\x1b[38;2;103;234;107m rgb: 38;2;103;234;107\x1b[0m");
    println!();

    print!("Back colors: ");
    for i in 40..=47 {
        print!("\x1b[{i}m {i} \x1b[0m");
    }
    for i in 100..=107 {
        print!("\x1b[{i}m {i} \x1b[0m");
    }
    print!("\x1b[48;2;103;234;107m rgb: 48;2;103;234;107\x1b[0m");
    println!();

    println!("\n\x1b[1;4;44mText styles:\x1b[0m");
    println!("  bold:      \x1b[1m 1 \x1b[0m");
    println!("  dim:       \x1b[2m 2 \x1b[0m");
    println!("  italic:    \x1b[3m 3 \x1b[0m");
    println!("  underline: \x1b[4m 4 \x1b[0m");
    println!("  blink:     \x1b[5m 5 \x1b[0m");
    println!("  overline:  \x1b[6m 6 \x1b[0m");
    println!("  invert:    \x1b[7m 7 \x1b[0m");
    println!("  hidden:    \x1b[8m 8 \x1b[0m");
    println!("  strike:    \x1b[9m 9 \x1b[0m");

    println!("\n\x1b[1;4;101mLanguages:\x1b[0m");
    println!(
        "  bash:   {}, {}, {}. Use \x1b[33mecho -e\x1b[0m or \x1b[33mprintf\x1b[0m",
        c("\\x1b", "33"),
        c("\\033", "33"),
        c("\\e", "33")
    );
    println!(
        "  python: {}, {}, {}.",
        c("\\x1b", "33"),
        c("\\033", "33"),
        c("\\N{ESC}", "33")
    );
    println!("  rust:   {}, {}.", c("\\x1b", "33"), c("\\u{001b}", "33"));
    println!("  kotlin: {}.", c("\\u001b", "33"));

    // println!("\n\x1b[1;4;105m Examples:\x1b[0m");
    // println!("  - {}", c("1;4;44", "1;4;44"));
    // println!("  - {}", c("1;105", "1;105"));
    // println!("  - {}", c("1;46", "1;46"));
    // println!("  - {}", c("1;96", "1;96"));

}

fn c(input: &str, colors: &str) -> String {
    let mut s = String::new();
    s.push_str("\x1b[");
    s.push_str(colors);
    s.push_str("m");
    s.push_str(input);
    s.push_str("\x1b[0m");
    s
}
