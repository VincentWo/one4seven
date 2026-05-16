use core::time::Duration;

use embedded_hal::i2c::{self, ErrorType};
use ht16k33::{HT16K33, LedLocation};

pub struct Display<I2c>
where
    I2c: i2c::I2c,
{
    pub ht16k33: HT16K33<I2c>,
}

impl<I2c> Display<I2c>
where
    I2c: i2c::I2c,
{
    pub fn new(ht16k33: HT16K33<I2c>) -> Self {
        Self { ht16k33 }
    }

    pub fn set_duration(&mut self, duration: Duration) -> Result<(), <I2c as ErrorType>::Error> {
        let seconds = duration.as_secs();
        let minutes: u8 = (seconds / 60).try_into().unwrap();
        let seconds: u8 = (seconds % 60).try_into().unwrap();

        assert!(minutes < 99);

        let digits = [
            (minutes >= 10).then_some(minutes / 10),
            Some(minutes % 10),
            Some(seconds / 10),
            Some(seconds % 10),
        ];

        for (i, digit) in digits.into_iter().enumerate() {
            if let Some(digit) = digit {
                self.set_digit(i as u8, digit)
            } else {
                self.clear_digit(i as u8)
            };
        }

        self.ht16k33.write_display_buffer()
    }

    pub fn set_digit(&mut self, mut column: u8, digit: u8) {
        assert!(column < 4);
        // The third column is the colon
        if column > 1 {
            column += 1;
        }

        let data = match digit {
            0 => [true, true, true, true, true, true, false],
            1 => [false, true, true, false, false, false, false],
            2 => [true, true, false, true, true, false, true],
            3 => [true, true, true, true, false, false, true],
            4 => [false, true, true, false, false, true, true],
            5 => [true, false, true, true, false, true, true],
            6 => [true, false, true, true, true, true, true],
            7 => [true, true, true, false, false, false, false],
            8 => [true, true, true, true, true, true, true],
            9 => [true, true, true, true, false, true, true],
            _too_large => unreachable!(),
        };

        for (row, enabled) in data.into_iter().enumerate() {
            self.ht16k33.update_display_buffer(
                ht16k33::LedLocation {
                    row: row as u8,
                    column,
                },
                enabled,
            );
        }
    }
    pub fn clear_digit(&mut self, mut column: u8) {
        if column > 1 {
            column += 1;
        }

        for row in 0..8 {
            self.ht16k33
                .update_display_buffer(LedLocation { row, column }, false);
        }
    }

    pub fn set_colon(&mut self, enabled: bool) {
        self.ht16k33
            .update_display_buffer(ht16k33::LedLocation { row: 1, column: 2 }, enabled);
    }
}
