use rs_ws281x::ControllerBuilder;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::StripType;
use rs_ws281x::Controller;
use std::sync::{Once, Mutex};
mod colors;
use colors_transform::Hsl;
use colors_transform::Color;
use colors_transform::Rgb;

static INIT: Once = Once::new();
static mut STRIP: Option<Mutex<Controller>> = None;
// static ref LAST_COLORS: Mutex<Vec<(i32,i32,i32,i32)>> = Mutex::new(Vec::new());

pub fn init() {
    INIT.call_once(|| {
        let controller = ControllerBuilder::new()
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

        unsafe {
            STRIP = Some(Mutex::new(controller));
        }
    });

}

pub async fn strip_on() {
    let mut controller = unsafe {
        STRIP.as_ref().unwrap().lock().unwrap()
    };

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 255, 0];
    }

    // let all_leds = controller.leds(0);
    controller.render().unwrap();
}

pub async fn strip_off () {
    let mut controller = unsafe {
        STRIP.as_ref().unwrap().lock().unwrap()
    };

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }

    // let all_leds = controller.leds(0);
    controller.render().unwrap();
}

// pub fn strip_set_brightness(brightness: i32) {
//     let controller = unsafe {
//         STRIP.as_ref().unwrap().lock().unwrap()
//     };

//     controller.set_brightness(0, brightness);
//     controller.render().unwrap();
// }



pub fn strip_set_hsv(h: f32, s: f32, v: f32) {
    let mut controller = unsafe {
        STRIP.as_ref().unwrap().lock().unwrap()
    };

    let hex_color = Hsl::from(h,s,v);
    let rgb = hex_color.to_rgb();
    // let (r, g, b) = colors::hsv_to_rgb(h as f32 / 360.0, s as f32 / 100.0, v as f32 / 100.0);

    let r = rgb.get_red() as u8;
    let g = rgb.get_green() as u8;
    let b = rgb.get_blue() as u8;

    println!("Setting color to: {}, {}, {}", r, g, b);

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [g, b, r, 100];
    }

    let all_leds = controller.leds(0);
    controller.render().unwrap();
}

// pub fn strip_set_temperature(temperature: i32) {
//     let controller = unsafe {
//         STRIP.as_ref().unwrap().lock().unwrap()
//     };

//     let r,g,b = colors::convert_k_to_rgb(colors::convert_ha_temperature(temperature));

//     let leds = controller.leds_mut(0);
//     for led in leds {
//         *led = [r, g, b, 0];
//     }

//     let all_leds = controller.leds(0);
//     controller.render().unwrap();
// }


// pub fn main() {
//     let mut controller = ControllerBuilder::new()
//     .freq(800_000)
//     .dma(10)
//     .channel(
//         0, // Channel Index
//         ChannelBuilder::new()
//             .pin(18) // GPIO 10 = SPI0 MOSI
//             .count(100) // Number of LEDs
//             .strip_type(StripType::Ws2812)
//             .brightness(255) // default: 255
//             .invert(false)
//             .build(),
//     )
//     .build()
//     .unwrap();

//     let leds = controller.leds_mut(0);

//     for led in leds {
//         *led = [0, 0, 255, 0];
//     }

//     let all_leds = controller.leds(0);

//     controller.render().unwrap();
// }