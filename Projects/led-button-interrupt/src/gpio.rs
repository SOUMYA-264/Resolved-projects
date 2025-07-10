use crate::{mcu::{GPIOD_BASE, RCC_BASE}, register::{register_set_bits,register_set_bit, write_register}};

pub enum PinState {
    Low = 0,
    High = 1,
}

pub fn enable_gpio_clock(port: u32) {
    let rcc_ahb1enr_address = (RCC_BASE + 0x30) as *mut u32;

    match port {
        GPIOD_BASE => {
            register_set_bit(rcc_ahb1enr_address, 3, true);
        },
        _ => {}
    }
}

pub fn set_gpio_mode_output(port: u32, pin: u32) {
    let gpio_mode_reg_addr = (port + 0x00) as *mut u32;
    let bit_position = pin * 2; // Each pin uses 2 bits in the mode register
    let mode_value = 0x1;

    register_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_output_type_push_pull(port: u32, pin: u32) {
    let gpio_op_type_reg_addr = (port + 0x04) as *mut u32;
    let bit_position = pin; // Each pin uses 1 bit in the output type register
    let bit_value = 0;

    register_set_bits(gpio_op_type_reg_addr, bit_value, bit_position, 1);
}

pub fn set_gpio_pin_state(port: u32, pin: u32, state: PinState) {
    let gpio_bsrr_address = (port + 0x18) as *mut u32;

    match state {
        PinState::High => write_register(gpio_bsrr_address, 1 << pin),
        PinState::Low => write_register(gpio_bsrr_address, 1 << (pin + 16)),
    }
}