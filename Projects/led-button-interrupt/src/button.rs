enum ButtonStatus {
    Pressed,
    Released,
}

pub fn button_init(port: u32, pin: u32) {}

pub fn button_interrupt_configuration() {}

pub fn button_read_state(pin: u32) -> ButtonStatus {
    ButtonStatus::Released
}
