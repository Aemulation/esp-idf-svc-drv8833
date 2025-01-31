use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{AnyOutputPin, Output, OutputPin, PinDriver},
    peripheral::Peripheral,
};

use anyhow::Result;

pub struct Sleep<'d> {
    pin: PinDriver<'d, AnyOutputPin, Output>,
}

impl<'d> Sleep<'d> {
    pub fn new(sleep_pin: impl Peripheral<P = impl OutputPin> + 'd) -> Result<Self> {
        Ok(Self {
            pin: PinDriver::output(sleep_pin.into_ref().map_into())?,
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

    pub fn do_while_awake<F, U>(&mut self, function: F) -> Result<U>
    where
        F: FnOnce() -> U,
    {
        self.wakeup()?;

        let result = function();

        self.sleep()?;

        Ok(result)
    }
}
