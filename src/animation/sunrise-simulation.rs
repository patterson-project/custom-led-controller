use rs_ws281x::ControllerBuilder;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::StripType;
use rs_ws281x::Controller;
use rand::{thread_rng, Rng};

pub fn main() {
    let mut controller = ControllerBuilder::new()
    .freq(800_000)
    .dma(10)
    .channel(
        0, // Channel Index
        ChannelBuilder::new()
            .pin(18) // GPIO 10 = SPI0 MOSI
            .count(100) // Number of LEDs
            .strip_type(StripType::Ws2812)
            .brightness(255) // default: 255
            .invert(false)
            .build(),
    )
    .build()
    .unwrap();

    // Phase 7: Sunrise Simulation
    for brightness in 1..=255u8 {
        let leds = controller.leds_mut(0);
        // Use a non-linear scaling for brightness to enhance the sunrise effect
        let scaled_brightness = (brightness as u32).pow(2) / 255; // Squaring the brigh>
        for led in leds.iter_mut() {
            // Adjust the color calculation for a smoother transition
            // Ensure the red component starts low and increases more noticeably
            let red = (3 * scaled_brightness / 4) as u8; // Increase the red component >
            let green = (scaled_brightness / 6) as u8; // Keep the green component lowe>
            *led = [green, 0, red, 0]; // Adjusted color: from dark red to bright yellow
        }
        controller.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(471)); // Testing speed, adj>
    }

    for red in 192..=255u8 {
        let leds = controller.leds_mut(0);
        for led in leds.iter_mut() {
            let red = red as u8;
            *led = [42, 0, red, 0];
        }
        controller.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(938)); // Testing speed, adj>
    }

    for i in 0..=213 { // The range is set to accommodate the slower transition of blue
        let leds = controller.leds_mut(0);
        let green = if i + 42 <= 255 { i + 42 } else { 255 }; // Green starts at 42 and ends at 255
        let blue = (i as f32 / 213.0 * 185.0) as u8; // Blue transitions from 0 to 210 over the same period

        for led in leds.iter_mut() {
            *led = [green, blue, 255, 0]; // Update LED colors with synchronized green and blue values
        }
        controller.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(281)); // Adjust speed as needed
    }
}