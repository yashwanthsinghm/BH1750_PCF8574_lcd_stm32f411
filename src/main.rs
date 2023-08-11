#![no_std]
#![no_main]

// Imports
use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
// use stm32f4xx_hal::rcc;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
    serial::config::Config,
};

use defmt_rtt as _;




#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    defmt::println!("Started code");
    let dp = pac::Peripherals::take().unwrap();

    // I2C Config steps:
    // 1) Need to configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    // 8 MHz must be used for the Nucleo-F401RE board according to manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 2) Configure/Define SCL and SDA pins
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8;
    let sda = gpiob.pb9;



    // for blinking an led consider this ports:

    // GPIO Initialization
    let gpiod = dp.GPIOD.split();
    let mut green_led = gpiod.pd12.into_push_pull_output();
    let mut orange_led = gpiod.pd13.into_push_pull_output();
    let mut red_led = gpiod.pd14.into_push_pull_output();
    let mut blue_led = gpiod.pd15.into_push_pull_output();



    



    //defining a clock
    // Create a delay abstraction based on SysTick
    let dm =cortex_m::peripheral::Peripherals::take().unwrap();
    let mut delays = dm.SYST.delay(&clocks);




    // 3) Configure I2C peripheral channel


    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );


    // BH1750 I2C Address
    const BH1750_ADDR: u8 = 0x23;
    const POWER_DOWN: u8 = 0x00;
    let power_cont:u8=0x10;
    const POWER_ON: u8 = 0x01;
    const RESET: u8 = 0x07;
    const CONTINUOUS_HIGH_RES_MODE: u8 = 0x10;



    // let check2 = i2c.write(BH1750_ADDR, &[POWER_ON]).is_ok();
    // defmt::println!(".... :->  {:?}",check2);
    let mut rx_buffer: [u8; 2] = [0; 2];
    let mut rx_word: u16;

    //Power on the sensor
    i2c.write(BH1750_ADDR, &[POWER_ON]).unwrap();
    delays.delay_ms(10_u32);
    i2c.write(BH1750_ADDR, &[power_cont]).unwrap();
    delays.delay_ms(10_u32); // Wait for power-on initialization


    // Application Loop
    loop{
    let mut count=0;
    while count<1 {
        // Read light intensity data
            green_led.set_high();
            orange_led.set_high();
            red_led.set_high();
            blue_led.set_high();

            delays.delay_ms(500_u32);

            green_led.set_low();
            orange_led.set_low();
            red_led.set_low();
            blue_led.set_low();

            delays.delay_ms(500_u32);
            count+=1;
        }

        i2c.read(BH1750_ADDR, &mut rx_buffer).unwrap();

        rx_word = ((rx_buffer[0] as u16) << 8) | rx_buffer[1] as u16;  
        
        // Convert light intensity data to lux (value format as per datasheet)
        let lux = (rx_word as f32 / 1.2) as u16;
        
        // Print light intensity in lux
        defmt::println!(" Bh1750 intensity value: {:?}",lux as u16);

    }

}
