use core::time::Duration;

use crate::segment::{self, State};

pub struct Display<Driver>
where
    Driver: segment::Driver,
{
    pub inner: Driver,
}

impl<Driver> Display<Driver>
where
    Driver: segment::Driver,
{
    pub fn new(inner: Driver) -> Self {
        Self { inner }
    }

    pub fn set_duration(&mut self, duration: Duration) -> Result<(), Driver::Error> {
        let seconds = duration.as_secs();
        let minutes = seconds / 60;
        let seconds = seconds % 60;

        if minutes < 100 {
            let digits = [
                (minutes >= 10).then_some(minutes / 10),
                Some(minutes % 10),
                Some(seconds / 10),
                Some(seconds % 10),
            ];

            for (i, digit) in digits.into_iter().enumerate() {
                if let Some(digit) = digit {
                    self.set_digit(i as u8, digit as u8)?;
                } else {
                    self.clear_digit(i as u8)?;
                };
            }
        } else {
            let hours = minutes / 60;
            let minutes = minutes % 60;
            if hours < 10 {
                self.set_digit(0, hours as u8)?;
                self.inner.set_state(
                    1,
                    State {
                        top_left: true,
                        bottom_left: true,
                        middle: true,
                        bottom_right: true,
                        ..Default::default()
                    },
                )?;
                self.set_digit(2, (minutes / 10) as u8)?;
                self.set_digit(3, (minutes % 10) as u8)?;
            } else if hours < 1000 {
                for (pos, digit) in [
                    (hours >= 100).then_some(hours / 100),
                    (hours >= 10).then_some((hours / 10) % 10),
                    Some(hours % 10),
                ]
                .into_iter()
                .enumerate()
                {
                    if let Some(digit) = digit {
                        self.set_digit(pos as u8, (digit) as u8)?;
                    } else {
                        self.clear_digit(pos as u8)?;
                    }
                }
                self.inner.set_state(
                    3,
                    State {
                        top_left: true,
                        bottom_left: true,
                        middle: true,
                        bottom_right: true,
                        ..Default::default()
                    },
                )?;
            }
        }

        self.inner.update()
    }

    pub fn set_missing(&mut self) -> Result<(), Driver::Error> {
        for pos in 0..4 {
            self.inner.set_state(
                pos,
                State {
                    middle: true,
                    ..Default::default()
                },
            )?;
        }

        self.inner.update()
    }

    pub fn set_digit(&mut self, pos: u8, digit: u8) -> Result<(), Driver::Error> {
        let state = match digit {
            0 => segment::State {
                top: true,
                top_left: true,
                top_right: true,
                bottom_left: true,
                bottom_right: true,
                bottom: true,
                ..Default::default()
            },
            1 => segment::State {
                top_right: true,
                bottom_right: true,
                ..Default::default()
            },
            2 => segment::State {
                top: true,
                top_right: true,
                middle: true,
                bottom_left: true,
                bottom: true,
                ..Default::default()
            },
            3 => segment::State {
                top: true,
                top_right: true,
                middle: true,
                bottom_right: true,
                bottom: true,
                ..Default::default()
            },
            4 => segment::State {
                top_left: true,
                top_right: true,
                middle: true,
                bottom_right: true,
                ..Default::default()
            },
            5 => segment::State {
                top: true,
                top_left: true,
                middle: true,
                bottom_right: true,
                bottom: true,
                ..Default::default()
            },
            6 => segment::State {
                top: true,
                top_left: true,
                middle: true,
                bottom_right: true,
                bottom: true,
                bottom_left: true,
                ..Default::default()
            },
            7 => segment::State {
                top: true,
                top_right: true,
                bottom_right: true,
                ..Default::default()
            },
            8 => segment::State {
                top: true,
                top_left: true,
                top_right: true,
                middle: true,
                bottom_left: true,
                bottom_right: true,
                bottom: true,
                ..Default::default()
            },
            9 => segment::State {
                top: true,
                top_left: true,
                top_right: true,
                middle: true,
                bottom_right: true,
                bottom: true,
                ..Default::default()
            },
            _too_large => unreachable!(),
        };
        //     [true, true, true, true, true, true, false],
        //     [false, true, true, false, false, false, false],
        //     [true, true, false, true, true, false, true],
        //     [true, true, true, true, false, false, true],
        //     [false, true, true, false, false, true, true],

        //     [true, false, true, true, false, true, true],

        //     [true, false, true, true, true, true, true]

        // [true, true, true, false, false, false, false]

        //     [true, true, true, true, true, true, true]

        //     [true, true, true, false, false, true, true]

        self.inner.set_state(pos, state)
    }
    pub fn clear_digit(&mut self, digit: u8) -> Result<(), Driver::Error> {
        self.inner.set_state(digit, Default::default())
    }

    pub fn set_colon(&mut self, enabled: bool) -> Result<(), Driver::Error> {
        self.inner.set_colon(enabled)
    }
}
