use anyhow::Result;

pub mod bidirectional_motor;
pub mod bidirectional_pwm_motor;
pub mod directional_motor;
pub mod directional_pwm_motor;
pub mod sleep;

pub trait DirectionalMotor {
    fn start(&mut self) -> Result<()>;

    fn stop(&mut self) -> Result<()>;
}

pub trait BidirectionalMotor {
    fn forward(&mut self) -> Result<()>;

    fn backward(&mut self) -> Result<()>;

    fn brake(&mut self) -> Result<()>;

    fn coast(&mut self) -> Result<()>;
}

pub trait DirectionalPwmMotor {
    fn start(&mut self, duty: u32) -> Result<()>;

    fn stop(&mut self) -> Result<()>;

    fn max_duty(&self) -> u32;
}

pub trait BidirectionalPwmMotor {
    fn forward(&mut self, duty: u32) -> Result<()>;

    fn backward(&mut self, duty: u32) -> Result<()>;

    fn brake(&mut self) -> Result<()>;

    fn coast(&mut self) -> Result<()>;

    fn max_duty(&self) -> u32;
}
