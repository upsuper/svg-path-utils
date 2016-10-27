use command::{Command, CommandParam, CoordType};

macro_rules! handle_params {
    ($command:ident {
        $( ref mut $value:ident @ ( $( $tag:ident ) |* ) => $transform:block )*
    }) => {
        for param in $command.params.iter_mut() {
            match *param {
                $(
                    $( CommandParam::$tag(ref mut $value) => $transform )*
                )*
                _ => {}
            };
        }
    }
}

pub fn scale(commands: &mut [Command], factor: CoordType) {
    for command in commands {
        handle_params!(command {
            ref mut value @ (AbsoluteX | AbsoluteY |
                             RelativeX | RelativeY) => {
                *value *= factor;
            }
        })
    }
}

pub fn translate(commands: &mut [Command], offset_x: CoordType, offset_y: CoordType) {
    for command in commands {
        handle_params!(command {
            ref mut value @ (AbsoluteX) => { *value += offset_x; }
            ref mut value @ (AbsoluteY) => { *value += offset_y; }
        })
    }
}
