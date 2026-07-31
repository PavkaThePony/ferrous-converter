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
                let value = get_single_value(&mut response);
                hu_to_cm(value);
            }
            Action::CmToHu => {
                let value = get_single_value(&mut response);
                cm_to_hu(value);
            }
            Action::CelToFah => {
                let value = get_single_value(&mut response);
                c_to_f(value);
            }
            Action::FahToCel => {
                let value = get_single_value(&mut response);
                f_to_c(value); 
            }
            Action::CelToKel => {
                let value = get_single_value(&mut response);
                c_to_k(value);
            }
            Action::KelToCel => {
                let value = get_single_value(&mut response);
                k_to_c(value);
            }
            Action::OverworldToNether => {
                let coords = get_coordinates(&mut response);
                overworld_to_nether(&coords);
            }
            Action::NetherToOverworld => {
                let coords = get_coordinates(&mut response);
                nether_to_overworld(&coords);
            }
            Action::MilesToKms => {
                let value = get_single_value(&mut response);
                miles_to_kms(value);
            }
            Action::FeetToMeters => {
                let value = get_single_value(&mut response);
                feet_to_meters(value);
            }
            Action::LightToKms => {
                let value = get_single_value(&mut response);
                light_to_kms(value);
            }
            Action::KmsToLight => {
                let value = get_single_value(&mut response);
                kms_to_light(value);
            }
            Action::PonyMagicToWatt => {
                do_earth_scales_to_uups_process();
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

fn do_earth_scales_to_uups_process() {
    // object levitation
    let mut mass_of_object = String::new();
    let mut distance       = String::new();
    let mut direction      = String::new();
    #[allow(unused_variables)]
    let direction_enum = Direction::Unspecified;
    
    // teleportation
    let mut mass_of_teleporting_pony         = String::new();
    let mut displacement_distance            = String::new();
    let mut barrier_type_passed_through      = String::new();
    #[allow(unused_variables)]
    let barrier_type_passed_through_enum = ObstacleType::None; 

    // entropic restructuring (it also uses mass_of_object var)
    let mut will_power_of_casting_pony = String::new();
    let mut restructuring_type = String::new();
    
    // mutual for all cases
    let mut time_of_casting             = String::new();

    loop {
        mass_of_object.clear();
        distance.clear();
        direction.clear();
        mass_of_teleporting_pony.clear();
        displacement_distance.clear();
        barrier_type_passed_through.clear();
        time_of_casting.clear();

        // direction_enum = Direction::Unspecified;
        // barrier_type_passed_through_enum = ObstacleType::None;

        let mut specification = String::new();        
        println!("╔══════════════════════════════════╗");
        println!("║       Choose specification       ║");
        println!("╠══════════════════════════════════╣");
        println!("║ 1 >> Kinetic Thaumodynamics      ║");
        println!("║ 2 >> Spatial Displacement        ║");
        println!("║ 3 >> Entropic Restructuring      ║");
        println!("╚══════════════════════════════════╝");
        println!("--Help: Kinetic Thaumodynamics cover lifting, throwing, crushing, or holding objects.");
        println!("--Help: Spatial Displacement covers instantaneous teleportation");
        println!("--Help: Entropic Restructuring covers object creation, cloning");
        println!();
        print!("  Specification > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut specification).expect("Couldn't read line");
        let specification = match specification.trim().parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Failed to parse! Did you enter something besides number?");
                continue;
            }
        };
        match specification {
            1 => {
                println!("┌────────────────────────────────────────┐");
                print!("│  Mass of object (kg) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut mass_of_object).expect("Couldn't read line");
                let mass_of_object = match mass_of_object.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                print!("│  Distance of move (m) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut distance).expect("Couldn't read line");
                let distance = match distance.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                print!("│  Time of casting (s) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut time_of_casting).expect("Couldn't read line");
                let time_of_casting = match time_of_casting.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                println!("│╔══════════════════╗");
                println!("│║Direction:        ║");
                println!("│║1 -> Horizontal   ║");
                println!("│║2 -> Vertical     ║");
                println!("│╚══════════════════╝");
                print!("│Direction(number) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut direction).expect("Couldn't read line");
                let direction = match direction.trim().parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                let direction = match direction {
                    1 => Direction::Horizontal,
                    2 => Direction::Vertical,
                    _ => {
                        println!("Invalid direction value.");
                        continue;
                    }
                };
                println!("│╭─────────────────────────────────────╮");
                println!("││ EQUESTRIAN GRAVITY >> 9.807 m/s^2   │");
                println!("││ STD PONY MASS REF  >> 120 kg        │");
                println!("│╰─────────────────────────────────────╯");
                println!("└────────────────────────────────────────┘");
                println!();
                let mut blank = String::new();
                print!("Enter anything to proceed ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut blank).expect("Couldn't read line");
                blank.clear();
                match direction {
                    Direction::Horizontal => {
                        // v = d / t
                        let velocity: f64 = distance / time_of_casting;
                        // KE = 0.5 * m * v^2
                        let kinetic_energy: f64 = 0.5 * mass_of_object * (velocity * velocity);
                        // W = energy / t
                        let result: f64 = (kinetic_energy / time_of_casting) * RESISTANCE_FACTOR;
                        let res = ConvResult {
                            value: result.to_string(),
                            type_m: String::from("Watt"),
                        };
                        print_result(&res);
                        break;
                    }
                    Direction::Vertical => {
                        let velocity: f64 = distance / time_of_casting;
                        // PE = m * g * h
                        let potential_energy: f64 = mass_of_object * EQUESTRIAN_GRAVITY * distance;
                        // KE = 0.5 * m * v^2
                        let kinetic_energy: f64 = 0.5 * mass_of_object * (velocity * velocity);
                        let result: f64 = ((potential_energy + kinetic_energy) / time_of_casting) * RESISTANCE_FACTOR;
                        let res = ConvResult {
                            value: result.to_string(),
                            type_m: String::from("Watt"),
                        };
                        print_result(&res);
                        break;
                    }
                    Direction::Unspecified => {
                        println!("Direction is unspecified.");
                        continue;
                    }
                }
            }
            2 => {
                println!("┌────────────────────────────────────────┐");
                print!("│  Mass of the Pony (kg) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut mass_of_teleporting_pony).expect("Couldn't read line");
                let mass_of_teleporting_pony = match mass_of_teleporting_pony.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                print!("│  Displacement distance (m) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut displacement_distance).expect("Couldn't read line");
                let displacement_distance = match displacement_distance.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                print!("│  Time of casting (s) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut time_of_casting).expect("Couldn't read line");
                let time_of_casting = match time_of_casting.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                println!("│╔══════════════════╗");
                println!("│║Obstacle Type:    ║");
                println!("│║1 -> None         ║");
                println!("│║2 -> With holes   ║");
                println!("│║3 -> Solid        ║");
                println!("│╚══════════════════╝");
                print!("│Type(number) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut barrier_type_passed_through).expect("Couldn't read line");
                let barrier_type_passed_through = match barrier_type_passed_through.trim().parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                let barrier_type_passed_through_enum = match barrier_type_passed_through {
                    1 => ObstacleType::None,
                    2 => ObstacleType::WithHoles,
                    3 => ObstacleType::Solid,
                    _ => {
                        println!("Invalid obtacle type value.");
                        continue;
                    }
                };
                let barrier_modifier: f64 = match barrier_type_passed_through_enum {
                    ObstacleType::None => 1.0,
                    ObstacleType::WithHoles => 5.0,
                    ObstacleType::Solid => 25.0,
                };
                // Final calculation
                // Energy = m * d * barrier_modifier * DISPLACEMENT_SCALE_FACTOR
                // W = Energy / t
                let energy = mass_of_teleporting_pony * displacement_distance * barrier_modifier * DISPLACEMENT_SCALE_FACTOR;
                let result = energy / time_of_casting;
                let res = ConvResult {
                    value: result.to_string(),
                    type_m: String::from("Watt"),
                };
                print_result(&res);
                break;
            }
            3 => {
                println!("┌─────────────────────────────────────────────────────────────────────────┐");
                print!("│  Mass of the target object (kg) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut mass_of_object).expect("Couldn't read line");
                let mass_of_object = match mass_of_object.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                print!("│  Time of casting (s) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut time_of_casting).expect("Couldn't read line");
                let time_of_casting = match time_of_casting.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                println!("│╔═════════════════════╗");
                println!("│║Restructuring Type:  ║");
                println!("│║1 -> Creation        ║");
                println!("│║2 -> Cloning         ║");
                println!("│╚═════════════════════╝");
                print!("│Type(number) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut restructuring_type).expect("Couldn't read line");
                let restructuring_type = match restructuring_type.trim().parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                let restructuring_type = match restructuring_type {
                    1 => RestructuringType::Creation,
                    2 => RestructuringType::Cloning,
                    _ => {
                        println!("Invalid obtacle type value.");
                        continue;
                    }
                };
                println!("│");
                println!("│--Note on Will Power: Incorrect insertion(anything beside 1..10) with result");
                println!("│-- in incorrect result. Measure pony's will power carefully.");
                println!("│--Help on Will Power: 1 -> Complete surrender; 5 -> Moderate effort");
                println!("│-- 10 -> Unshakeable Resolve");
                print!("│Will Power of casting pony (1-10) > ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut will_power_of_casting_pony).expect("Couldn't read line");
                let will_power_of_casting_pony = match will_power_of_casting_pony.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse! Did you enter something besides number?");
                        continue;
                    }
                };
                println!("└─────────────────────────────────────────────────────────────────────────┘");
                let complexity_factor: f64 = match restructuring_type {
                    RestructuringType::Cloning => 10000.0,
                    RestructuringType::Creation => 1000000.0,
                };
                // final calculation
                // Work = m * 3 * t * complexity_factor * will_power
                // W = Work / t
                let work = mass_of_object * 3.0 * complexity_factor * will_power_of_casting_pony;
                let result = work / time_of_casting;
                let res = ConvResult {
                    value: result.to_string(),
                    type_m: String::from("Watt"),
                };
                print_result(&res);
                break;
            }
            _ => {
                println!("Invalid specification value.");
                continue;
            }
        };
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

fn hu_to_cm(hu: f64) {
    // 1 HU == 2.54 CM
    let result = hu * 2.54_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("CM"),
    };
    print_result(&res);
}

fn cm_to_hu(cm: f64) {
    // 1 HU == 2.54 CM
    let result = cm / 2.54_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("HU"),
    };
    print_result(&res);
}
fn c_to_f(c: f64) {
    // F = C * 1.8 + 32
    let result = c * 1.8_f64 + 32.0_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°F"),
    };
    print_result(&res);
}

fn f_to_c(f: f64) {
    // C = (F - 32) / 1.8
    let result = (f - 32.0_f64) / 1.8_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°C"),
    };
    print_result(&res);
}

fn c_to_k(c: f64) {
    // K = C + 273.15
    let result = c + 273.15_f64;
    let res = ConvResult {
        value   : result.to_string(),
        type_m  : String::from("°K"),
    };
    print_result(&res);
}

fn k_to_c(k: f64) {
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

fn feet_to_meters(feet: f64) {
    // divide by 3.28084
    let result = feet / 3.28084_f64;
    let res = ConvResult {
        value : result.to_string(),
        type_m : String::from("Meters"),
    };
    print_result(&res);
}

fn miles_to_kms(miles: f64) {
    // multiply by 1.60934
    let result = miles * 1.60934_f64;
    let res = ConvResult {
        value : result.to_string(),
        type_m : String::from("KM"),
    };
    print_result(&res);
}

fn light_to_kms(light_years: f64) {
    // multiply by 9_460_730_472_580.8
    let result = light_years * 9_460_730_472_580.8_f64;
    let res = ConvResult {
        value : result.to_string(),
        type_m : String::from("KM")
    };
    print_result(&res);
}

fn kms_to_light(kms: f64) {
    // divide by 9_460_730_472_580.8
    let result = kms / 9_460_730_472_580.8_f64;
    let res = ConvResult {
        value : result.to_string(),
        type_m : String::from("Light Years"),
    };
    print_result(&res);
}

