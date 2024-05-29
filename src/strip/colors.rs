use std::f32::consts::E;


pub fn convert_ha_temperature(mut temperature: i32) -> i32 {
    if temperature < 154 {
        temperature = 154;
    } else if temperature >= 400 {
        temperature = 346;
    }

    let final_temperature = if temperature > 250 {
        1960 - (1960 - ((temperature * 10) - 540)).abs()
    } else {
        1960 + (1960 - ((temperature * 10) - 540)).abs()
    };

    final_temperature
}

pub fn convert_k_to_rgb(mut colour_temperature: i32) -> (u8, u8, u8) {
    if colour_temperature < 1000 {
        colour_temperature = 1000;
    } else if colour_temperature > 40000 {
        colour_temperature = 40000;
    }

    let tmp_internal = colour_temperature as f32 / 100.0;

    let red = if tmp_internal <= 66.0 {
        255
    } else {
        let tmp_red = 329.698727446 * (tmp_internal - 60.0).powf(-0.1332047592);
        tmp_red.max(0.0).min(255.0) as u8
    };

    let green = if tmp_internal <= 66.0 {
        let tmp_green = 99.4708025861 * (tmp_internal).ln() - 161.1195681661;
        tmp_green.max(0.0).min(255.0) as u8
    } else {
        let tmp_green = 288.1221695283 * (tmp_internal - 60.0).powf(-0.0755148492);
        tmp_green.max(0.0).min(255.0) as u8
    };

    let blue = if tmp_internal >= 66.0 {
        255
    } else if tmp_internal <= 19.0 {
        0
    } else {
        let tmp_blue = 138.5177312231 * (tmp_internal - 10.0).ln() - 305.0447927307;
        tmp_blue.max(0.0).min(255.0) as u8
    };

    (red, green, blue)
}