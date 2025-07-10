#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

use crate::{
    board::{BLUE_LED_PIN, BLUE_LED_PORT, BUTTON_PIN, BUTTON_PORT, GREEN_LED_PIN, GREEN_LED_PORT, ORANGE_LED_PIN, ORANGE_LED_PORT, RED_LED_PIN, RED_LED_PORT},
    button::{button_init, button_interrupt_configuration},
    led::{led_init, led_on},
};

mod board;
mod button;
mod gpio;
mod led;
mod mcu;
mod register;
mod startup_stm32f407vg;

#[unsafe(no_mangle)]
fn main() -> ! {
    led_init(BLUE_LED_PORT, BLUE_LED_PIN);
    led_init(RED_LED_PORT, RED_LED_PIN);
    led_init(GREEN_LED_PORT, GREEN_LED_PIN);
    led_init(ORANGE_LED_PORT, ORANGE_LED_PIN);

    led_on(BLUE_LED_PORT, BLUE_LED_PIN);
    led_on(GREEN_LED_PORT, GREEN_LED_PIN);
    led_on(RED_LED_PORT, RED_LED_PIN);
    led_on(ORANGE_LED_PORT, ORANGE_LED_PIN);

    // button_init(BUTTON_PORT, BUTTON_PIN);
    // button_interrupt_configuration();
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
