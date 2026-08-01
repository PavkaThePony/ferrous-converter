use crate::{Direction, ConvResult, ObstacleType, RestructuringType, EQUESTRIAN_GRAVITY, DISPLACEMENT_SCALE_FACTOR, RESISTANCE_FACTOR};
use crate::printer;
use std::io::{self, Write};

pub fn do_earth_scales_to_uups_process() {
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
                print!("│  Direction(number) > ");
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
                        printer::print_result(&res);
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
                        printer::print_result(&res);
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
                println!("└────────────────────────────────────────┘");
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
                printer::print_result(&res);
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
                printer::print_result(&res);
                break;
            }
            _ => {
                println!("Invalid specification value.");
                continue;
            }
        };
    }
}

