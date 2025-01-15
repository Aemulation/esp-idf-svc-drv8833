use anyhow::Result;

pub mod single_motor;
pub mod single_pwm_motor;
pub mod sleep;

pub trait Motor {
    fn forward(&mut self) -> Result<()>;

    fn backward(&mut self) -> Result<()>;

    fn brake(&mut self) -> Result<()>;

    fn coast(&mut self) -> Result<()>;
}

pub trait PwmMotor {
    fn forward(&mut self, duty: u32) -> Result<()>;

    fn backward(&mut self, duty: u32) -> Result<()>;

    fn brake(&mut self) -> Result<()>;

    fn coast(&mut self) -> Result<()>;

    fn max_duty(&self) -> u32;
}
