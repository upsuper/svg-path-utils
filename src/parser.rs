use leak::Leak;
use regex::{Regex, FindMatches};
use std::str::FromStr;

use command::{Command, CommandParam};

struct CommandFormats {
    names: Vec<char>,
    params: Vec<&'static str>,
}

impl CommandFormats {
    fn get_format_for(&self, name: char) -> Option<&'static str> {
        self.names.iter().zip(self.params.iter())
            .find(|&(&n, _)| n == name).map(|(_, &param)| param)
    }
}

lazy_static! {
    static ref TOKEN_PATTERN: Regex =
        Regex::new(r"[a-zA-Z]|-?\.\d+|-?\d+(?:\.\d*)?").unwrap();

    static ref CMD_FORMAT: CommandFormats = {
        let table = [
            ('M', "XY"),
            ('Z', ""),
            ('L', "XY"),
            ('H', "X"),
            ('V', "Y"),
            ('C', "XYXYXY"),
            ('S', "XYXY"),
            ('Q', "XYXY"),
            ('T', "XY"),
            ('A', "xynnnXY"),
        ];
        let size = table.len() * 2;
        let mut names = Vec::with_capacity(size);
        let mut params = Vec::with_capacity(size);
        for &(name, param) in table.into_iter() {
            names.push(name);
            params.push(param);
            names.push(name.to_lowercase().next().unwrap());
            params.push(param.to_lowercase().leak::<'static>());
        }
        CommandFormats { names: names, params: params }
    };
}

struct Tokens<'c> {
    content: &'c str,
    iter: FindMatches<'static, 'c>,
}

impl<'c> Iterator for Tokens<'c> {
    type Item = &'c str;
    fn next(&mut self) -> Option<&'c str> {
        self.iter.next().map(|(start, end)| &self.content[start..end])
    }
}

pub struct Commands<'c> {
    iter: Tokens<'c>,
    last_name: Option<char>,
}

impl<'c> Iterator for Commands<'c> {
    type Item = Command;
    fn next(&mut self) -> Option<Command> {
        let first = match self.iter.next() {
            Some(token) => token,
            None => { return None; }
        };
        // Check whether the new token is a command name.
        let first_char = first.chars().next().unwrap();
        let (name, is_new_name) = if first_char.is_alphabetic() {
            assert!(first.chars().count() == 1, "invalid command name");
            self.last_name = Some(first_char);
            (first_char, true)
        } else {
            (self.last_name.expect("no reusable command name"), false)
        };
        // Get the format for the given command.
        let format = CMD_FORMAT.get_format_for(name).expect("invalid command name");
        if format.len() == 0 {
            assert!(is_new_name, "command with zero param is not reusable");
            self.last_name = None;
            return Some(Command { name: Some(name), params: vec![] });
        }
        // Create iterator for params.
        let first_param = if is_new_name { self.iter.next() } else { Some(first) };
        let param_iter = first_param.into_iter().chain(&mut self.iter);
        // Parse parameters.
        let params = format.chars().zip(param_iter).map(|(format_char, param)| {
            macro_rules! match_format {
                ($var:ident, {
                    $( $char:expr => $tag:ident, )*
                }) => {
                    match $var {
                        $(
                            $char => {
                                let value = FromStr::from_str(param);
                                CommandParam::$tag(value.expect("invalid param"))
                            }
                        )*
                        _ => panic!("unexpected param")
                    }
                }
            }
            match_format!(format_char, {
                'X' => AbsoluteX,
                'Y' => AbsoluteY,
                'x' => RelativeX,
                'y' => RelativeY,
                'n' => Flag,
            })
        }).collect::<Vec<_>>();
        assert!(params.len() == format.len(), "param list ends unexpectedly");
        // Return the command.
        Some(Command {
            name: if is_new_name { Some(name) } else { None },
            params: params,
        })
    }
}

pub fn parse_commands<'c>(content: &'c str) -> Commands<'c> {
    Commands {
        iter: Tokens {
            content: content,
            iter: TOKEN_PATTERN.find_iter(content),
        },
        last_name: None,
    }
}
