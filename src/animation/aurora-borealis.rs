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

    for _ in 0..20 { // Run the animation for 50 iterations
        let leds = controller.leds_mut(0);
        for i in 0..led_count {
            let color = match i % 3 {
                0 => [0, 255, 0, 0], // Green
                1 => [0, 0, 255, 0], // Blue
                _ => [128, 0, 128, 0], // Purple
            };
            leds[i] = color;
        }
        controller.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200)); // Testing speed, adjust to 1000ms for slower, mesmerizing effect
    }
}