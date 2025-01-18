use super::DirectionalMotor;

use anyhow::Result;
use esp_idf_svc::hal::{
    gpio::{AnyOutputPin, Output, PinDriver},
    peripheral::PeripheralRef,
};

pub struct DirectionalMotorDriver<'d> {
    in1: PinDriver<'d, AnyOutputPin, Output>,
}

unsafe impl Sync for DirectionalMotorDriver<'static> {}
unsafe impl Send for DirectionalMotorDriver<'static> {}

impl<'d> DirectionalMotorDriver<'d> {
    pub fn new(in1: PeripheralRef<'d, AnyOutputPin>) -> Result<Self> {
        let in1 = PinDriver::output(in1)?;

        Ok(Self { in1 })
    }
}

impl DirectionalMotor for DirectionalMotorDriver<'_> {
    fn start(&mut self) -> Result<()> {
        self.in1.set_high()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.in1.set_low()?;

        Ok(())
    }
}
