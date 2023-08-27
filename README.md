# BH1750_stm32f411



********************update code***********************
#![no_std]
#![no_main]

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
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // I2C Config steps:
    // 1) Need to configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 2) Configure/Define SCL and SDA pins
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8;
    let sda = gpiob.pb9;
    // 3) Configure I2C peripheral channel
    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );

    // PCF8574 I2C Address
    const PCF8574_ADDR: u8 = 0x27;
    const RS: u8 = 0x01;
    const EN_MASK: u8 = 0x04;
    const D4: u8 = 0x10;
    const D5: u8 = 0x20;
    const D6: u8 = 0x40;
    const D7: u8 = 0x80;

    // Set RS pin low (command mode)
    i2c.write(PCF8574_ADDR, &[0]).unwrap();
    // Set EN pin high (pulse)
    i2c.write(PCF8574_ADDR, &[EN_MASK]).unwrap();
    // Set EN pin low
    i2c.write(PCF8574_ADDR, &[0]).unwrap();

    // Initialize the LCD (4-bit mode)
    let init_commands: [u8; 5] = [0x20 | D5, 0x20 | D5, 0x20 | D4, 0x20 | D4, 0x80];
    i2c.write(PCF8574_ADDR, &init_commands).unwrap();

    // Clear the display
    let clear_display: [u8; 2] = [0x00, 0x01];
    i2c.write(PCF8574_ADDR, &clear_display).unwrap();

    // Set cursor to the first row
    let set_cursor_row1: [u8; 2] = [0x00, 0x80];
    i2c.write(PCF8574_ADDR, &set_cursor_row1).unwrap();

    // Display text on the first row
    let text_row1: [u8; 21] = [
        0x40, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'E', b'm', b'b', b'e', b'd', b'd', b'e',
        b'd', b' ', b'R', b'u', b's', b't',
    ];
    i2c.write(PCF8574_ADDR, &text_row1).unwrap();

//RS => 1, characters
//RS => 0, numbeers

    // Application Loop
    loop {
        // Set all pins of the PCF8574 as outputs
        // Each bit of the data byte corresponds to a pin on the PCF8574.
        // Set a bit to 0 to configure the corresponding pin as an output.
        // In this example, all pins are set as outputs, so we set the data byte to 0x00.
        //i2c.write(PCF8574_ADDR, &[rs_pin_mask]).unwrap();
        defmt::println!("check-1");
        
        // i2c.write(PCF8574_ADDR, &[output_config]).unwrap();
        defmt::println!("check-2");
        //delay.delay_ms(1000_u32); // Wait for 1 second

        // Toggle all pins of the PCF8574
        // To toggle the pins, we first read the current state of the GPIO pins,
        // then complement the bits (1s to 0s and 0s to 1s) and write back the new state.
        let mut input_buffer=[0;1];
        defmt::println!("check-3");
        i2c.read(PCF8574_ADDR, &mut input_buffer).unwrap();

        defmt::println!("{:x}",input_buffer);

        let current_state = input_buffer[0];
        let new_state = !current_state;
        i2c.write(PCF8574_ADDR, &[new_state]).unwrap();
       // delay.delay_ms(1000_u32); // Wait for 1 second

        let init_commands: [u8; 5] = [0x00, 0x38, 0x00, 0x06, 0x0C];
        i2c.write(PCF8574_ADDR, &init_commands).unwrap();
    

        let clear_display: [u8; 2] = [0x00, 0x01];
        i2c.write(PCF8574_ADDR, &clear_display).unwrap();
        defmt::println!("check 4- Gopi");
        // Your name "Santosh"
        let name: [u8; 12] = [0x40, b'S', b'a', b'n', b't', b'o', b's', b'h',b' ',b' ',b'i',b's'];
    
        // Send your name to the LCD
        i2c.write(PCF8574_ADDR, &name).unwrap();

        // Set cursor to the second row
        let set_cursor_row2: [u8; 2] = [0x00, 0xC0];
        i2c.write(PCF8574_ADDR, &set_cursor_row2).unwrap();
        defmt::println!("check 5- Gopi vemula");
        // Display text on the second row
        let text_row2: [u8; 9] = [0x40, b'I', b'n', b'n', b'o', b'c', b'e', b'n',b't'];
        i2c.write(PCF8574_ADDR, &text_row2).unwrap();
        defmt::println!("finish");
    }
}








**************************************************************





#![no_std]
#![no_main]

use stm32f4xx_hal::{
    pac,
    i2c::Mode,
    prelude::*,
    
};
// stm32,
// delay::Delay,
use embedded_hal::blocking::delay::DelayMs;
use pcf857x::Pcf8574;
use cortex_m_rt::entry;
extern crate core;
#[entry]

fn main() -> !{


    #[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // Customize panic behavior here
    loop {}
}



    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // let gpioa = dp.GPIOA.split();
    // let scl = gpioa.pa8.into_alternate().set_open_drain();
    // let sda = gpioa.pa9.into_alternate().set_open_drain();

    // let i2c = I2c::new(
    //     dp.I2C1,
    //     (scl, sda),
    //     Mode::standard(100.khz()),
    //     cp.SYST,
    //     &mut rcc.apb1,
    //     100_000,
    // );

    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8;
    let sda = gpiob.pb9;
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    let mut i2c = dp.I2C1.i2c(
        (scl,sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );

    
    let mut delay = dp.TIM1.delay_ms(&clocks);
    // let mut delay = Delay::new(cp.SYST);

    let mut display = Pcf8574::new(i2c, 0x27);

    loop {
        #[panic_handler]
        display.write(b"Hello, World!").unwrap();
        delay.delay_ms(1000u16);
    }

    loop{};
}





commands ra thammudu:

&&&&&&&&&&  commands intitalization &&&&&&&&&&&&
void lcd_init (void)
{
	// 4 bit initialisation
	usleep(50000);  // wait for >40ms
	lcd_send_cmd (0x30);
	usleep(4500);  // wait for >4.1ms
	lcd_send_cmd (0x30);
	usleep(200);  // wait for >100us
	lcd_send_cmd (0x30);
	usleep(200);
	lcd_send_cmd (0x20);  // 4bit mode
	usleep(200);

  // dislay initialisation
	lcd_send_cmd (0x28); // Function set --> DL=0 (4 bit mode), N = 1 (2 line display) F = 0 (5x8 characters)
	usleep(1000);
	lcd_send_cmd (0x08); //Display on/off control --> D=0,C=0, B=0  ---> display off
	usleep(1000);
	lcd_send_cmd (0x01);  // clear display
	usleep(1000);
	usleep(1000);
	lcd_send_cmd (0x06); //Entry mode set --> I/D = 1 (increment cursor) & S = 0 (no shift)
	usleep(1000);
	lcd_send_cmd (0x0C); //Display on/off control --> D = 1, C and B = 0. (Cursor and blink, last two bits)
	usleep(2000);
}
