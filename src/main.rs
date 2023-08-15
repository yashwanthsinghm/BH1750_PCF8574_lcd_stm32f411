#![no_std]
#![no_main]

use core::u8;

// Importss
// use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
    serial::{config::Config, SerialExt},
};
use defmt_rtt;
#[entry]
fn main() -> ! {
    // Setup handler for device peripheralshttps://github.com/wfraser/lcd-pcf8574.git
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();    
    // I2C Config steps:
    // 1) Need to configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    // 8 MHz must be used for the Nucleo-F401RE board according to the manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 2) Configure/Define SCL and SDA pins
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8;
    let sda = gpiob.pb9;
    // 3) Configure I2C peripheral channel
    // We're going to use I2C1 since its pins are the ones connected to the I2C interface we're using
    // To configure/instantiate serial peripheral channel we have two options:
    // Use the i2c device peripheral handle and instantiate a transmitter instance using an extension trait
    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );
    // Or use the I2C abstraction
    // let mut i2c = I2c::new(
    //     dp.I2C1,
    //     (scl, sda),
    //     Mode::Standard {
    //         frequency: 300.kHz(),
    //     },
    //     &clocks,
    // );.


    let mut delay = dp.TIM1.delay_ms(&clocks);


    // PCF8574 I2C Address
    const PCF8574_ADDR: u8 = 0x27;    
    let clear_display = 0x01; //*********rs pin********
    let return_home = 0x02;//cursor comes to original position// ******rw pin*******
    let entry_mode_set = 0x06;//assign cursor moving direction and enable the shift of entire display
    let display_on_off= 0x0F;
    let cursor_display_shift = 0x1C;//set cursor moving and display shift control bit, and the direction without changing of DDRAM data 
    let function_set = 0x28;//******D5 pin ********//set interface data length 4bit or 8 bit
    let cg_ram_address = 0x40;//  ****** D6 pin******//CGRAM address usage is a way of creating custom characters and patterns on an LCD display
    let dd_ram_address = 0x80;//*******D7 pin ******//DD-RAM address usage is a way of accessing the data stored in the display data RAM

//DDRAM is used to store the data that is displayed on the LCD screen, 
//while CGRAM is used to store the data that defines the shape of custom characters and symbols.


    let output_config: u8 = 0x00; // let rs = 0x00;     let EntryLeft = 0x00;       let BlinkOff = 0x00;
    // Send data 4 bits at the time
    let Bit4 = 0x00;
    const D4: u8 = 0x10;        //let CursorShift = 0x10;
    // Send data 8 bits at the time
    let Bit8 = 0x10;

    i2c.write(PCF8574_ADDR, &[function_set]);
    delay.delay_ms(10_u32);
    i2c.write(PCF8574_ADDR, &[display_on_off]);
    delay.delay_ms(10_u32);
    i2c.write(PCF8574_ADDR, &[clear_display]);
    delay.delay_ms(10_u32);
    i2c.write(PCF8574_ADDR, &[entry_mode_set]);
    delay.delay_ms(10_u32);
    i2c.write(PCF8574_ADDR, &[return_home]);
    delay.delay_ms(10_u32);



    i2c.write(PCF8574_ADDR, &[cg_ram_address]);
    delay.delay_ms(10_u32);
    i2c.write(PCF8574_ADDR, &[dd_ram_address]);
    delay.delay_ms(10_u32);





    loop{



    i2c.write(PCF8574_ADDR, &[cursor_display_shift]);
    delay.delay_ms(10_u32);

        // let x= 0xFF;
        // i2c.write(PCF8574_ADDR, &[x]).unwrap();
        // The number you want to display
        let d1=1;
        let d2=2;

// Define custom characters for digits (if needed)


// Send commands to display the first digit
        let _= i2c.write(PCF8574_ADDR, &[dd_ram_address | 0x00, d1]).unwrap();
        delay.delay_ms(100_u32);
        i2c.write(PCF8574_ADDR, &[d1]).ok();
        defmt::println!("middle order thammudu");


// Send commands to display the second digit
        let _= i2c.write(PCF8574_ADDR, &[dd_ram_address | 0x01, d2]).unwrap();
        delay.delay_ms(100_u32);
        defmt::println!("print cheyyara ungamma");
    }
}







    


//RS => 1, characters  => data register is delected
//RS => 0, numbeers  => instructtion register is selected




    
 