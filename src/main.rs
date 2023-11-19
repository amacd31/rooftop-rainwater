use clap::{value_parser, Arg, ArgAction, Command};

fn calculate_runoff(initial_loss: f64, coefficient: f64, area: f64, rainfall: f64) -> f64 {
    (rainfall - initial_loss).max(0.0) * area * coefficient
}

fn main() {
    let matches = Command::new("rooftop-rainfall")
        .arg_required_else_help(true)
        .arg(
            Arg::new("rainfall")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(f64)),
        )
        // Default to 232.3 as that was the average floor area for new
        // residential dwellings in Australia for 2021-2023 according to the
        // Australian Bureau of Statistics:
        // https://www.abs.gov.au/articles/average-floor-area-new-residential-dwellings
        .arg(
            Arg::new("rooftop-area")
                .short('a')
                .long("area")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(f64))
                .default_value("232.3"),
        )
        // I saw somewhere that 0.95 was a reasonable coefficient to model losses
        // Should probably find some paper or something to back this up...
        .arg(
            Arg::new("coefficient")
                .short('C')
                .long("coefficient")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(f64))
                .default_value("0.95"),
        )
        // 0.3mm of rain is required to fill the first flush system on my house
        // before any water starts flowing to the tanks.
        .arg(
            Arg::new("initial-loss")
                .short('l')
                .long("initial-loss")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(value_parser!(f64))
                .default_value("0.3"),
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
