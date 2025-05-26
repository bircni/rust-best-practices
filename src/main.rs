use clap::Parser;
use serde::Deserialize;

/// Simple program to fetch weather information from wttr.in
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// City name
    cities: Vec<String>,
    /// Use imperial units (Fahrenheit)
    #[arg(long)]
    units: bool,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current_condition: Vec<CurrentCondition>,
}

#[derive(Debug, Deserialize)]
struct CurrentCondition {
    temp_C: String,
    temp_F: String,
    weatherDesc: Vec<Description>,
}

#[derive(Debug, Deserialize)]
struct Description {
    value: String,
}

fn main() {
    let cli = Cli::parse();

    for city in &cli.cities {
        let url = format!("https://wttr.in/{}?format=j1", city);
        let res = ureq::get(&url).call();

        if res.is_ok() {
            let mut response = res.unwrap();
            let body = response.body_mut();
            let body: WeatherResponse = body.read_json().expect("Failed to parse JSON");
            let cond = &body.current_condition[0];
            println!(
                "{}: {}°, {}",
                city,
                if cli.units {
                    &cond.temp_F
                } else {
                    &cond.temp_C
                },
                cond.weatherDesc[0].value
            );
        } else {
            eprintln!("Failed to fetch weather for {}", city);
        }
    }
}
