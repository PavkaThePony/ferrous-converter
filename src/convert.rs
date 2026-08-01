use crate::{printer, ConvResult, McCoordinates};

pub fn hu_to_cm(hu: f64) {
    // 1 HU == 2.54 CM
    let result = hu * 2.54_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("CM"),
    };
    printer::print_result(&res);
}

pub fn cm_to_hu(cm: f64) {
    // 1 HU == 2.54 CM
    let result = cm / 2.54_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("HU"),
    };
    printer::print_result(&res);
}
pub fn c_to_f(c: f64) {
    // F = C * 1.8 + 32
    let result = c * 1.8_f64 + 32.0_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°F"),
    };
    printer::print_result(&res);
}

pub fn f_to_c(f: f64) {
    // C = (F - 32) / 1.8
    let result = (f - 32.0_f64) / 1.8_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°C"),
    };
    printer::print_result(&res);
}

pub fn c_to_k(c: f64) {
    // K = C + 273.15
    let result = c + 273.15_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°K"),
    };
    printer::print_result(&res);
}

pub fn k_to_c(k: f64) {
    // C = K - 273.15
    let result = k - 273.15_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°C"),
    };
    printer::print_result(&res);
}

pub fn overworld_to_nether(coords: &McCoordinates) {
    // divide x and z coordinates by 8
    let result = McCoordinates {
        x: coords.x / 8_f64,
        y: coords.y,
        z: coords.z / 8_f64,
    };
    printer::print_result_mc_coordinates(&result)
}

pub fn nether_to_overworld(coords: &McCoordinates) {
    // multiply x and z coordinates by 8
    let result = McCoordinates {
        x: coords.x * 8_f64,
        y: coords.y,
        z: coords.z * 8_f64,
    };
    printer::print_result_mc_coordinates(&result);
}

pub fn feet_to_meters(feet: f64) {
    // divide by 3.28084
    let result = feet / 3.28084_f64;
    let res = ConvResult {
        value : result.to_string(),
        type_m : String::from("Meters"),
    };
    printer::print_result(&res);
}

pub fn miles_to_kms(miles: f64) {
    // multiply by 1.60934
    let result = miles * 1.60934_f64;
    let res = ConvResult {
        value : result.to_string(),
        type_m : String::from("KM"),
    };
    printer::print_result(&res);
}

pub fn light_to_kms(light_years: f64) {
    // multiply by 9_460_730_472_580.8
    let result = light_years * 9_460_730_472_580.8_f64;
    let res = ConvResult {
        value : result.to_string(),
        type_m : String::from("KM")
    };
    printer::print_result(&res);
}

pub fn kms_to_light(kms: f64) {
    // divide by 9_460_730_472_580.8
    let result = kms / 9_460_730_472_580.8_f64;
    let res = ConvResult {
        value : result.to_string(),
        type_m : String::from("Light Years"),
    };
    printer::print_result(&res);
}

