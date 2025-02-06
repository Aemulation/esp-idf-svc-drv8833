use super::{DirectionalMotor, DirectionalPwmMotor};
use std::marker::PhantomData;

use anyhow::Result;
use esp_idf_svc::hal::{
    gpio::OutputPin,
    ledc::{LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver},
    peripheral::Peripheral,
};

pub struct DirectionalPwmMotorDriver<'d, T, C1> {
    in1: LedcDriver<'d>,

    _ledc_timer: PhantomData<T>,
    _channel1: PhantomData<C1>,
}

impl<'d, C1, T> DirectionalPwmMotorDriver<'d, T, C1> {
    pub fn new(
        in1: impl Peripheral<P = impl OutputPin> + 'd,
        timer_driver: &'d LedcTimerDriver<'d, T>,
        channel1: impl Peripheral<P = C1> + 'd,
    ) -> Result<Self>
    where
        C1: LedcChannel<SpeedMode = <T as LedcTimer>::SpeedMode>,
        T: LedcTimer + 'd,
    {
        let in1 = LedcDriver::new(channel1, timer_driver, in1)?;

        Ok(Self {
            in1,
            _ledc_timer: PhantomData::<T>,
            _channel1: PhantomData::<C1>,
        })
    }
}

impl<T, C1> DirectionalMotor for DirectionalPwmMotorDriver<'_, T, C1> {
    fn start(&mut self) -> Result<()> {
        DirectionalPwmMotor::start(self, self.max_duty())
    }

    fn stop(&mut self) -> Result<()> {
        DirectionalPwmMotor::stop(self)
    }
}

impl<T, C1> DirectionalPwmMotor for DirectionalPwmMotorDriver<'_, T, C1> {
    #[must_use]
    fn max_duty(&self) -> u32 {
        self.in1.get_max_duty()
    }

    fn start(&mut self, duty: u32) -> Result<()> {
        self.in1.set_duty_cycle(duty)?;

        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.in1.set_duty_cycle(0)?;

        Ok(())
    }
}
