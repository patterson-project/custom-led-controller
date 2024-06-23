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

    for _ in 0..20 { // Run the animation for 20 iterations
        // Initialize all LEDs to off within a separate scope
        {
            let leds = controller.leds_mut(0);
            for led in leds.iter_mut() {
                *led = [0, 0, 0, 0];
            }
        } // Mutable borrow of `controller` through `leds` ends here
    
        let start_index = thread_rng().gen_range(0..led_count - 20); // Ensure there's room for 5 LEDs
        let color = match thread_rng().gen_range(0..3) {
            0 => [255, 0, 0, 0], // Green
            1 => [0, 255, 0, 0], // Blue
            _ => [0, 128, 128, 0], // Purple
        };
    
        // Fade in the spot
        for brightness in 1..=10 {
            {
                let leds = controller.leds_mut(0);
                for i in 0..5 {
                    leds[start_index + i] = [
                        (color[0] * brightness / 10) as u8,
                        (color[1] * brightness / 10) as u8,
                        (color[2] * brightness / 10) as u8,
                        0,
                    ];
                }
            } // Mutable borrow of `controller` through `leds` ends here
            controller.render().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    
        // Hold the maximum brightness for a short duration
        std::thread::sleep(std::time::Duration::from_millis(500));
    
        // Fade out the spot
        for brightness in (1..=10).rev() {
            {
                let leds = controller.leds_mut(0);
                for i in 0..5 {
                    leds[start_index + i] = [
                        (color[0] * brightness / 10) as u8,
                        (color[1] * brightness / 10) as u8,
                        (color[2] * brightness / 10) as u8,
                        0,
                    ];
                }
            } // Mutable borrow of `controller` through `leds` ends here
            controller.render().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
}