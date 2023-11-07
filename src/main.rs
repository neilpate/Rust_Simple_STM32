#![no_std]
#![no_main]

pub use cortex_m_rt::entry;
pub use panic_itm; // panic handler
pub use stm32f3_discovery::stm32f3xx_hal; // Needed for the interrupt vector

// Reset and Clock Control
const RCC_ADDR: u32 = 0x4002_1000;
const RCC_AHBENR_OFFSET: u32 = 0x14;
const RCC_AHBENR: u32 = RCC_ADDR + RCC_AHBENR_OFFSET;

// GPIO

const GPIO_BSRR_OFFSET: u32 = 0x18;
const GPIO_MODER_OFFSET: u32 = 0x00;

const GPIOA_ADDR: u32 = 0x4800_0000;
const GPIOA_IDR: u32 = GPIOA_ADDR + 0x10;

const GPIOE_ADDR: u32 = 0x4800_1000;
const GPIOE_BSRR_ADDR: u32 = GPIOE_ADDR + GPIO_BSRR_OFFSET;
const GPIOE_MODER_ADDR: u32 = GPIOE_ADDR + GPIO_MODER_OFFSET;

fn setup_gpioa_pin_as_input() -> () {
    unsafe {
        // Enable the GPIOA peripheral
        let ahbenr = &*(RCC_AHBENR as *mut volatile_register::RW<u32>);
        ahbenr.modify(|r| r | (1 << 17));

        // By default the pin is set to be an input, so no further config is needed
    }
}

fn setup_gpioe_pin_as_output(pin: i32) -> () {
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
    for _ in 0..(delay_ms * 67) {} // Constant experimentally determined
}

fn read_input() -> bool {
    unsafe {
        let idr = &*(GPIOA_IDR as *mut volatile_register::RW<u32>);
        let value = idr.read();

        let mask = 0x1;

        value & mask > 0
    }
}

fn blink_forever(pin: i32) -> ! {
    let fast_blink_half_period_ms = 100_u16;
    let slow_blink_half_period_ms = fast_blink_half_period_ms / 2;
    let mut blink_half_period_ms = fast_blink_half_period_ms;

    let mut previous_switch_state = read_input();
    let mut blinking_fast = true;

    loop {
        // Sample the state of the switch, and if it transitions from low to high
        // then switch to the other rate of blinking (fast/slow/fast/slow etc)

        let switch_pressed = read_input();

        if switch_pressed && !previous_switch_state {
            // Toggle blinking rate

            blinking_fast = !blinking_fast;

            blink_half_period_ms = if blinking_fast {
                fast_blink_half_period_ms
            } else {
                slow_blink_half_period_ms
            };
        }

        previous_switch_state = switch_pressed;

        set_led_on(pin);
        short_delay(blink_half_period_ms);

        set_led_off(pin);
        short_delay(blink_half_period_ms);
    }
}

#[entry]
fn main() -> ! {
    setup_gpioa_pin_as_input();

    let led_output_pin = 13; // South direction of compass (Red)
    setup_gpioe_pin_as_output(led_output_pin);

    blink_forever(led_output_pin);
}
