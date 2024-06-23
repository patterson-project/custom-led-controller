use rs_ws281x::ControllerBuilder;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::StripType;
use rs_ws281x::Controller;
use std::sync::{Once, Mutex};
mod colors;
use colors_transform::Hsl;
use colors_transform::Color;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use rand::{thread_rng, Rng};

static INIT: Once = Once::new();
static mut STRIP: Option<Mutex<Controller>> = None;

pub struct LastState {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
static mut LAST_STATE: Option<LastState> = None;
static mut LAST_STATE_BRIGHTNESS: u8 = 255;

static ANIMATION_RUNNING: AtomicBool = AtomicBool::new(false);

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

pub fn sunrise() {
    thread::spawn(move || {
        if ANIMATION_RUNNING.load(Ordering::SeqCst) {
            let mut controller = unsafe {
                STRIP.as_ref().unwrap().lock().unwrap()
            };

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
                    if !ANIMATION_RUNNING.load(Ordering::SeqCst) {
                        return;
                    }
                }
                controller.render().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(471)); // Testing speed, adj>
            }
        
            for red in 192..=255u8 {
                let leds = controller.leds_mut(0);
                for led in leds.iter_mut() {
                    let red = red as u8;
                    *led = [42, 0, red, 0];
                    if !ANIMATION_RUNNING.load(Ordering::SeqCst) {
                        return;
                    }
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
                    if !ANIMATION_RUNNING.load(Ordering::SeqCst) {
                        return;
                    }
                }
                controller.render().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(281)); // Adjust speed as needed
            }
        }
    });
}

pub fn aurora_borealis() {
    thread::spawn(move || {
        while ANIMATION_RUNNING.load(Ordering::SeqCst) {
            // Initialize all LEDs to off within a separate scope
            let mut controller = unsafe {
                STRIP.as_ref().unwrap().lock().unwrap()
            };
            {
                let leds = controller.leds_mut(0);
                for led in leds.iter_mut() {
                    *led = [0, 0, 0, 0];
                }
            } // Mutable borrow of `controller` through `leds` ends here
            let led_count = controller.leds_mut(0).len();

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
                if !ANIMATION_RUNNING.load(Ordering::SeqCst) {
                    return;
                }
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
    });
}

pub fn opening_animation() {
    thread::spawn(move || {
        if ANIMATION_RUNNING.load(Ordering::SeqCst) {
            let mut controller = unsafe {
                STRIP.as_ref().unwrap().lock().unwrap()
            };
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
    });
}

pub fn starry_night() {
    thread::spawn(move || {
        while ANIMATION_RUNNING.load(Ordering::SeqCst) {
            let mut controller = unsafe {
                STRIP.as_ref().unwrap().lock().unwrap()
            };
            let led_count = controller.leds_mut(0).len();
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

                if !ANIMATION_RUNNING.load(Ordering::SeqCst) {
                    return;
                }
                controller.render().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(500)); // Testing speed, adjust to 500ms for slower, calming effect
            }
        }
    });
}

pub fn rainbow() {
    thread::spawn(move || {
        while ANIMATION_RUNNING.load(Ordering::SeqCst) {
            let mut controller = unsafe {
                STRIP.as_ref().unwrap().lock().unwrap()
            };
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

                if !ANIMATION_RUNNING.load(Ordering::SeqCst) {
                    return;
                }
        
                controller.render().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(50)); // Adjust for desired speed
            }
        }
    });
}

pub fn start_animation(animation_id: u8) {
    stop_animation();

    ANIMATION_RUNNING.store(true, Ordering::SeqCst);

    match animation_id {
        1 => sunrise(),
        2 => aurora_borealis(),
        3 => opening_animation(),
        4 => starry_night(),
        5 => rainbow(),
        _ => (),
    }
}

pub fn stop_animation() {
    ANIMATION_RUNNING.store(false, Ordering::SeqCst);
}

pub async fn strip_on() {
    let mut controller = unsafe {
        STRIP.as_ref().unwrap().lock().unwrap()
    };
    
    let color = unsafe {
        LAST_STATE.as_ref().unwrap_or(&LastState {
            r: 255,
            g: 255,
            b: 255,
        })
    };

    let brightness = unsafe {
        LAST_STATE_BRIGHTNESS
    };
    
    controller.set_brightness(0, brightness);

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [color.g , color.b, color.r, 100];
    }

    controller.render().unwrap();
}

pub async fn strip_off () {
    stop_animation();
    let mut controller = unsafe {
        STRIP.as_ref().unwrap().lock().unwrap()
    };

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }

    controller.render().unwrap();
}

pub fn strip_set_brightness(brightness: i32) {
    stop_animation();
    let mut controller = unsafe {
        STRIP.as_ref().unwrap().lock().unwrap()
    };

    let u_brightness = brightness as u8;

    unsafe {
        LAST_STATE_BRIGHTNESS = u_brightness;
    }

    controller.set_brightness(0,u_brightness);
    controller.render().unwrap();
}



pub fn strip_set_hsv(h: f32, s: f32, v: f32) {
    stop_animation();
    let mut controller = unsafe {
        STRIP.as_ref().unwrap().lock().unwrap()
    };

    let hex_color = Hsl::from(h,s,v);
    let rgb = hex_color.to_rgb();

    let r = rgb.get_red() as u8;
    let g = rgb.get_green() as u8;
    let b = rgb.get_blue() as u8;

    unsafe {
        LAST_STATE = Some(LastState {
            r: r,
            g: g,
            b: b,
        });
    }

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [g, b, r, 100];
    }

    controller.render().unwrap();
}

pub fn strip_set_temperature(temperature: i32) {
    stop_animation();
    let mut controller = unsafe {
        STRIP.as_ref().unwrap().lock().unwrap()
    };

    let (r,g,b) = colors::convert_k_to_rgb(colors::convert_ha_temperature(temperature));

    unsafe {
        LAST_STATE = Some(LastState {
            r: r,
            g: g,
            b: b,
        });
    }

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [g, b, r, 100];
    }

    controller.render().unwrap();
}
