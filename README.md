# esp-idf-svc-drv8833

## example

```rust
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{AnyOutputPin, PinDriver};
use esp_idf_svc::hal::ledc::{config::TimerConfig, LedcChannel, LedcTimer, LedcTimerDriver};
use esp_idf_svc::hal::peripheral::{Peripheral, PeripheralRef};
use esp_idf_svc::hal::peripherals::Peripherals;

use drv8833::single_motor::SingleMotor;
use drv8833::single_pwm_motor::SinglePwmMotor;
use drv8833::sleep::Sleep;

use anyhow::Result;

fn single_motor_test<'d>(
    mut pin1: PeripheralRef<'d, AnyOutputPin>,
    mut pin2: PeripheralRef<'d, AnyOutputPin>,
) -> Result<()> {
    PinDriver::output(pin1.reborrow())?
        .set_drive_strength(esp_idf_svc::hal::gpio::DriveStrength::I40mA)?;
    PinDriver::output(pin2.reborrow())?
        .set_drive_strength(esp_idf_svc::hal::gpio::DriveStrength::I40mA)?;
    let mut motor = SingleMotor::new(pin1, pin2)?;

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

fn pwm_motor_test<'d, T, C1, C2>(
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
    let mut pwm_motor = SinglePwmMotor::new(pin1, pin2, timer_driver, channel1, channel2)?;

    let max_duty = pwm_motor.max_duty();
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
    esp_idf_svc::sys::link_patches();
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

    single_motor_test(pin_16.reborrow().map_into(), pin_17.reborrow().map_into())?;

    log::warn!("Going to sleep for 5 seconds");
    sleep.sleep()?;
    FreeRtos::delay_ms(5_000);
    sleep.wakeup()?;
    log::warn!("Awake");

    let mut timer = peripherals.ledc.timer0.into_ref();
    let timer_driver = LedcTimerDriver::new(timer.reborrow(), &TimerConfig::default())?;
    let channel0 = peripherals.ledc.channel0.into_ref();
    let channel1 = peripherals.ledc.channel1.into_ref();
    pwm_motor_test(
        pin_16.map_into(),
        pin_17.map_into(),
        &timer_driver,
        channel0,
        channel1,
    )?;

    log::warn!("Going back to sleep");
    sleep.sleep()?;

    Ok(())
}
```
