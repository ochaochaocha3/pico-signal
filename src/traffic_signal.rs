use core::fmt::Debug;

use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;

use crate::driver::led::Led;

/// 信号の色を表す型
#[derive(Debug, Clone, Copy)]
pub enum Color {
    /// 青信号
    Green,
    /// 黄信号
    Yellow,
    /// 赤信号
    Red,
}

/// 点灯状態を表す型
#[derive(Debug, Clone, Copy)]
pub struct Light {
    /// 点灯させる色
    pub color: Color,
    /// 点灯させる時間 [秒]
    pub sec: u32,
}

/// 信号機を表す型
#[derive(Debug)]
pub struct TrafficSignal<GreenLedPin, YellowLedPin, RedLedPin>
where
    GreenLedPin: OutputPin,
    GreenLedPin::Error: Debug,
    YellowLedPin: OutputPin,
    YellowLedPin::Error: Debug,
    RedLedPin: OutputPin,
    RedLedPin::Error: Debug,
{
    /// 緑色LED
    green_led: Led<GreenLedPin>,
    /// 黄色LED
    yellow_led: Led<YellowLedPin>,
    /// 赤色LED
    red_led: Led<RedLedPin>,
}

impl<GreenLedPin, YellowLedPin, RedLedPin> TrafficSignal<GreenLedPin, YellowLedPin, RedLedPin>
where
    GreenLedPin: OutputPin,
    GreenLedPin::Error: Debug,
    YellowLedPin: OutputPin,
    YellowLedPin::Error: Debug,
    RedLedPin: OutputPin,
    RedLedPin::Error: Debug,
{
    /// 新しいインスタンスを生成する
    pub fn new(
        green_led: Led<GreenLedPin>,
        yellow_led: Led<YellowLedPin>,
        red_led: Led<RedLedPin>,
    ) -> Self {
        Self {
            green_led,
            yellow_led,
            red_led,
        }
    }

    /// 1周期分の点灯制御を行う
    pub fn run_cycle(&mut self, light_pattern: &[Light], delay: &mut Delay) {
        for l in light_pattern.into_iter() {
            self.turn_off_all_leds();
            self.turn_on(l.color);
            delay.delay_ms(l.sec * 1000);
        }
    }

    /// すべてのLEDを消灯させる
    fn turn_off_all_leds(&mut self) {
        self.green_led.turn_off();
        self.yellow_led.turn_off();
        self.red_led.turn_off();
    }

    /// 指定した色のLEDを点灯させる
    fn turn_on(&mut self, color: Color) {
        match color {
            Color::Green => self.green_led.turn_on(),
            Color::Yellow => self.yellow_led.turn_on(),
            Color::Red => self.red_led.turn_on(),
        };
    }
}
