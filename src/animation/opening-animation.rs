use rs_ws281x::ControllerBuilder;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::StripType;
use rs_ws281x::Controller;

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

    let led_count = controller.leds_mut(0).len();
    let custom_mid_point = 60; // Custom midpoint, can be set dynamically

    // Calculate the delay for the second LED to start
    let delay_start = custom_mid_point - (led_count - custom_mid_point);

    // Phase 1: Two LEDs meet in the middle
    for step in 0..custom_mid_point {
        let leds = controller.leds_mut(0);
        // Clear the strip in each iteration to ensure only the intended LEDs are lit
        for led in leds.iter_mut() {
            *led = [0, 0, 0, 0];
        }

        // Light up the first LED from the start
        if step < custom_mid_point {
            leds[step] = [255, 255, 255, 0]; // Left moving LED
        }

        // Start the second LED after the calculated delay
        if step >= delay_start {
            let right_led_position = led_count - (step - delay_start) - 1;
            leds[right_led_position] = [255, 255, 255, 0]; // Right moving LED
        }

        controller.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50)); // Movement speed
    }

    // Ensure the midpoint includes two LEDs, one for each side
    let leds = controller.leds_mut(0);
    leds[custom_mid_point - 1] = [255, 255, 255, 0]; // Adjust for 0-based index
    leds[custom_mid_point] = [255, 255, 255, 0];
    controller.render().unwrap();

    // Phase 2: Fill in the strip from the middle back to the ends
    for step in 1..custom_mid_point { // Start from 1 to avoid re-lighting the midpoint LEDs
        let leds = controller.leds_mut(0);

        // Fill towards the start from the left of the custom midpoint
        if custom_mid_point - step > 0 { // Check to avoid underflow
            leds[custom_mid_point - step - 1] = [255, 255, 255, 0]; // Left side
        }

        // Fill towards the end from the right of the custom midpoint
        if custom_mid_point + step < led_count { // Check to avoid overflow
            leds[custom_mid_point + step] = [255, 255, 255, 0]; // Right side
        }

        controller.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50)); // Movement speed
    }

    // Phase 3: Breathing effect
    for _ in 0..4 {
        let min_brightness = 1; // Very dim but not off
        let max_brightness = 255; // Max brightness
        let step_size = 5; // Adjust for smoother or quicker transitions
    
        // Gradually decrease brightness
        for brightness in (min_brightness..=max_brightness).rev().step_by(step_size) {
            controller.set_brightness(0, brightness); // Apply to channel 0
            controller.render().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(30)); // Adjust speed
        }
    
        // Gradually increase brightness
        for brightness in (min_brightness..=max_brightness).step_by(step_size) {
            controller.set_brightness(0, brightness); // Apply to channel 0
            controller.render().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(30)); // Adjust speed
        }
    }
}
