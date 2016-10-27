use std::fmt;

pub type CoordType = f32;
pub type FlagType = i32;

pub enum CommandParam {
    AbsoluteX(CoordType),
    AbsoluteY(CoordType),
    RelativeX(CoordType),
    RelativeY(CoordType),
    Flag(FlagType),
}

pub struct Command {
    pub name: Option<char>,
    pub params: Vec<CommandParam>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.name {
            try!(write!(f, "{}", name));
        } else {
            try!(write!(f, " "));
        }
        for param in &self.params {
            match *param {
                CommandParam::AbsoluteX(value) |
                CommandParam::AbsoluteY(value) |
                CommandParam::RelativeX(value) |
                CommandParam::RelativeY(value) => {
                    try!(write!(f, " {}", value));
                }
                CommandParam::Flag(value) => {
                    try!(write!(f, " {}", value));
                }
            }
        }
        Ok(())
    }
}
