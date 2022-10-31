//! LED信号機その3：宣言的な記述

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use cortex_m::delay::Delay;

// 1. モジュールの参照
mod driver;
use driver::led::Led;

mod traffic_signal;
use traffic_signal::{Color, Light, TrafficSignal};

#[entry]
fn main() -> ! {
    // 変数宣言・初期設定
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // クロックの設定
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // ビジーウェイトの抽象化
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // ピンの集合
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // 2. 変数宣言

    // 緑色LED
    let green_led = Led::new(pins.gpio13.into_push_pull_output());
    // 黄色LED
    let yellow_led = Led::new(pins.gpio12.into_push_pull_output());
    // 赤色LED
    let red_led = Led::new(pins.gpio11.into_push_pull_output());

    // 信号機
    let mut signal = TrafficSignal::new(green_led, yellow_led, red_led);
    // 信号機の点灯パターン
    let light_pattern = [
        Light {
            color: Color::Green,
            sec: 5,
        },
        Light {
            color: Color::Yellow,
            sec: 2,
        },
        Light {
            color: Color::Red,
            sec: 3,
        },
    ];

    // 3. メインループ
    loop {
        signal.run_cycle(&light_pattern, &mut delay);
    }
}
