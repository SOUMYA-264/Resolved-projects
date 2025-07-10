use core::ptr;

pub fn read_register(address: *mut u32) -> u32 {
    unsafe { ptr::read_volatile(address) }
}

pub fn write_register(address: *mut u32, value: u32) {
    unsafe { ptr::write_volatile(address, value) }
}

pub fn register_set_bits(
    register_address: *mut u32,
    new_bits_value: u32,
    bit_position: u32,
    number_of_bits: u32,
) {
    assert!(
        number_of_bits > 0 && number_of_bits <= 32,
        "number_of_bits must be between 1 and 32"
    );
    assert!(bit_position < 32, "bit_position must be less than 32");

    // Read the current value of the register
    let register_value = read_register(register_address);

    // Create a mask for the bits to clear
    let mask = ((1 << number_of_bits) - 1) << bit_position;

    // Clear the relevant bits in the register and set the new value
    let updated_value = (register_value & !mask) | ((new_bits_value << bit_position) & mask);

    // Write the modified value back to the register
    write_register(register_address, updated_value);
}

pub fn register_set_bit(register_address: *mut u32, bit_position: u32, bit_value: bool) {
    // Read the current value of the register
    let reg_value = read_register(register_address);

    // Set or clear the specific bit based on `bit_value`
    let updated_value = if bit_value {
        reg_value | (1 << bit_position)
    } else {
        reg_value & !(1 << bit_position)
    };

    // Write the modified value back to the register
    write_register(register_address, updated_value);
}
