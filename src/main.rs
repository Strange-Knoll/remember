use clap::{error::ContextValue, Command, *};
use std::{
    borrow::BorrowMut,
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
    //let args: Vec<String> = env::args().collect();
    //_old(args);

    let cmd = Command::new("reminder").args([
        arg!(--list "list reminders"),
        arg!(--new <note> "create reminder"),
        arg!(--remove <num> "remove reminder"),
    ]);

    let matches = cmd.get_matches();

    for m in matches.get_many::<bool>("list").unwrap() {
        //println!("Matched ls");
        if m == &true {
            list()
        }
    }

    for mut m in matches.get_many::<String>("new") {
        //println!("Matched n");
        let c = m.next().unwrap();
        new(&mut c.to_string());
        list();
    }
    for m in matches.get_one::<String>("remove") {
        //println!("Matched rm");
        let m = m.parse::<usize>().unwrap();
        rm(&m);
        list();
    }

    //pretty_print(&["Hello", "world!"], &[Color::White, Color::Magenta]);
}

fn list() {
    let mut content = String::new();
    let mut file = File::open("./example.txt").unwrap();
    file.read_to_string(&mut content);
    print_line_number(content);
}

fn new(s: &mut String) {
    let mut file = OpenOptions::new().append(true).open("example.txt").unwrap();

    s.push_str("\n");
    file.write_all(s.as_bytes());
    list();
}

fn rm(index: &usize) {
    let mut content = String::new();
    let mut file = File::open("example.txt").unwrap();
    file.read_to_string(&mut content);
    let mut split: Vec<&str> = content.split("\n").collect();
    split.remove(*index);
    //println!("{}", split);
    let mut file = File::create("example.txt").unwrap();
    file.write_all(split.join("\n").as_bytes());
}

fn print_line_number(s: String) {
    let split: Vec<&str> = s.split('\n').collect();

    for (i, s) in split.iter().enumerate() {
        pretty_print_ln(
            &[format!("{}:", i).as_str(), &s],
            &[Color::Green, Color::White],
        )
    }
}

fn pretty_print_ln(strings: &[&str], colors: &[Color]) {
    pretty_print(strings, colors);
    println!("\n");
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
