use std::{env, path::PathBuf};

use clap::{value_parser, Arg, ArgAction, Command};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Parameters {
    roof_area: f64,
    coefficient: f64,
    initial_loss: f64,
}

#[derive(Deserialize, Debug)]
struct Config {
    parameters: Parameters,
}

fn read_config() -> Parameters {
    let config_file_name = "rooftop-rainfall-parameters.toml";

    let exe = env::current_exe().unwrap();
    let exe_path = exe.parent().unwrap();

    let mut possible_configs = vec![PathBuf::from(exe_path).join(config_file_name)];

    match home::home_dir() {
        Some(home_dir) => {
            let home_dir_path = home_dir.clone();
            let home_config = home_dir_path.join(".config").join(config_file_name);
            possible_configs.insert(0, home_config)
        }
        None => (),
    }

    let mut config = Config {
        parameters: Parameters {
            // Default to 232.3 as that was the average floor area for new
            // residential dwellings in Australia for 2021-2023 according to the
            // Australian Bureau of Statistics:
            // https://www.abs.gov.au/articles/average-floor-area-new-residential-dwellings
            roof_area: 232.3,
            // I saw somewhere that 0.95 was a reasonable coefficient to model losses
            // Should probably find some paper or something to back this up...
            coefficient: 0.95,
            // 0.3mm of rain is required to fill the first flush system on my house
            // before any water starts flowing to the tanks.
            initial_loss: 0.3,
        },
    };

    let mut successful_config = None;
    for path in possible_configs {
        match std::fs::read_to_string(path.clone()) {
            Ok(c) => {
                let last_config: Config = match toml::from_str(&c) {
                    Ok(parsed) => parsed,
                    Err(_) => break,
                };
                successful_config = Some(path.clone());
                config.parameters = last_config.parameters
            }
            Err(_) => (),
        };
    }

    match successful_config {
        Some(config_file) => match config_file.to_str() {
            Some(file) => println!("Using defaults from configuration file: {}\n", file),
            None => println!("Using defaults from configuration file with unprintable name\n"),
        },
        None => (),
    }

    config.parameters
}

fn calculate_runoff(initial_loss: f64, coefficient: f64, area: f64, rainfall: f64) -> f64 {
    (rainfall - initial_loss).max(0.0) * area * coefficient
}

fn main() {
    let config = read_config();

    let matches = Command::new("rooftop-rainfall")
        .arg_required_else_help(true)
        .arg(
            Arg::new("rainfall")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(f64)),
        )
        .arg(
            Arg::new("rooftop-area")
                .short('a')
                .long("area")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(f64))
                .default_value(&config.roof_area.to_string()),
        )
        .arg(
            Arg::new("coefficient")
                .short('C')
                .long("coefficient")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(f64))
                .default_value(&config.coefficient.to_string()),
        )
        .arg(
            Arg::new("initial-loss")
                .short('l')
                .long("initial-loss")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(f64))
                .default_value(&config.initial_loss.to_string()),
        )
        .get_matches();

    let rainfall = matches.get_one::<f64>("rainfall").unwrap();
    let rooftop_coefficient = matches.get_one::<f64>("coefficient").unwrap();
    let rooftop_area = matches.get_one::<f64>("rooftop-area").unwrap();
    let initial_loss = matches.get_one::<f64>("initial-loss").unwrap();
    println!(
        "{}",
        calculate_runoff(
            *initial_loss,
            *rooftop_coefficient,
            *rooftop_area,
            *rainfall
        )
    );
}
