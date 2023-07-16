//#![deny(unsafe_code)]
#![no_std]
#![no_main]

pub use cortex_m_rt::entry;
pub use panic_itm; // panic handler
pub use stm32f3_discovery::stm32f3xx_hal; // Needed for the interrupt vector

const RCC_ADDR: u32 = 0x4002_1000;
const RCC_AHBENR_OFFSET: u32 = 0x14;
const RCC_AHBENR: u32 = RCC_ADDR + RCC_AHBENR_OFFSET;

const GPIOE_ADDR: u32 = 0x4800_1000;
const GPIOE_BSRR_OFFSET: u32 = 0x18;
const GPIOE_BSRR_ADDR: u32 = GPIOE_ADDR + GPIOE_BSRR_OFFSET;
const GPIOE_MODER_OFFSET: u32 = 0x00;
const GPIOE_MODER_ADDR: u32 = GPIOE_ADDR + GPIOE_MODER_OFFSET;

fn setup_gpio(pin: i32) -> () {
    unsafe {
        // Enable the GPIOE peripheral
        let ahbenr = &*(RCC_AHBENR as *mut volatile_register::RW<u32>);
        ahbenr.modify(|r| r | (1 << 21));

        // Set pin as output
        let moder = &*(GPIOE_MODER_ADDR as *mut volatile_register::RW<u32>);

        let pin_shift = pin * 2; // Calculate the bit position based on pin number
        let mask = 0b11 << pin_shift; // Create a mask for the pin bits in the register

        let mode = 0b01; // General purpose output mode
        let set_mode = mode << pin_shift; // Shift the mode to the correct position

        moder.modify(|r| (r & !mask) | set_mode); // First clear the two bits of this pins mode, then OR with the new (bit-shifted) value
    }
}

#[allow(non_snake_case)]
fn set_led_on(pin: i32) -> () {
    unsafe {
        let BSRR = &*(GPIOE_BSRR_ADDR as *mut volatile_register::RW<u32>);
        BSRR.write(1 << pin); // A pin is set by setting the bit in the lower 16 bits of the BSRR
    }
}

#[allow(non_snake_case)]
fn set_led_off(pin: i32) -> () {
    unsafe {
        let BSRR = &*(GPIOE_BSRR_ADDR as *mut volatile_register::RW<u32>);
        BSRR.write(1 << 16 + pin); // A pin is cleared by setting the bit in the top 16 bits of the BSRR
    }
}

fn short_delay(delay_ms: u16) {
    for _ in 0..(delay_ms * 50) {}
}

fn blink_forever(pin: i32) -> ! {
    let half_period_ms = 250_u16;

    loop {
        set_led_on(pin);
        short_delay(half_period_ms);

        set_led_off(pin);
        short_delay(half_period_ms);
    }
}

#[entry]
fn main() -> ! {
    let pin = 14;
    setup_gpio(pin);
    blink_forever(pin);
}
