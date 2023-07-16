//#![deny(unsafe_code)]
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

pub use panic_itm; // panic handler

pub use cortex_m_rt::entry;

pub use stm32f3_discovery::{leds::Leds, stm32f3xx_hal, switch_hal};
pub use switch_hal::{ActiveHigh, OutputSwitch, Switch, ToggleableOutputSwitch};

use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::{
    delay::Delay,
    gpio::{gpioe, Output, PushPull},
    hal::blocking::delay::DelayMs,
    pac,
};



pub type LedArray = [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8];

pub fn init() -> () {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

   // initialize user leds
   let mut gpioa = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let leds = Leds::new(
        gpioa.pe8,
        gpioa.pe9,
        gpioa.pe10,
        gpioa.pe11,
        gpioa.pe12,
        gpioa.pe13,
        gpioa.pe14,
        gpioa.pe15,
        &mut gpioa.moder,
        &mut gpioa.otyper,
    );

   // leds.into_array()
}


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

use volatile_register::{RO, RW, WO};

#[allow(non_snake_case)]
#[repr(C)]
pub struct Gpio {
    MODER: RW<u32>,
    OTYPER: RW<u32>,
    OSPEEDR: RW<u32>,
    PUPDR: RW<u32>,
    IDR: RO<u32>,
    ODR: RW<u32>,
    BSRR: WO<u32>,
    LCKR: RW<u32>,
    AFR: [RW<u32>; 2],
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct Rcc {
    CR: RW<u32>,
    CFGR: RW<u32>,
    CIR: RW<u32>,
    APB2RSTR: RW<u32>,
    APB1RSTR: RW<u32>,
    AHBENR: RW<u32>,
    APB2ENR: RW<u32>,
    APB1ENR: RW<u32>,
    BDCR: RW<u32>,
    CSR: RW<u32>,
    AHBRSTR: RW<u32>,
    CFGR2: RW<u32>,
    CFGR3: RW<u32>,
    CR2: RW<u32>,
}

fn take2() -> !{

    let pin = 9;

    let ahbenr = unsafe { &*(0x40021014 as *mut volatile_register::RW<u32>) };

    let moder = unsafe { &*(0x48001000 as *mut volatile_register::RW<u32>) };
    let bsrr = unsafe { &*(0x48001018 as *mut volatile_register::RW<u32>) };

    // Enable the GPIOE peripheral
    // unsafe {ahbenr.modify(|r| r | (1 << 21)) };

    // // Set PE8 as output
    // unsafe {moder.modify(|r| (r & !(0b11 << (pin * 2))) | (0b01 << (pin * 2)))};

    let half_period_ms = 100_u16;

    //  setup_gpio();
  
      loop {
          set_led_on();
          short_delay(half_period_ms);
  
          set_led_off();
          short_delay(half_period_ms);
      
      }


    // Set PE8 high
   // unsafe {bsrr.write(1 << pin) };

loop {

}

}


#[entry]
fn main() -> !{
    setup_gpio();
    blink_forever();
    // take2();
}
