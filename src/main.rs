#![no_std]
#![no_main]

// Imports
use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
    serial::config::Config,
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
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
    // 3) Configure I2C peripheral channel
    // We're going to use I2C1 since its pins are the ones connected to the I2C interface we're using
    // To configure/instantiate serial peripheral channel we have two options:
    // Use the i2c device peripheral handle and instantiate a transmitter instance using extension trait
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
    // );

    // Serial config steps:
    // 1) Need to configure the system clocks
    // Already done earlier for I2C module
    // 2) Configure/Define TX pin
    // Use PA2 as it is connected to the host serial interface
    let gpioa = dp.GPIOA.split();
    let tx_pin = gpioa.pa2.into_alternate();
    // 3) Configure Serial peripheral channel
    // We're going to use USART2 since its pins are the ones connected to the USART host interface
    // To configure/instantiate serial peripheral channel we have two options:
    // Use the device peripheral handle to directly access USART2 and instantiate a transmitter instance
    let mut tx = dp
        .USART2
        .tx(
            tx_pin,
            Config::default()
                .baudrate(9600.bps())
                .wordlength_8()
                .parity_none(),
            &clocks,
        )
        .unwrap();

    let mut delay = dp.TIM1.delay_ms(&clocks);

    // BH1750 I2C Address
    const BH1750_ADDR: u8 = 0x23;
    // const POWER_DOWN: u8 = 0x00;
    const POWER_ON: u8 = 0x01;
    // const RESET: u8 = 0x07;s
    const CONTINUOUS_HIGH_RES_MODE: u8 = 0x10;

    let mut rx_buffer: [u8; 2] = [0; 2];
    let mut rx_word: u16;

    // Power on the sensor
    i2c.write(BH1750_ADDR, &[POWER_ON]).unwrap();
    delay.delay_ms(10_u32); // Wait for power-on initialization

    // Set the measurement mode
    i2c.write(BH1750_ADDR, &[CONTINUOUS_HIGH_RES_MODE]).unwrap();
    delay.delay_ms(120_u32); // Wait for measurement stabilization

    // Application Loop
    loop {
        // Read light intensity data
        i2c.read(BH1750_ADDR, &mut rx_buffer).unwrap();
        rx_word = ((rx_buffer[0] as u16) << 8) | rx_buffer[1] as u16;
        
        // Convert light intensity data to lux (value format as per datasheet)
        let lux = (rx_word as f32 / 1.2) as u16;
        
        // Print light intensity in lux
        writeln!(tx, "Light Intensity = {} lux\r", lux).unwrap();
        
        delay.delay_ms(1000_u32); // Wait for 1 second before reading again
    }
}
