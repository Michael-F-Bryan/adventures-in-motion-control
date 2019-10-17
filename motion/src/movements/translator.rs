use super::Point;
use core::time::Duration;
use gcode::{self, GCode, Mnemonic};
use uom::si::{
    f32::{Length, Time, Velocity},
    length::{inch, millimeter},
    time::minute,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Translator {
    current_location: Point,
    coordinate_mode: CoordinateMode,
    units: Units,
    feed_rate: Velocity,
}

impl Translator {
    pub fn translate<C: Callbacks>(&mut self, command: &GCode, mut cb: C) {
        match command.mnemonic() {
            Mnemonic::Miscellaneous => self.handle_miscellaneous(command, cb),
            Mnemonic::General => self.handle_general(command, cb),
            _ => cb.unsupported_command(command),
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

    fn handle_miscellaneous<C: Callbacks>(
        &mut self,
        command: &GCode,
        mut cb: C,
    ) {
        match command.major_number() {
            30 => cb.end_of_program(),
            _ => cb.unsupported_command(command),
        }
    }

    fn handle_general<C: Callbacks>(&mut self, command: &GCode, mut cb: C) {
        match command.major_number() {
            0 | 1 => {
                let end = self.calculate_end(command);
                let feed_rate = self.calculate_feed_rate(command);
                cb.linear_interpolate(self.current_location, end, feed_rate);

                self.current_location = end;
                self.feed_rate = feed_rate;
            },

            2 | 3 => {
                let end = self.calculate_end(command);
                let start = self.current_location;
                let feed_rate = self.calculate_feed_rate(command);
                let direction = if command.major_number() == 2 {
                    Direction::Clockwise
                } else {
                    Direction::Anticlockwise
                };

                match self.get_centre(command) {
                    Ok(centre) => cb.circular_interpolate(
                        start, centre, end, direction, feed_rate,
                    ),
                    Err(arg) => cb.invalid_argument(command, arg, "Missing"),
                }

                self.feed_rate = feed_rate;
                self.current_location = end;
            },

            4 => match command.value_for('P') {
                Some(dwell_time) => {
                    cb.dwell(Duration::from_secs_f32(dwell_time))
                },
                None => {
                    cb.invalid_argument(command, 'P', "Dwell time not provided")
                },
            },

            20 => self.units = Units::Inches,
            21 => self.units = Units::Millimetres,
            90 => self.coordinate_mode = CoordinateMode::Absolute,
            91 => self.coordinate_mode = CoordinateMode::Relative,
            _ => cb.unsupported_command(command),
        }
    }

    /// Gets the centre of a circular interpolate move (G02, G03), bailing out
    /// if the centre coordinates aren't provided.
    fn get_centre(&self, command: &GCode) -> Result<Point, char> {
        let x = command.value_for('I').ok_or('I')?;
        let y = command.value_for('J').ok_or('J')?;

        // TODO: Take the plane into account (G17, G18, G19)
        Ok(Point {
            x: self.to_length(x),
            y: self.to_length(y),
            z: self.current_location.z,
        })
    }

    fn calculate_feed_rate(&self, command: &GCode) -> Velocity {
        let raw = match command.value_for('F') {
            Some(f) => f,
            None => return self.feed_rate,
        };

        let time = Time::new::<minute>(1.0);

        match self.units {
            Units::Inches => Length::new::<inch>(raw) / time,
            Units::Millimetres => Length::new::<millimeter>(raw) / time,
        }
    }

    fn calculate_end(&self, command: &GCode) -> Point {
        let x = command.value_for('X').unwrap_or(0.0);
        let y = command.value_for('Y').unwrap_or(0.0);
        let z = command.value_for('Z').unwrap_or(0.0);
        self.absolute_location(x, y, z)
    }

    fn to_length(&self, raw: f32) -> Length {
        match self.units {
            Units::Millimetres => Length::new::<millimeter>(raw),
            Units::Inches => Length::new::<inch>(raw),
        }
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Clockwise,
    Anticlockwise,
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
    fn linear_interpolate(
        &mut self,
        _start: Point,
        _end: Point,
        _feed_rate: Velocity,
    ) {
    }
    fn circular_interpolate(
        &mut self,
        _start: Point,
        _centre: Point,
        _end: Point,
        _direction: Direction,
        _feed_rate: Velocity,
    ) {
    }
    fn dwell(&mut self, _period: Duration) {}
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

    fn linear_interpolate(
        &mut self,
        start: Point,
        end: Point,
        feed_rate: Velocity,
    ) {
        (**self).linear_interpolate(start, end, feed_rate);
    }

    fn dwell(&mut self, period: Duration) { (**self).dwell(period); }

    fn circular_interpolate(
        &mut self,
        start: Point,
        centre: Point,
        end: Point,
        direction: Direction,
        feed_rate: Velocity,
    ) {
        (**self).circular_interpolate(start, centre, end, direction, feed_rate);
    }
}
