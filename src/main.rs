//! LED信号機その2：LEDの抽象化

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
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
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // ピンの集合
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // 2. LEDの変数宣言

    // 緑色LED
    let mut green_led = Led::new(pins.gpio13.into_push_pull_output());
    // 黄色LED
    let mut yellow_led = Led::new(pins.gpio12.into_push_pull_output());
    // 赤色LED
    let mut red_led = Led::new(pins.gpio11.into_push_pull_output());

    // 3. メインループ
    loop {
        // 青信号（5秒間）
        // 緑：点灯、黄：消灯、赤：消灯
        red_led.turn_off();
        yellow_led.turn_off();
        green_led.turn_on();
        delay_sec(&mut delay, 5);

        // 黄信号（2秒間）
        // 緑：消灯、黄：点灯、赤：消灯
        green_led.turn_off();
        red_led.turn_off();
        yellow_led.turn_on();
        delay_sec(&mut delay, 2);

        // 赤信号（3秒間）
        // 緑：消灯、黄：消灯、赤：点灯
        yellow_led.turn_off();
        green_led.turn_off();
        red_led.turn_on();
        delay_sec(&mut delay, 3);
    }
}

/// 指定された秒数だけビジーウェイトで待機する
fn delay_sec(delay: &mut Delay, sec: u32) {
    delay.delay_ms(sec * 1000);
}
