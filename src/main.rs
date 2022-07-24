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

pub fn init() -> (Delay, LedArray) {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let delay = Delay::new(core_periphs.SYST, clocks);

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

    (delay, leds.into_array())
}

fn using_hal() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = init();
    
    let half_period = 1000_u16;
    
    loop {
        leds[0].on().ok();
        delay.delay_ms(half_period);
    
        leds[0].off().ok();
        delay.delay_ms(half_period);
    }

}

fn setup_gpio() -> (){
    const RCC_AHBENR_ADDR: *mut u32 = (0x4002_1000 + 0x14) as *mut u32;
    
    //GPIOE_MODER
    //Port mode register 0x00
    const GPIOE_MODER: *mut u32 = (0x4800_1000 + 0x00) as *mut u32;
    
    //GPIOE_OTYPER 0x04
    //Port output type register
    const GPIOE_OTYPER: *mut u32 = (0x4800_1000 + 0x04) as *mut u32;

    unsafe{
        let rcc_value = read_volatile(RCC_AHBENR_ADDR);
        write_volatile(RCC_AHBENR_ADDR, rcc_value | 0x2000);   //Set IOPEEN

        //North LED is PE9 
        let moder = read_volatile(GPIOE_MODER);
        write_volatile(GPIOE_MODER, moder & (1 << 9) );    //Set Pin 9 as push-pull output
        
        let otyper = read_volatile(GPIOE_OTYPER);
        write_volatile(GPIOE_OTYPER, otyper & (0x0000) );    //Set Pin 9 as push-pull output
                                                                           //This is actually the reset state so not strictly necessary
    
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
    

    let (mut delay, _): (Delay, LedArray) = aux5::init();

    let half_period_ms = 100_u16;

    //   setup_gpio();

    loop {
        set_led_on();
        short_delay(half_period_ms);

        set_led_off();
        short_delay(half_period_ms);
    
    }
}


#[entry]
fn main() -> !{
    blink_forever();
}
