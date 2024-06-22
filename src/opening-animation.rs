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
    let mid_point = led_count / 2;

// Phase 1: Two LEDs meet in the middle
for step in 0..mid_point {
    {
        let leds = controller.leds_mut(0);
        // Clear the strip in each iteration to ensure only two LEDs are lit
        for led in leds.iter_mut() {
            *led = [0, 0, 0, 0];
        }

        leds[step] = [255, 0, 0, 0]; // Start index LED
        leds[led_count - step - 1] = [0, 255, 0, 0]; // End index LED
    } // Mutable borrow of leds ends here

    controller.render().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50)); // Movement speed
}

// Phase 2: Fill in the strip from the middle back to the ends
for step in 0..mid_point {
    {
        let leds = controller.leds_mut(0);
        // No clearing of the strip, so LEDs remain lit as they fill in
        if step > 0 { // Avoid re-lighting the middle LEDs
            leds[mid_point - step] = [255, 0, 0, 0]; // Filling in towards the start
            leds[mid_point + step - 1] = [0, 255, 0, 0]; // Filling in towards the end
        }
        // Handle the case for odd number of LEDs
        if led_count % 2 != 0 && step == 0 {
            leds[mid_point] = [255, 255, 0, 0]; // Middle LED for odd count
        }
    } // Mutable borrow of leds ends here
    

    controller.render().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50)); // Movement speed
}
}
