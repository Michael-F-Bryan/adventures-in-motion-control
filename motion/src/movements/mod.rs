use gcode::{self, GCode};

#[derive(Debug, Clone, PartialEq)]
pub struct Translator {}

impl Translator {
    pub fn translate<C: Callbacks>(&mut self, _command: &GCode, _cb: C) {
        unimplemented!()
    }

    pub fn translate_src<C, G>(
        &mut self,
        src: &str,
        cb: &mut C,
        parse_errors: &mut G,
    ) where
        C: Callbacks + ?Sized,
        G: gcode::Callbacks + ?Sized,
    {
        for line in gcode::parse_with_callbacks(src, parse_errors) {
            for command in line.gcodes() {
                self.translate(&command, &mut *cb);
            }
        }
    }
}

pub trait Callbacks {
    fn unsupported_command(&mut self, _command: &GCode) {}
    fn invalid_argument(
        &mut self,
        _command: &GCode,
        _arg: char,
        _reason: &'static str,
    ) {
    }
}

impl<'a, C: Callbacks + ?Sized> Callbacks for &'a mut C {
    fn unsupported_command(&mut self, command: &GCode) {
        (**self).unsupported_command(command);
    }

    fn invalid_argument(
        &mut self,
        command: &GCode,
        arg: char,
        reason: &'static str,
    ) {
        (**self).invalid_argument(command, arg, reason);
    }
}
