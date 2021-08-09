use regex::Regex;
use std::error::Error;
use std::fs;
use std::process;

// const ARITHMETIC_COMMANDS =
const ARITHMETIC_COMMANDS: &'static [&'static str] =
    &["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];

#[derive(Debug, Clone, Copy)]
enum Command<'a> {
    Arithmetic(&'a str),
    Push(&'a str),
    Pop(&'a str),
    // Label(&'a str),
    // Goto(&'a str),
    // If(&'a str),
    // Function(&'a str),
    // Return(&'a str),
    // Call(&'a str),
}

impl Command<'_> {
    // fn arg_one(&self) -> Option<&str> {

    //     // match self {
    //     //     Command::A(_val) => None,
    //     //     Command::L(_val) => None,
    //     //     Command::C(val) => {
    //     //         let idx = val.find('=');
    //     //         match idx {
    //     //             Some(idx) => {
    //     //                 let comp = &val[0..idx];
    //     //                 Some(comp)
    //     //             }
    //     //             None => None,
    //     //         }
    //     //     }
    //     // }
    // }

    // fn arg_two(&self) -> Option<&str> {

    //     // match self {
    //     //     Command::A(_val) => None,
    //     //     Command::L(_val) => None,
    //     //     Command::C(val) => {
    //     //         let idx = val.find('=');
    //     //         match idx {
    //     //             Some(idx) => {
    //     //                 let comp = &val[0..idx];
    //     //                 Some(comp)
    //     //             }
    //     //             None => None,
    //     //         }
    //     //     }
    //     // }
    // }
}

// Arithmetic(&'a str),
// Push(&'a str),
// Pop(&'a str),
// Label(&'a str),
// Goto(&'a str),
// If(&'a str),
// Function(&'a str),
// Return(&'a str),
// Call(&'a str),
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn get_command_type(command: &str) -> Option<Command> {
    let word = first_word(command);

    let is_arithmetic = ARITHMETIC_COMMANDS.contains(&word);
    let is_push = word == "push";
    let is_pop = word == "pop";

    if is_arithmetic {
        Some(Command::Arithmetic(command))
    } else if is_push {
        Some(Command::Push(command))
    } else if is_pop {
        Some(Command::Pop(command))
    } else {
        None
    }
}

pub fn parse_args(args: &[String]) -> Result<String, &str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    let filename = args[1].clone();

    Ok(filename)
}

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let filename = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(&filename).expect("no such file");

    // remove comments and whitespace, may still contain newlines
    let regex = Regex::new("\n{2,}|//.*").unwrap();
    let contents = regex.replace_all(&contents, "").into_owned();

    let commands = contents
        .lines()
        .filter_map(|line| get_command_type(line))
        .collect::<Vec<Command>>();

    println!("commands: {:?}", commands);
    Ok(())
}
