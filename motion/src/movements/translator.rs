use super::Point;
use gcode::{self, GCode, Mnemonic};
use uom::si::{
    f32::Length,
    length::{inch, millimeter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Translator {
    current_location: Point,
    coordinate_mode: CoordinateMode,
    units: Units,
}

impl Translator {
    pub fn translate<C: Callbacks>(&mut self, command: &GCode, mut cb: C) {
        match command.mnemonic() {
            Mnemonic::Miscellaneous => self.handle_miscellaneous(command, cb),
            Mnemonic::General => self.handle_general(command, cb),
            _ => cb.unsupported_command(command),
        }
    }

    fn handle_general<C: Callbacks>(&mut self, command: &GCode, mut cb: C) {
        match command.major_number() {
            0 | 1 => {
                let end = self.calculate_end(command);
                cb.linear_interpolate(self.current_location, end);
                self.current_location = end;
            },
            _ => cb.unsupported_command(command),
        }
    }

    fn handle_miscellaneous<C: Callbacks>(
        &mut self,
        command: &GCode,
        mut cb: C,
    ) {
        match command.major_number() {
            30 => unimplemented!(),
            _ => cb.unsupported_command(command),
        }
    }

    fn calculate_end(&self, command: &GCode) -> Point {
        let x = command.value_for('x').unwrap_or(0.0);
        let y = command.value_for('y').unwrap_or(0.0);
        let z = command.value_for('z').unwrap_or(0.0);
        self.absolute_location(x, y, z)
    }

    fn absolute_location(&self, x: f32, y: f32, z: f32) -> Point {
        let raw = match self.units {
            Units::Millimetres => Point::new::<millimeter>(x, y, z),
            Units::Inches => Point::new::<inch>(x, y, z),
        };

        match self.coordinate_mode {
            CoordinateMode::Absolute => raw,
            CoordinateMode::Relative => raw + self.current_location,
        }
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum CoordinateMode {
    Absolute,
    Relative,
}

impl Default for CoordinateMode {
    fn default() -> CoordinateMode { CoordinateMode::Absolute }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Units {
    Millimetres,
    Inches,
}

impl Default for Units {
    fn default() -> Units { Units::Millimetres }
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

    fn end_of_program(&mut self) {}
    fn linear_interpolate(&mut self, _start: Point, _end: Point) {}
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

    fn end_of_program(&mut self) { (**self).end_of_program(); }

    fn linear_interpolate(&mut self, start: Point, end: Point) {
        (**self).linear_interpolate(start, end);
    }
}
