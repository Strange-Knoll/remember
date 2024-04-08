use clap::*;
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{stdout, Read, Seek, SeekFrom, Write},
    usize,
};

use crossterm::{
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "-n" | "--new" => {
            assert!(args.len() > 2);
            let content = &args[2];
            println!("content: {}", content);
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("example.txt")
                .unwrap();

            //file.seek(From::End(0)).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            file.write_all("\n".as_bytes()).unwrap();
            let mut content = String::new();

            let mut file = File::open("./example.txt").unwrap();
            file.read_to_string(&mut content);
            print_line_number(content);
        }
        "-ls" | "--list" => {
            let mut content = String::new();
            let mut file = File::open("./example.txt").unwrap();
            file.read_to_string(&mut content).unwrap();
            print_line_number(content);
        }
        "-rm" | "--remove" => {
            assert!(args.len() > 2);
            let line: usize = args[2].parse::<usize>().unwrap();

            let mut content = String::new();
            let mut file = File::open("./example.txt").unwrap();
            file.read_to_string(&mut content).unwrap();
            let mut split: Vec<&str> = content.split('\n').collect();
            split.remove(line);
            let mut file = File::create("./example.txt").unwrap();
            file.write_all(split.join("\n").as_bytes()).unwrap();

            let mut file = File::open("./example.txt").unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content);
            print_line_number(content);
        }
        "-h" | "--help" => {}
        _ => {}
    }

    //pretty_print(&["Hello", "world!"], &[Color::White, Color::Magenta]);
}

fn print_line_number(s: String) {
    let split: Vec<&str> = s.split('\n').collect();

    for (i, s) in split.iter().enumerate() {
        pretty_print(
            &[format!("{}: ", i).as_str(), &s, "\n"],
            &[Color::Green, Color::White, Color::White],
        )
    }
}

fn pretty_print(strings: &[&str], colors: &[Color]) {
    assert!(strings.len() == colors.len());

    for (i, s) in strings.iter().enumerate() {
        let s = *s;
        let c = colors[i];
        let _ = execute!(
            stdout(),
            SetForegroundColor(c),
            SetBackgroundColor(Color::Reset),
            Print(s),
            Print(" "),
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::Reset)
        );
    }
}
