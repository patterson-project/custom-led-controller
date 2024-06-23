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

        // Phase 4: Starry Night
    // Initialize a vector to track the state and duration of each LED
    let mut led_states = vec![(0, 0); led_count]; // (brightness, duration)

    for _ in 0..50 { // Run the animation for 50 iterations
        let leds = controller.leds_mut(0);

        // Update LED states based on their remaining duration
        for (i, led) in leds.iter_mut().enumerate() {
            if led_states[i].1 > 0 {
                // If the LED has remaining duration, decrease it
                led_states[i].1 -= 1;
                *led = [led_states[i].0, led_states[i].0, led_states[i].0, 0];
            } else {
                // Turn off the LED if its duration has elapsed
                *led = [0, 0, 0, 0];
            }
        }

        // Randomly select a new LED to light up
        let led_index = thread_rng().gen_range(0..led_count);
        // Use a weighted approach for brightness to favor lower values and more off LEDs
        let brightness = if thread_rng().gen_bool(0.3) { // 30% chance to light up an LED
            thread_rng().gen_range(1..=255)
        } else {
            0 // Most LEDs stay off
        };

        if brightness > 0 {
            // Light up the new LED and set its duration
            leds[led_index] = [brightness, brightness, brightness, 0];
            led_states[led_index] = (brightness, 5); // Keep the LED on for 5 iterations
        }

        controller.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100)); // Testing speed, adjust to 500ms for slower, calming effect
    }


}