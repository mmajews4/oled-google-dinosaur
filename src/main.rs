// Created 10 March 2024
// Imitating the Google dinosaur game on oled screen with arduino 
#![no_std]
#![no_main]

use panic_halt as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},

    mono_font::{ascii::FONT_5X7, MonoTextStyleBuilder},
    text::{Baseline, Text},
};

const DINOSAUR: [u8; 26] = [
    0b00000000,0b00111110,
    0b00000000,0b01101111,
    0b00000000,0b01111111,
    0b00000000,0b01111111,
    0b00000000,0b01110000,
    0b10000000,0b01111110,
    0b10000001,0b11110000,
    0b11000011,0b11110000,
    0b01100111,0b11111100,
    0b01111111,0b11110100,
    0b00111111,0b11110000,
    0b00011111,0b11100000,
    0b00001111,0b11000000,
];

const LEGS_1: [u8; 3] = [
    0b10001000,
    0b11001000,
    0b00001100,
];
const LEGS_2: [u8; 3] = [
    0b11000110,
    0b10000000,
    0b11000000,
];
const LEGS_JUMP: [u8; 3] = [
    0b11001000,
    0b10001000,
    0b11001100,
];
const COVER: [u8; 22] = [
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
    0b00000000,0b00000000,
];
const BIG_CACTUS: [u8; 32] = [
    0b00011000,0b00000000,
    0b00011000,0b00000000,
    0b11011000,0b00000000,
    0b11011000,0b00000000,
    0b11011011,0b00000000,
    0b11011011,0b00000000,
    0b11111011,0b00000000,
    0b01111011,0b00000000,
    0b00011111,0b00000000,
    0b00011110,0b00000000,
    0b00011000,0b00000000,
    0b00011000,0b00000000,
    0b00011000,0b00000000,
    0b00011000,0b00000000,
    0b00011000,0b00000000,
    0b00011000,0b00000000,
];
const CACTUS_COVER: [u8; 6] = [
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
];


#[arduino_hal::entry]
fn main() -> ! {
    // Perypherials initiations
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();

    let sda = pins.a4.into_pull_up_input();
    let scl = pins.a5.into_pull_up_input();

    let i2c = arduino_hal::i2c::I2c::new(dp.TWI, sda, scl, 50000);

    let button = pins.d12.into_floating_input();

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, ssd1306::size::DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();


    // Images initiations for oled screen
    let mut dino_pos: i32 = 30;

    let dinosaur_raw: ImageRaw<BinaryColor> = ImageRaw::new(&DINOSAUR, 16);
    let dino = Image::new(&dinosaur_raw, Point::new(0, dino_pos));

    let legs_1_raw: ImageRaw<BinaryColor> = ImageRaw::new(&LEGS_1, 8);
    let legs_1 = Image::new(&legs_1_raw, Point::new(0, dino_pos+13));

    let legs_2_raw: ImageRaw<BinaryColor> = ImageRaw::new(&LEGS_2, 8);
    let legs_2 = Image::new(&legs_2_raw, Point::new(0, dino_pos+13));

    let legs_jump_raw: ImageRaw<BinaryColor> = ImageRaw::new(&LEGS_JUMP, 8);
    let legs_jump = Image::new(&legs_jump_raw, Point::new(0, dino_pos+23));

    let cover_raw: ImageRaw<BinaryColor> = ImageRaw::new(&COVER, 16);
    let big_cactus_raw: ImageRaw<BinaryColor> = ImageRaw::new(&BIG_CACTUS, 16);
    let cactus_cover_raw: ImageRaw<BinaryColor> = ImageRaw::new(&CACTUS_COVER, 8);

    let mut alternating_legs: u8 = 0;

    let mut cactus_pos: i32 = 128;

    arduino_hal::delay_ms(1000);

    display.flush().unwrap();

    loop {
    
        // Drawing cactus
        Image::new(&big_cactus_raw, Point::new(cactus_pos, dino_pos))
        .draw(&mut display)
        .unwrap();

        // Updating cactus position
        if cactus_pos > -8 {
            cactus_pos = cactus_pos - 2;
        } else {
            cactus_pos = 128;
        }

        // Dinosaur animation when button not pressed
        if button.is_low() {

            led.set_low();

            Image::new(&dinosaur_raw, Point::new(4, dino_pos))
            .draw(&mut display)
            .unwrap();

            if alternating_legs < 2
            {
                Image::new(&legs_1_raw, Point::new(8, dino_pos+13))
                .draw(&mut display)
                .unwrap();

                alternating_legs = alternating_legs + 1;
            }
            else
            {    
                Image::new(&legs_2_raw, Point::new(8, dino_pos+13))
                .draw(&mut display)
                .unwrap();

                alternating_legs = alternating_legs + 1;
                if alternating_legs == 3 {
                    alternating_legs = 0;
                }
            }
            display.flush().unwrap();
            
        }
        else // Animating jump
        {
            led.set_high();

            // jump profile
            let offset_array = [7,11,14,17,19,20,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,20,19,17,14,11,7,0];
            let mut counter: u8 = 0;

            for offset in offset_array {
             
                if counter < 8 {
                    Image::new(&cover_raw, Point::new(4, dino_pos-offset+13))
                    .draw(&mut display)
                    .unwrap();
                } else if counter > 22 {
                    Image::new(&cover_raw, Point::new(4, dino_pos-offset-8))
                    .draw(&mut display)
                    .unwrap();
                }
                counter = counter + 1;

                // Updating cactus whille dinosaur is jumping
                Image::new(&big_cactus_raw, Point::new(cactus_pos, dino_pos))
                .draw(&mut display)
                .unwrap();  

                Image::new(&dinosaur_raw, Point::new(4, dino_pos-offset))
                .draw(&mut display)
                .unwrap();

                Image::new(&legs_jump_raw, Point::new(8, dino_pos-offset+13))
                .draw(&mut display)
                .unwrap(); 
        
                if cactus_pos > -8 {
                    cactus_pos = cactus_pos - 2;
                }

                display.flush().unwrap();
            }
        }
    }
}