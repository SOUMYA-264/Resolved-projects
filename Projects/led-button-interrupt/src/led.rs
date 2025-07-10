use crate::gpio::{
    enable_gpio_clock, set_gpio_mode_output, set_gpio_output_type_push_pull, set_gpio_pin_state, PinState
};

/// Initializes the LED on the specified GPIO port and pin
///
/// This functions performs the following steps:
/// 1. Enables the peripheral clock for the GPIO port
/// 2. Sets the GPIO pin mode to output mode
/// 3. Sets the output type to push/pull
/// 4. Optionally sets the output speed
/// 
/// # Arguments
/// * `port` - The GPIO port memory address to which the LED is connected
/// * `pin` - The GPIO pin number to which the LED is connected
///
/// # Example
/// ```rust
/// led_init(0, 5); // Initializes pin 5 on GPIOA
/// ```
pub fn led_init(port: u32, pin: u32) {
    // Enable the peripheral clock for the GPIO port
    enable_gpio_clock(port);
    
    // Set the GPIO pin mode to output mode
    set_gpio_mode_output(port, pin);

    // Set the output mode to push/pull
    set_gpio_output_type_push_pull(port, pin);

    // Set the output speed (optional)
}

pub fn led_on(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::High);
}

pub fn led_off(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Low);
}

pub fn led_toggle(port: u32, pin: u32) {
    // Read the current state of the LED pin using ODR register
    let gpio_odr_address = (port + 0x14) as *mut u32;
    let current_state = unsafe { core::ptr::read_volatile(gpio_odr_address) };

    if (current_state & (1 << pin)) != 0 {
        // If the LED is currently on, turn it off
        led_off(port, pin);
    } else {
        // If the LED is currently off, turn it on
        led_on(port, pin);
    }
}
