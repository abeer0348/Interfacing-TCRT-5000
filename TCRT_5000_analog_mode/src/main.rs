#![no_std]
#![no_main]

extern crate panic_itm;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};

// For On Board Leds
use stm32f3_discovery::{leds::Leds, switch_hal};
use switch_hal::OutputSwitch;


use stm32f3xx_hal::{adc, pac, prelude::*,delay::Delay};

#[entry]

fn main() -> ! {
    // Using stm32f3xx
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
    let mut itm = cp.ITM ;
    let mut delay = Delay::new(cp.SYST, clocks);

    
    let mut adc1 = adc::Adc::adc1(
        dp.ADC1,
        &mut dp.ADC1_2,
        &mut rcc.ahb,
        adc::CkMode::default(),
        clocks,
    );

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
   
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );
    let mut leds = leds.into_array();

    let mut adc1_in1_pin = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    let mut previous:usize = 0 ;

    loop {
        let adc_read_current : u16 = adc1.read(&mut adc1_in1_pin).expect("Error reading adc1.");
        
        iprintln!(&mut itm.stim[0],"PA1 reads {}", adc_read_current);
        let current:usize = led_num(adc_read_current);
        
        // for i in 0..8 {
        //     leds[i].off();
        // }
        if current >previous {
            for i in previous..current {
                leds[i].on();
                delay.delay_ms(50_u16);
            }
        }
        else if current <previous {
            for i in (current..previous).rev() {
                leds[i].off();
                delay.delay_ms(50_u16);
            }
        }
        else {}
        
        previous = current ;
        

}
}

fn led_num (value : u16) -> usize {
    if value > 3900 {0}
    else if value > 3500 {1}
    else if value > 3000 {2}
    else if value > 2500 {3}
    else if value > 2000 {4}
    else if value > 1500 {5}
    else if value > 1000 {6}
    else if value > 500 {7}
    else {8}
}