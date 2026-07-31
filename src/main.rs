use std::io::{self, Write};

enum Action {
    None,
    HuToCm,
    CmToHu,
    CelToFah,
    FahToCel,
    CelToKel,
    KelToCel,
    NetherToOverworld,
    OverworldToNether,
}

struct ConvResult {
    value: String,
    type_m: String,
}

struct McCoordinates{
    x: f64, y: f64, z: f64,
}

fn main() {
    let mut response = String::new();
    println!("╔════════════════════════════════════════╗");
    println!("║ Ferrous Converter                      ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    loop {
        #[allow(unused_assignments, unused_variables)]
        let action = Action::None;
        response.clear();
        println!(" » Select conversion type(q to quit):");
        println!("  [1] Hammer Unit     -> Centimeter");
        println!("  [2] Centimeter      -> Hammer Unit");
        println!("  [3] Celsius         -> Fahrenheit");
        println!("  [4] Fahrenheit      -> Celsius");
        println!("  [5] Celsius         -> Kelvin");
        println!("  [6] Kelvin          -> Celsius");
        println!("  [7] Overworld       -> Nether");
        println!("  [8] Nether          -> Overworld");

        print!(" Enter index »» ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut response).expect("Something went wrong");
        if response.trim() == "q" { std::process::exit(0) }

        let index = response.trim().parse::<i32>();
        let index: i32 = match index{
            Ok(index)  => index,
            Err(_) => {
                println!("Failed to parse the number. Did you enter something besides a number?");
                continue;
            }
        };
        let action = match index {
            1   => Action::HuToCm,
            2   => Action::CmToHu,
            3   => Action::CelToFah,
            4   => Action::FahToCel,
            5   => Action::CelToKel,
            6   => Action::KelToCel,
            7   => Action::OverworldToNether,
            8   => Action::NetherToOverworld,
            _   => Action::None,
        };
        
        match action {
            Action::None => {
                println!("No action is selected");
                continue;
            },
            Action::HuToCm => {
                let value = get_single_value(&mut response);
                hu_to_cm(&value);
            }
            Action::CmToHu => {
                let value = get_single_value(&mut response);
                cm_to_hu(&value);
            }
            Action::CelToFah => {
                let value = get_single_value(&mut response);
                c_to_f(&value);
            }
            Action::FahToCel => {
                let value = get_single_value(&mut response);
                f_to_c(&value); 
            }
            Action::CelToKel => {
                let value = get_single_value(&mut response);
                c_to_k(&value);
            }
            Action::KelToCel => {
                let value = get_single_value(&mut response);
                k_to_c(&value);
            }
            Action::OverworldToNether => {
                let coords = get_coordinates(&mut response);
                overworld_to_nether(&coords);
            }
            Action::NetherToOverworld => {
                let coords = get_coordinates(&mut response);
                nether_to_overworld(&coords);
            }
        }
    }
}

fn get_single_value(response: &mut String) -> f64{
    response.clear();
    print!("Enter value (float 64bit precision) »» ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(response).expect("Couldn't read line.");

    let response = response.trim().parse::<f64>();
    match response {
        Ok(value)  => value,
        Err(_) => {
            println!("Failed to parse the number. Returning 0. Did you enter something besides a number?");
            0_f64
        }
    }
}

fn get_coordinates(response: &mut String) -> McCoordinates {
    loop {
        response.clear();
        println!("Enter coordinates (delimit with whitespace)");
        print!("    x y z »» ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(response).expect("Couldn't read line");
        
        // core::str:split_whitespace() automatically looks for any whitespaces, including \n,
        // so no .trim() is needed
        let mut iter = response.split_whitespace();
        let x = match iter.next() {
            Some(x) => x,
            None => {
                println!("Invalid input.");
                continue;
            }
        };
        let y = match iter.next() {
            Some(y) => y,
            None => {
                println!("Invalid input.");
                continue;
            }
        };
        let z = match iter.next() {
            Some(z) => z,
            None => {
                println!("Invalid input.");
                continue;
            }
        };
        let x = match x.parse::<f64>() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
        let y = match y.parse::<f64>() {
            Ok(y) => y,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
        let z = match z.parse::<f64>() {
            Ok(z) => z,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
        let coords = McCoordinates { x, y, z };
        // loop {...} never stops on its own, i have to explicitly return.
        return coords
    }
}

fn print_result(res: &ConvResult){
    println!("┌────────────────────────────────────────┐");
    println!("│ {} {}",             res.value, res.type_m);
    println!("└────────────────────────────────────────┘");
    println!();
    let mut blank = String::new();
    print!("Enter anything to proceed ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut blank).expect("Couldn't read line");
    blank.clear();
}

fn hu_to_cm(hu: &f64) {
    // 1 HU == 2.54 CM
    let result = hu * 2.54_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("CM"),
    };
    print_result(&res);
}

fn cm_to_hu(cm: &f64) {
    // 1 HU == 2.54 CM
    let result = cm / 2.54_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("HU"),
    };
    print_result(&res);
}
fn c_to_f(c: &f64) {
    // F = C * 1.8 + 32
    let result = c * 1.8_f64 + 32.0_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°F"),
    };
    print_result(&res);
}

fn f_to_c(f: &f64) {
    // C = (F - 32) / 1.8
    let result = (f - 32.0_f64) / 1.8_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°C"),
    };
    print_result(&res);
}

fn c_to_k(c: &f64) {
    // K = C + 273.15
    let result = c + 273.15_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°K"),
    };
    print_result(&res);
}

fn k_to_c(k: &f64) {
    // C = K - 273.15
    let result = k - 273.15_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°C"),
    };
    print_result(&res);
}

fn overworld_to_nether(coords: &McCoordinates) {
    // divide x and z coordinates by 8
    let result = McCoordinates {
        x: coords.x / 8_f64,
        y: coords.y,
        z: coords.z / 8_f64,
    };
    print_result_mc_coordinates(&result)
}

fn nether_to_overworld(coords: &McCoordinates) {
    // multiply x and z coordinates by 8
    let result = McCoordinates {
        x: coords.x * 8_f64,
        y: coords.y,
        z: coords.z * 8_f64,
    };
    print_result_mc_coordinates(&result);
}

fn print_result_mc_coordinates(coords: &McCoordinates) {
    println!("┌────────────────────────────────────────┐");
    println!("│ x: {}, y: {}, z: {}", coords.x, coords.y, coords.z);
    println!("└────────────────────────────────────────┘");
    println!();
    let mut blank = String::new();
    print!("Enter anything to proceed ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut blank).expect("Couldn't read line");
    blank.clear();
}
