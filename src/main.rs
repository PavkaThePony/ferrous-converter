mod printer;
mod convert;
mod equestria;
mod get;

use std::io::{self, Write};

const EQUESTRIAN_GRAVITY: f64        = 9.807;   // m/s^2
const DISPLACEMENT_SCALE_FACTOR: f64 = 1000.0;  // teleportation is hard for the world 
const RESISTANCE_FACTOR: f64         = 10.0;

enum Direction {
    Unspecified,
    Vertical,
    Horizontal,
}

enum ObstacleType{
    None,
    WithHoles,
    Solid,
}

enum RestructuringType {
    Creation,
    Cloning,
}

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
    FeetToMeters,
    MilesToKms,
    LightToKms,
    KmsToLight,
    PonyMagicToWatt,
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
    println!(" » Select conversion type(q to quit):");
    println!("  [1] Hammer Unit     -> Centimeter");
    println!("  [2] Centimeter      -> Hammer Unit");
    println!("  [3] Celsius         -> Fahrenheit");
    println!("  [4] Fahrenheit      -> Celsius");
    println!("  [5] Celsius         -> Kelvin");
    println!("  [6] Kelvin          -> Celsius");
    println!("  [7] Overworld       -> Nether");
    println!("  [8] Nether          -> Overworld");
    println!("  [9] Feet            -> Meters");
    println!("  [10] Miles          -> Kilometers");
    println!("  [11] Light Years    -> Kilometers");
    println!("  [12] Kilometers     -> Light Years");
    println!("  [13] Pony Magic     -> Watts");
    loop {
        #[allow(unused_assignments, unused_variables)]
        let action = Action::None;
        response.clear();
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
            9   => Action::FeetToMeters,
            10  => Action::MilesToKms,
            11  => Action::LightToKms,
            12  => Action::KmsToLight,
            13  => Action::PonyMagicToWatt,
            _   => Action::None,
        };
        
        match action {
            Action::None => {
                println!("No action is selected");
                continue;
            },
            Action::HuToCm => {
                let value = get::get_single_value(&mut response);
                convert::hu_to_cm(value);
            }
            Action::CmToHu => {
                let value = get::get_single_value(&mut response);
                convert::cm_to_hu(value);
            }
            Action::CelToFah => {
                let value = get::get_single_value(&mut response);
                convert::c_to_f(value);
            }
            Action::FahToCel => {
                let value = get::get_single_value(&mut response);
                convert::f_to_c(value); 
            }
            Action::CelToKel => {
                let value = get::get_single_value(&mut response);
                convert::c_to_k(value);
            }
            Action::KelToCel => {
                let value = get::get_single_value(&mut response);
                convert::k_to_c(value);
            }
            Action::OverworldToNether => {
                let coords = get::get_coordinates(&mut response);
                convert::overworld_to_nether(&coords);
            }
            Action::NetherToOverworld => {
                let coords = get::get_coordinates(&mut response);
                convert::nether_to_overworld(&coords);
            }
            Action::MilesToKms => {
                let value = get::get_single_value(&mut response);
                convert::miles_to_kms(value);
            }
            Action::FeetToMeters => {
                let value = get::get_single_value(&mut response);
                convert::feet_to_meters(value);
            }
            Action::LightToKms => {
                let value = get::get_single_value(&mut response);
                convert::light_to_kms(value);
            }
            Action::KmsToLight => {
                let value = get::get_single_value(&mut response);
                convert::kms_to_light(value);
            }
            Action::PonyMagicToWatt => {
                equestria::do_earth_scales_to_uups_process();
            }
        }
    }
}
