#![deny(warnings)]

use embedded_graphics::{coord::Coord, fonts::Font6x8, prelude::*, Drawing};
use embedded_hal::prelude::*;
use epd_waveshare::{
    epd2in13::{Display2in13, EPD2in13},
    graphics::{Display, DisplayRotation},
    prelude::*,
};
use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    sysfs_gpio::Direction,
    Delay, Pin, Spidev,
};

// activate spi, gpio in raspi-config
// needs to be run with sudo because of some sysfs_gpio permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

//TODO: Test this implemenation with a new display
fn main() {
    if let Err(e) = run() {
        eprintln!("Program exited early with error: {}", e);
    }
}

fn run() -> Result<(), std::io::Error> {
    // Configure Delay
    let mut delay = Delay {};

    // Setup of the needed pins is finished here
    // Now the "real" usage of the eink-waveshare-rs crate begins
    let mut epd = EPD2in13::new(&mut spi, cs_pin, busy, dc, rst, &mut delay)?;

    // Clear the full screen
    epd.clear_frame(&mut spi).expect("clear frame 1");
    epd.display_frame(&mut spi).expect("disp 1");

    println!("Test all the rotations");
    let mut display = Display2in13::default();
    epd.update_frame(&mut spi, display.buffer()).unwrap();
    epd.display_frame(&mut spi).expect("display frame x03");

    display.set_rotation(DisplayRotation::Rotate0);
    display.draw(
        Font6x8::render_str("Rotate 0!")
            .with_stroke(Some(Color::Black))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );

    display.set_rotation(DisplayRotation::Rotate90);
    display.draw(
        Font6x8::render_str("Rotate 90!")
            .with_stroke(Some(Color::Black))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );

    display.set_rotation(DisplayRotation::Rotate180);
    display.draw(
        Font6x8::render_str("Rotate 180!")
            .with_stroke(Some(Color::Black))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );

    display.set_rotation(DisplayRotation::Rotate270);
    display.draw(
        Font6x8::render_str("Rotate 270!")
            .with_stroke(Some(Color::Black))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );

    // Display updated frame
    epd.update_frame(&mut spi, &display.buffer()).unwrap();
    epd.display_frame(&mut spi)
        .expect("display frame new graphics");
    delay.delay_ms(5000u16);

    // a quickly moving `Hello World!`
    display.set_rotation(DisplayRotation::Rotate0);
    epd.set_lut(&mut spi, Some(RefreshLUT::QUICK))
        .expect("SET LUT QUICK error");
    let limit = 20;
    for i in 0..limit {
        println!("Moving Hello World. Loop {} from {}", (i + 1), limit);

        display.draw(
            Font6x8::render_str("  Hello World! ")
                .with_style(Style {
                    fill_color: Some(Color::White),
                    stroke_color: Some(Color::Black),
                    stroke_width: 0u8, // Has no effect on fonts
                })
                .translate(Coord::new(5 + i * 6, 50))
                .into_iter(),
        );

        epd.update_frame(&mut spi, &display.buffer()).unwrap();
        epd.display_frame(&mut spi)
            .expect("display frame new graphics");
    }

    // Set the EPD to sleep
    epd.sleep(&mut spi).expect("sleep");

    Ok(())
}
