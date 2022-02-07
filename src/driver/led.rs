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

/// LEDの操作を表すトレイト
pub trait LedController {
    /// LEDを点灯させる
    fn turn_on(&mut self);

    /// LEDを消灯させる
    fn turn_off(&mut self);
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
}

impl<T> LedController for Led<T>
where
    T: OutputPin,
    T::Error: Debug,
{
    /// LEDを点灯させる
    fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    /// LEDを消灯させる
    fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }
}
