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
    let mut color = [255, 0, 0, 0]; // Start with dark red

    loop {
        // Transition logic
        if color[0] == 255 && color[1] < 255 && color[2] == 0 { // Red to Yellow
            color[1] += 1;
        } else if color[0] > 0 && color[1] == 255 { // Yellow to Green
            color[0] -= 1;
        } else if color[1] == 255 && color[2] < 255 { // Green to Cyan
            color[2] += 1;
        } else if color[2] == 255 && color[1] > 0 { // Cyan to Blue
            color[1] -= 1;
        } else if color[2] == 255 && color[0] < 255 { // Blue to Purple
            color[0] += 1;
        } else if color[0] == 255 && color[2] > 0 { // Purple to Red
            color[2] -= 1;
        }
        // Apply the current color to all LEDs
        for i in 0..led_count {
            controller.leds_mut(0)[i] = color;
        }

        // Render and wait
        println!("Current color: R: {}, G: {}, B: {}", color[0], color[1], color[2]);
        controller.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50)); // Adjust for desired speed

    }
}
