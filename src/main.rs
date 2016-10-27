#[macro_use]
extern crate lazy_static;
extern crate leak;
extern crate regex;
extern crate rustyline;

mod command;
mod parser;
mod transform;

use regex::Regex;
use std::io;
use std::io::Read;

use command::Command;

lazy_static! {
    static ref SCALE_ARG: Regex = Regex::new(r"^x([\d\.]+)$").unwrap();
    static ref TRANSLATE_ARG: Regex = Regex::new(r"^([+-][\d\.]+)([+-][\d\.]+)$").unwrap();
}

fn handle_operation(commands: &mut [Command], op: &str) -> Result<(), &'static str> {
    macro_rules! try_parse {
        ($value:expr, $msg:expr) => {
            try!($value.parse().map_err(|_| $msg))
        }
    }
    if let Some(captures) = SCALE_ARG.captures(op) {
        let factor = try_parse!(captures[1], "invalid scale factor");
        transform::scale(commands, factor);
    } else if let Some(captures) = TRANSLATE_ARG.captures(op) {
        let offset_x = try_parse!(captures[1], "invalid x offset");
        let offset_y = try_parse!(captures[2], "invalid y offset");
        transform::translate(commands, offset_x, offset_y);
    } else {
        return Err("unknown operation");
    }
    Ok(())
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut commands: Vec<_> = parser::parse_commands(&buffer).collect();
    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(op) = rl.readline("> ") {
        match handle_operation(&mut commands, &op) {
            Ok(_) => {
                for command in commands.iter() {
                    println!("{}", command);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
