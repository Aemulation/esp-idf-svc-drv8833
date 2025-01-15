use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{AnyOutputPin, Output, PinDriver},
    peripheral::PeripheralRef,
};

use anyhow::Result;

pub struct Sleep<'d> {
    pin: PinDriver<'d, AnyOutputPin, Output>,
}

impl<'d> Sleep<'d> {
    pub fn new(sleep_pin: PeripheralRef<'d, AnyOutputPin>) -> Result<Self> {
        Ok(Self {
            pin: PinDriver::output(sleep_pin)?,
        })
    }

    pub fn sleep(&mut self) -> Result<()> {
        self.pin.set_low()?;

        Ok(())
    }

    #[must_use]
    pub fn asleep(&self) -> bool {
        self.pin.is_set_low()
    }

    #[must_use]
    pub fn awake(&self) -> bool {
        self.pin.is_set_high()
    }

    pub fn wakeup(&mut self) -> Result<()> {
        if self.asleep() {
            self.pin.set_high()?;
            FreeRtos::delay_ms(1);
        }

        Ok(())
    }
}
