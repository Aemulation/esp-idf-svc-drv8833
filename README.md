# esp-idf-svc-drv8833

## example

```rust
use drv8833::bidirectional_motor::BidirectionalMotorDriver;
use drv8833::bidirectional_pwm_motor::BidirectionalPwmMotorDriver;
use drv8833::directional_motor::DirectionalMotorDriver;
use drv8833::directional_pwm_motor::DirectionalPwmMotorDriver;
use drv8833::{BidirectionalMotor, BidirectionalPwmMotor, DirectionalMotor, DirectionalPwmMotor};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::AnyOutputPin;
use esp_idf_svc::hal::ledc::{config::TimerConfig, LedcChannel, LedcTimer, LedcTimerDriver};
use esp_idf_svc::hal::peripheral::{Peripheral, PeripheralRef};
use esp_idf_svc::hal::peripherals::Peripherals;

use drv8833::sleep::Sleep;

use anyhow::Result;

fn directional_motor_test(pin1: PeripheralRef<'_, AnyOutputPin>) -> Result<()> {
    let mut motor = DirectionalMotorDriver::new(pin1)?;

    log::info!("Forward...");
    motor.start()?;
    FreeRtos::delay_ms(5_000);

    log::info!("Stopping...");
    motor.stop()?;
    FreeRtos::delay_ms(3_000);

    Ok(())
}

fn bidirectional_motor_test<'d>(
    pin1: PeripheralRef<'d, AnyOutputPin>,
    pin2: PeripheralRef<'d, AnyOutputPin>,
) -> Result<()> {
    let mut motor = BidirectionalMotorDriver::new(pin1, pin2)?;

    log::info!("Forward...");
    motor.forward()?;
    FreeRtos::delay_ms(5_000);

    log::info!("Coasting...");
    motor.coast()?;
    FreeRtos::delay_ms(3_000);

    log::info!("Backward...");
    motor.backward()?;
    FreeRtos::delay_ms(5_000);

    log::info!("Braking...");
    motor.brake()?;
    FreeRtos::delay_ms(3_000);

    Ok(())
}

fn directional_pwm_motor_test<'d, T, C1>(
    pin1: PeripheralRef<'d, AnyOutputPin>,
    timer_driver: &LedcTimerDriver<'d, T>,
    channel1: impl Peripheral<P = C1>,
) -> Result<()>
where
    T: LedcTimer + 'd,
    C1: LedcChannel<SpeedMode = <T as LedcTimer>::SpeedMode>,
{
    let mut pwm_motor = DirectionalPwmMotorDriver::new(pin1, timer_driver, channel1)?;

    let max_duty = pwm_motor.max_duty();
    let duty = max_duty * 85 / 100;

    log::info!("Starting...");
    DirectionalPwmMotor::start(&mut pwm_motor, duty)?;
    FreeRtos::delay_ms(5_000);

    log::info!("Stopping...");
    DirectionalPwmMotor::stop(&mut pwm_motor)?;
    FreeRtos::delay_ms(500);

    Ok(())
}

fn bidirectional_pwm_motor_test<'d, T, C1, C2>(
    pin1: PeripheralRef<'d, AnyOutputPin>,
    pin2: PeripheralRef<'d, AnyOutputPin>,
    timer_driver: &LedcTimerDriver<'d, T>,
    channel1: impl Peripheral<P = C1>,
    channel2: impl Peripheral<P = C2>,
) -> Result<()>
where
    T: LedcTimer + 'd,
    C1: LedcChannel<SpeedMode = <T as LedcTimer>::SpeedMode>,
    C2: LedcChannel<SpeedMode = <T as LedcTimer>::SpeedMode>,
{
    let mut pwm_motor =
        BidirectionalPwmMotorDriver::new(pin1, pin2, timer_driver, channel1, channel2)?;

    let max_duty = drv8833::BidirectionalPwmMotor::max_duty(&pwm_motor);
    let duty = max_duty * 85 / 100;

    log::info!("Forward...");
    pwm_motor.forward(duty)?;
    FreeRtos::delay_ms(5_000);

    log::info!("Coasting...");
    pwm_motor.coast()?;
    FreeRtos::delay_ms(500);

    log::info!("Backward...");
    pwm_motor.backward(duty)?;
    FreeRtos::delay_ms(5_000);

    log::info!("Coasting...");
    pwm_motor.coast()?;
    FreeRtos::delay_ms(500);

    Ok(())
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take()?;

    let mut pin_4 = peripherals.pins.gpio4.into_ref();
    let mut pin_16 = peripherals.pins.gpio16.into_ref();
    let mut pin_17 = peripherals.pins.gpio17.into_ref();

    let mut sleep = Sleep::new(pin_4.reborrow().map_into())?;

    if sleep.asleep() {
        log::warn!("Currently sleeping, waking up...");
        sleep.wakeup()?;
    }

    log::warn!("Starting directional motor test");
    directional_motor_test(pin_16.reborrow().map_into())?;
    log::warn!("Starting bidirectional motor test");
    bidirectional_motor_test(pin_16.reborrow().map_into(), pin_17.reborrow().map_into())?;

    log::warn!("Going to sleep for 5 seconds");
    sleep.sleep()?;
    FreeRtos::delay_ms(5_000);
    sleep.wakeup()?;
    log::warn!("Awake");

    let mut timer = peripherals.ledc.timer0.into_ref();
    let timer_driver = LedcTimerDriver::new(timer.reborrow(), &TimerConfig::default())?;
    let mut channel0 = peripherals.ledc.channel0.into_ref();
    let mut channel1 = peripherals.ledc.channel1.into_ref();

    log::warn!("Starting directional PWM motor test");
    directional_pwm_motor_test(
        pin_16.reborrow().map_into(),
        &timer_driver,
        channel0.reborrow(),
    )?;
    log::warn!("Starting bidirectional PWM motor test");
    bidirectional_pwm_motor_test(
        pin_16.reborrow().map_into(),
        pin_17.reborrow().map_into(),
        &timer_driver,
        channel0.reborrow(),
        channel1.reborrow(),
    )?;

    log::warn!("Going back to sleep");
    sleep.sleep()?;

    Ok(())
}
```
