//#![deny(unsafe_code)]
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

pub use panic_itm; // panic handler

pub use cortex_m_rt::entry;




use volatile_register::{RW};
fn setup_gpio() -> (){
    let pin = 9;
    const RCC_ADDR : u32 = 0x4002_1000; 
    const AHBENR_OFFSET: u32 = 0x14;
    const AHBENR: u32 = RCC_ADDR + AHBENR_OFFSET;

    const GPIO_E : u32 = 0x4800_1000;
    const MODER_OFFSET: u32 = 0x00;
    const MODER:u32 = GPIO_E + MODER_OFFSET;

    unsafe {

        // Enable the GPIOE peripheral
    let ahbenr =  &*(AHBENR as *mut volatile_register::RW<u32>) ;
    ahbenr.modify(|r| r | (1 << 21)) ;

    // Set PE8 as output
    let moder =  &*(MODER as *mut volatile_register::RW<u32>) ;
    moder.modify(      |r| (r & !(0b11 << (pin * 2))) | (0b01 << (pin * 2))        );
   
    }

}

#[allow(non_snake_case)]
fn set_led_on() -> (){
    const GPIOE_BSRR: *mut u32 = (0x4800_1000 + 0x18) as *mut u32;

    unsafe{
        write_volatile(GPIOE_BSRR, 1 << 9);   
    }
}

fn set_led_off() ->() {
    const GPIOE_BSRR: *mut u32 = (0x4800_1000 + 0x18) as *mut u32;

    unsafe{
      write_volatile(GPIOE_BSRR, 1 << 25);   
    }

}

fn short_delay(delay_ms :u16){
    for i in 0..(delay_ms* 50) {

    }
}

fn blink_forever() -> !{
    
    let half_period_ms = 100_u16;

    loop {
        set_led_on();
        short_delay(half_period_ms);

        set_led_off();
        short_delay(half_period_ms);
    
    }
}



#[entry]
fn main() -> !{
    setup_gpio();
    blink_forever();
}
