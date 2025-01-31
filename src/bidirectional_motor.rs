use super::BidirectionalMotor;

use anyhow::Result;
use esp_idf_svc::hal::{
    gpio::{AnyOutputPin, Output, OutputPin, PinDriver},
    peripheral::Peripheral,
};

pub struct BidirectionalMotorDriver<'d> {
    in1: PinDriver<'d, AnyOutputPin, Output>,
    in2: PinDriver<'d, AnyOutputPin, Output>,
}

unsafe impl Sync for BidirectionalMotorDriver<'static> {}
unsafe impl Send for BidirectionalMotorDriver<'static> {}

impl<'d> BidirectionalMotorDriver<'d> {
    pub fn new(
        in1: impl Peripheral<P = impl OutputPin> + 'd,
        in2: impl Peripheral<P = impl OutputPin> + 'd,
    ) -> Result<Self> {
        let in1 = PinDriver::output(in1.into_ref().map_into())?;
        let in2 = PinDriver::output(in2.into_ref().map_into())?;

        Ok(Self { in1, in2 })
    }
}

impl BidirectionalMotor for BidirectionalMotorDriver<'_> {
    fn forward(&mut self) -> Result<()> {
        self.in1.set_high()?;
        self.in2.set_low()?;

        Ok(())
    }

    fn backward(&mut self) -> Result<()> {
        self.in1.set_low()?;
        self.in2.set_high()?;

        Ok(())
    }

    fn brake(&mut self) -> Result<()> {
        self.in1.set_high()?;
        self.in2.set_high()?;

        Ok(())
    }

    fn coast(&mut self) -> Result<()> {
        self.in1.set_low()?;
        self.in2.set_low()?;

        Ok(())
    }
}
