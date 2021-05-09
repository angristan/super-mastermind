use ansi_term::Colour;
use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
};

#[derive(Debug, Copy, Clone)]
enum Color {
    Green,
    Red,
    Blue,
    Yellow,
    Purple,
    Cyan,
    Orange,
    White,
}

fn main() {
    let solution: Vec<Color> = vec![
        Color::Green,
        Color::Red,
        Color::Blue,
        Color::Purple,
        Color::Cyan,
    ];

    for line in io::stdin().lock().lines() {
        let guess = parse_colors(line.unwrap());
        match guess {
            Ok(g) => fancy_print_guess(&g),
            Err(e) => eprintln!("{}", e),
        }

        io::stdout().flush().unwrap();
    }
}

fn fancy_print_guess(guess: &[Color]) {
    for c in guess.iter() {
        match c {
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Blue => print!("{}", Colour::Blue.paint("B")),
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
            Color::Orange => print!("{}", Colour::RGB(255, 165, 0).paint("O")),
            Color::White => print!("{}", Colour::White.paint("W")),
        }
    }
    println!();
}

fn parse_colors(raw_colors: String) -> Result<Vec<Color>, String> {
    let char_vec: Vec<char> = raw_colors.chars().collect();
    let mut guess: Vec<Color> = vec![];

    let mut colors_map: HashMap<char, Color> = HashMap::new();

    colors_map.insert('R', Color::Red);
    colors_map.insert('G', Color::Green);
    colors_map.insert('B', Color::Blue);
    colors_map.insert('Y', Color::Yellow);
    colors_map.insert('P', Color::Purple);
    colors_map.insert('C', Color::Cyan);
    colors_map.insert('O', Color::Orange);
    colors_map.insert('W', Color::White);

    for color in char_vec.iter() {
        if !colors_map.contains_key(color) {
            return Err(format!("Invalid color: {}", color).to_string());
        }
        guess.push(colors_map[color]);
    }

    return Ok(guess);
}
