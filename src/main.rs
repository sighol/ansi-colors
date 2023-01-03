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
    print!("\x1b[48;2;103;234;107m rgb: 38;2;103;234;107\x1b[0m");
    println!();


    println!("\n\x1b[1;4;44mText styles \x1b[3m(1;4;44):\x1b[0m");
    println!("  bold:      \x1b[1m 1 \x1b[0m");
    println!("  dim:       \x1b[2m 2 \x1b[0m");
    println!("  italic:    \x1b[3m 3 \x1b[0m");
    println!("  underline: \x1b[4m 4 \x1b[0m");
    println!("  blink:     \x1b[5m 5 \x1b[0m");
    println!("  overline:  \x1b[6m 6 \x1b[0m");
    println!("  invert:    \x1b[7m 7 \x1b[0m");
    println!("  hidden:    \x1b[8m 8 \x1b[0m");
    println!("  strike:    \x1b[9m 9 \x1b[0m");

    println!("\n\x1b[1;4;101mTerminal styles: (1;4;101)\x1b[0m");
    println!("  bash:   echo -e \x1b[33m'\\e[37;43m test \\e[0m'\x1b[m");
    println!("          echo -e \x1b[33m\"\\033[46m Hello \\033[1m\"\x1b[m");
    println!("  rust:   println!(\x1b[33m\"\\x1b[37;43m test \\x1b[0m\")\x1b[m");
    println!("  rust:   println!(\x1b[33m\"\\u{{001b}}[37;43m test \\u{{001b}}[0m\")\x1b[m");
    println!("  kotlin: println(\x1b[33m\"\\u001b[37;43m test \\u001b[0m\")\x1b[m");
    println!("  python: print(\x1b[33m\"\\x1b[1;37;46m Hello \\x1b[0m\")\x1b[m");
    println!("          print(\x1b[33m\"\\033[1;37;46m Hello \\033[0m\")\x1b[m");
    println!("          print(\x1b[33m\"\\N{{ESC}}[1;37;46m Hello \\033[0m\")\x1b[m");
}

