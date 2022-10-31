use core::fmt::Debug;

use embedded_hal::digital::v2::OutputPin;

/// LEDを表す構造体
#[derive(Debug)]
pub struct Led<T>
where
    T: OutputPin,
    T::Error: Debug,
{
    /// LEDが接続されているピン
    pin: T,
}

impl<T> Led<T>
where
    T: OutputPin,
    T::Error: Debug,
{
    /// 新しいインスタンスを返す
    pub fn new(pin: T) -> Self {
        Self { pin }
    }

    /// LEDを点灯させる
    pub fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    /// LEDを消灯させる
    pub fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }
}
