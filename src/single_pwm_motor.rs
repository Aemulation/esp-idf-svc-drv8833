use super::{Motor, PwmMotor};
use std::marker::PhantomData;

use anyhow::Result;
use esp_idf_svc::hal::{
    gpio::OutputPin,
    ledc::{LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver},
    peripheral::Peripheral,
};

pub struct SinglePwmMotor<'d, T, C1, C2> {
    in1: LedcDriver<'d>,
    in2: LedcDriver<'d>,

    _ledc_timer: PhantomData<T>,
    _channel1: PhantomData<C1>,
    _channel2: PhantomData<C2>,
}

impl<'d, C1, C2, T> SinglePwmMotor<'d, T, C1, C2> {
    pub fn new(
        in1: impl Peripheral<P = impl OutputPin> + 'd,
        in2: impl Peripheral<P = impl OutputPin> + 'd,
        timer_driver: &'d LedcTimerDriver<'d, T>,
        channel1: impl Peripheral<P = C1> + 'd,
        channel2: impl Peripheral<P = C2> + 'd,
    ) -> Result<Self>
    where
        C1: LedcChannel<SpeedMode = <T as LedcTimer>::SpeedMode>,
        C2: LedcChannel<SpeedMode = <T as LedcTimer>::SpeedMode>,
        T: LedcTimer + 'd,
    {
        let in1 = LedcDriver::new(channel1, timer_driver, in1)?;
        let in2 = LedcDriver::new(channel2, timer_driver, in2)?;

        if in1.get_max_duty() != in2.get_max_duty() {
            anyhow::bail!(
                "Maximum duty of input pins is not the same, in1: {}, in2: {}",
                in1.get_duty(),
                in2.get_duty()
            );
        }

        Ok(Self {
            in1,
            in2,
            _ledc_timer: PhantomData::<T>,
            _channel1: PhantomData::<C1>,
            _channel2: PhantomData::<C2>,
        })
    }

    pub fn forward(&mut self, duty: u32) -> Result<()> {
        self.in1.set_duty(duty)?;
        self.in2.disable()?;

        Ok(())
    }

    pub fn backward(&mut self, duty: u32) -> Result<()> {
        self.in1.disable()?;
        self.in2.set_duty(duty)?;

        Ok(())
    }

    pub fn coast(&mut self) -> Result<()> {
        self.in1.disable()?;
        self.in2.disable()?;

        Ok(())
    }

    pub fn brake(&mut self) -> Result<()> {
        self.in1.enable()?;
        self.in2.enable()?;

        Ok(())
    }

    #[must_use]
    pub fn max_duty(&self) -> u32 {
        self.in1.get_max_duty()
    }
}

impl<'d, T, C1, C2> Motor for SinglePwmMotor<'d, T, C1, C2> {
    fn forward(&mut self) -> Result<()> {
        self.forward(self.max_duty())
    }

    fn backward(&mut self) -> Result<()> {
        self.backward(self.max_duty())
    }

    fn brake(&mut self) -> Result<()> {
        self.brake()
    }

    fn coast(&mut self) -> Result<()> {
        self.coast()
    }
}

impl<'d, T, C1, C2> PwmMotor for SinglePwmMotor<'d, T, C1, C2> {
    fn forward(&mut self, duty: u32) -> Result<()> {
        self.forward(duty)
    }

    fn backward(&mut self, duty: u32) -> Result<()> {
        self.backward(duty)
    }

    fn coast(&mut self) -> Result<()> {
        self.coast()
    }

    fn brake(&mut self) -> Result<()> {
        self.brake()
    }

    #[must_use]
    fn max_duty(&self) -> u32 {
        self.max_duty()
    }
}
