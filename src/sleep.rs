use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{AnyOutputPin, Output, PinDriver},
    peripheral::PeripheralRef,
};

use crate::{single_motor::SingleMotor, Motor};

use anyhow::Result;

pub struct Sleep<'d> {
    pin: Option<PinDriver<'d, AnyOutputPin, Output>>,
}

impl<'d> Sleep<'d> {
    pub fn new(sleep_pin: Option<PeripheralRef<'d, AnyOutputPin>>) -> Result<Self> {
        Ok(Self {
            pin: sleep_pin.map(PinDriver::output).transpose()?,
        })
    }

    pub fn sleep(&mut self) -> Result<()> {
        let Some(sleep_pin) = &mut self.pin else {
            return Ok(());
        };

        if sleep_pin.is_set_high() {
            sleep_pin.set_low()?;
        }

        Ok(())
    }

    #[must_use]
    pub fn asleep(&self) -> bool {
        self.pin.as_ref().is_none_or(PinDriver::is_set_low)
    }

    #[must_use]
    pub fn awake(&self) -> bool {
        self.pin.as_ref().is_none_or(PinDriver::is_set_high)
    }

    pub fn wakeup(&mut self) -> Result<()> {
        let Some(sleep_pin) = &mut self.pin else {
            return Ok(());
        };

        if sleep_pin.is_set_low() {
            sleep_pin.set_high()?;
            FreeRtos::delay_ms(1);
        }

        Ok(())
    }
}
