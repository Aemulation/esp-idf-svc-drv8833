use super::Motor;

use anyhow::Result;
use esp_idf_svc::hal::{
    gpio::{AnyOutputPin, Output, PinDriver},
    peripheral::PeripheralRef,
};

pub struct SingleMotor<'d> {
    in1: PinDriver<'d, AnyOutputPin, Output>,
    in2: PinDriver<'d, AnyOutputPin, Output>,
}

unsafe impl Sync for SingleMotor<'static> {}
unsafe impl Send for SingleMotor<'static> {}

impl<'d> SingleMotor<'d> {
    pub fn new(
        in1: PeripheralRef<'d, AnyOutputPin>,
        in2: PeripheralRef<'d, AnyOutputPin>,
    ) -> Result<Self> {
        let in1 = PinDriver::output(in1)?;
        let in2 = PinDriver::output(in2)?;

        Ok(Self { in1, in2 })
    }

    pub fn forward(&mut self) -> Result<()> {
        self.in1.set_high()?;
        self.in2.set_low()?;

        Ok(())
    }

    pub fn backward(&mut self) -> Result<()> {
        self.in1.set_low()?;
        self.in2.set_high()?;

        Ok(())
    }

    pub fn brake(&mut self) -> Result<()> {
        self.in1.set_high()?;
        self.in2.set_high()?;

        Ok(())
    }

    pub fn coast(&mut self) -> Result<()> {
        self.in1.set_low()?;
        self.in2.set_low()?;

        Ok(())
    }
}

impl Motor for SingleMotor<'_> {
    fn forward(&mut self) -> Result<()> {
        self.forward()
    }

    fn backward(&mut self) -> Result<()> {
        self.backward()
    }

    fn brake(&mut self) -> Result<()> {
        self.brake()
    }

    fn coast(&mut self) -> Result<()> {
        self.coast()
    }
}
