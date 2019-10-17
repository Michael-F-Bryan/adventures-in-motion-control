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
            0 | 1 => self.handle_linear_interpolate(command, cb),
            2 | 3 => self.handle_circular_interpolate(command, cb),
            4 => self.handle_dwell(command, cb),

            20 => self.units = Units::Inches,
            21 => self.units = Units::Millimetres,
            90 => self.coordinate_mode = CoordinateMode::Absolute,
            91 => self.coordinate_mode = CoordinateMode::Relative,
            _ => cb.unsupported_command(command),
        }
    }

    fn handle_dwell<C: Callbacks>(&mut self, command: &GCode, mut cb: C) {
        match command.value_for('P') {
            Some(dwell_time) => cb.dwell(Duration::from_secs_f32(dwell_time)),
            None => {
                cb.invalid_argument(command, 'P', "Dwell time not provided")
            },
        }
    }

    fn handle_linear_interpolate<C: Callbacks>(
        &mut self,
        command: &GCode,
        mut cb: C,
    ) {
        let end = self.calculate_end(command);
        let feed_rate = self.calculate_feed_rate(command);
        cb.linear_interpolate(self.current_location, end, feed_rate);

        self.current_location = end;
        self.feed_rate = feed_rate;
    }

    fn handle_circular_interpolate<C: Callbacks>(
        &mut self,
        command: &GCode,
        mut cb: C,
    ) {
        let end = self.calculate_end(command);
        let start = self.current_location;
        let feed_rate = self.calculate_feed_rate(command);
        let direction = if command.major_number() == 2 {
            Direction::Clockwise
        } else {
            Direction::Anticlockwise
        };

        match self.get_centre(command) {
            Ok(centre) => {
                cb.circular_interpolate(
                    start, centre, end, direction, feed_rate,
                );

                self.feed_rate = feed_rate;
                self.current_location = end;
            },
            Err(arg) => cb.invalid_argument(command, arg, "Missing"),
        }
    }

    /// Gets the centre of a circular interpolate move (G02, G03), bailing out
    /// if the centre coordinates aren't provided.
    fn get_centre(&self, command: &GCode) -> Result<Point, char> {
        let x = command.value_for('I').ok_or('I')?;
        let y = command.value_for('J').ok_or('J')?;

        // TODO: Take the plane into account (G17, G18, G19)
        Ok(Point {
            z: self.current_location.z,
            ..self.absolute_location(x, y, 0.0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;
    use uom::si::velocity::millimeter_per_minute;

    #[derive(Debug, Default)]
    struct MockCallbacks {
        linear: Vec<(Point, Point, Velocity)>,
        circular: Vec<(Point, Point, Point, Direction, Velocity)>,
        dwell: Vec<Duration>,
        unknown_commands: u32,
        invalid: Vec<char>,
    }

    impl Callbacks for MockCallbacks {
        fn linear_interpolate(
            &mut self,
            start: Point,
            end: Point,
            feed_rate: Velocity,
        ) {
            self.linear.push((start, end, feed_rate));
        }

        fn circular_interpolate(
            &mut self,
            start: Point,
            centre: Point,
            end: Point,
            direction: Direction,
            feed_rate: Velocity,
        ) {
            self.circular
                .push((start, centre, end, direction, feed_rate));
        }

        fn dwell(&mut self, duration: Duration) { self.dwell.push(duration); }

        fn unsupported_command(&mut self, _command: &GCode) {
            self.unknown_commands += 1;
        }

        fn invalid_argument(
            &mut self,
            _command: &GCode,
            arg: char,
            _reason: &'static str,
        ) {
            self.invalid.push(arg);
        }
    }

    fn parse(src: &str) -> GCode {
        let lines: Vec<_> = gcode::parse(src).collect();
        assert_eq!(lines.len(), 1);
        let line = &lines[0];
        assert!(line.comments().is_empty());
        let commands = line.gcodes();
        assert_eq!(commands.len(), 1);
        commands[0].clone()
    }

    #[test]
    fn simple_linear_interpolation() {
        let mut trans = Translator::default();
        let start = trans.current_location;
        let gcode = parse("G01 X50 Y10 Z-5 F1000.0");
        let mut mocks = MockCallbacks::default();

        trans.translate(&gcode, &mut mocks);

        assert_eq!(mocks.linear.len(), 1);
        let got = mocks.linear[0];
        let expected_end = Point::new::<millimeter>(50.0, 10.0, -5.0);
        let expected_feed = Velocity::new::<millimeter_per_minute>(1000.0);
        assert_eq!(got, (start, expected_end, expected_feed));

        assert_eq!(trans.current_location, expected_end);
        assert_eq!(trans.feed_rate, expected_feed);
    }

    #[test]
    fn more_complicated_relative_circular_interpolation() {
        let mut trans = Translator {
            current_location: Point::new::<inch>(1.0, 10.0, -50.0),
            coordinate_mode: CoordinateMode::Relative,
            units: Units::Inches,
            ..Default::default()
        };
        let start = trans.current_location;
        let gcode = parse("G02 X10 Y-5 I5 J90 F1000.0");
        let mut mocks = MockCallbacks::default();

        trans.translate(&gcode, &mut mocks);

        assert_eq!(mocks.circular.len(), 1);
        let (got_start, got_centre, got_end, got_direction, got_feed) =
            mocks.circular[0];
        assert_eq!(got_start, start);
        assert_eq!(got_direction, Direction::Clockwise);
        let expected_end = start + Point::new::<inch>(10.0, -5.0, 0.0);
        assert_eq!(got_end, expected_end);
        let expected_centre = start + Point::new::<inch>(5.0, 90.0, 0.0);
        assert_eq!(got_centre, expected_centre);
        let expected_feed =
            Length::new::<inch>(1000.0) / Time::new::<minute>(1.0);
        assert_eq!(got_feed, expected_feed);

        assert_eq!(trans.current_location, expected_end);
        assert_eq!(trans.feed_rate, expected_feed);
    }

    #[test]
    fn switch_to_inches() {
        let mut trans = Translator {
            units: Units::Millimetres,
            ..Default::default()
        };
        let mut mocks = MockCallbacks::default();
        let gcode = parse("G20");

        trans.translate(&gcode, &mut mocks);

        assert_eq!(trans.units, Units::Inches);
    }

    #[test]
    fn switch_to_millimeters() {
        let mut trans = Translator {
            units: Units::Inches,
            ..Default::default()
        };
        let mut mocks = MockCallbacks::default();
        let gcode = parse("G21");

        trans.translate(&gcode, &mut mocks);

        assert_eq!(trans.units, Units::Millimetres);
    }

    #[test]
    fn switch_to_absolute() {
        let mut trans = Translator {
            coordinate_mode: CoordinateMode::Relative,
            ..Default::default()
        };
        let mut mocks = MockCallbacks::default();
        let gcode = parse("G90");

        trans.translate(&gcode, &mut mocks);

        assert_eq!(trans.coordinate_mode, CoordinateMode::Absolute);
    }

    #[test]
    fn switch_to_relative() {
        let mut trans = Translator {
            coordinate_mode: CoordinateMode::Absolute,
            ..Default::default()
        };
        let mut mocks = MockCallbacks::default();
        let gcode = parse("G91");

        trans.translate(&gcode, &mut mocks);

        assert_eq!(trans.coordinate_mode, CoordinateMode::Relative);
    }

    #[test]
    fn dwell() {
        let mut trans = Translator::default();
        let mut mocks = MockCallbacks::default();
        let gcode = parse("G04 P1.0");

        trans.translate(&gcode, &mut mocks);

        assert_eq!(mocks.dwell.len(), 1);
        assert_eq!(mocks.dwell[0], Duration::from_secs(1));
    }

    #[test]
    fn dwell_period_is_required() {
        let mut trans = Translator::default();
        let mut mocks = MockCallbacks::default();
        let gcode = parse("G04");

        trans.translate(&gcode, &mut mocks);

        assert!(mocks.dwell.is_empty());
        assert_eq!(1, mocks.invalid.len());
        assert_eq!('P', mocks.invalid[0]);
    }

    #[test]
    fn unsupported_g_command() {
        let mut trans = Translator::default();
        let mut mocks = MockCallbacks::default();
        let gcode = parse("G42");

        trans.translate(&gcode, &mut mocks);

        assert_eq!(1, mocks.unknown_commands);
    }

    #[test]
    fn unsupported_m_command() {
        let mut trans = Translator::default();
        let mut mocks = MockCallbacks::default();
        let gcode = parse("M42");

        trans.translate(&gcode, &mut mocks);

        assert_eq!(1, mocks.unknown_commands);
    }

    #[test]
    fn unsupported_mnemonic() {
        let mut trans = Translator::default();
        let mut mocks = MockCallbacks::default();
        let gcode = parse("T42");

        trans.translate(&gcode, &mut mocks);

        assert_eq!(1, mocks.unknown_commands);
    }
}
