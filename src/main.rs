mod borehole;
mod validation;

extern crate nalgebra as na;
use borehole::measurement::Plane as BoreholeMeasurement;
use clap::Parser;

use crate::borehole::{measurement::RawMeasurement, Borehole};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Alpha core angle. Must be in range [0, 90]
    #[arg(long)]
    alpha: f64,

    /// Beta core angle. Must be in range [0, 360]
    #[arg(long)]
    beta: f64,

    /// Inclination. Must be in range [0, 90]
    #[arg(long)]
    inclination: f64,

    /// Bearing. Must be in range [0, 360]
    #[arg(long)]
    bearing: f64,
}

fn main() {
    let mut args = Args::parse();
    args.inclination *= -1.0;

    let dh123 = Borehole::new(
        borehole::OrientationLine::Top,
        vec![
            RawMeasurement {
                depth: 1.0,
                alpha: 90.0,
                beta: 0.0,
            },
            RawMeasurement {
                depth: 10.0,
                alpha: 90.0,
                beta: 0.0,
            },
            RawMeasurement {
                depth: 20.0,
                alpha: 90.0,
                beta: 0.0,
            },
            RawMeasurement {
                depth: 30.0,
                alpha: 90.0,
                beta: 0.0,
            },
        ],
        vec![
            borehole::HoleOrientation {
                depth: 0.0,
                bearing: 0.0,
                inclination: -45.0,
            },
            borehole::HoleOrientation {
                depth: 12.5,
                bearing: 0.0,
                inclination: -45.0,
            },
            borehole::HoleOrientation {
                depth: 16.0,
                bearing: 0.0,
                inclination: -45.0,
            },
            borehole::HoleOrientation {
                depth: 22.0,
                bearing: 0.0,
                inclination: -45.0,
            },
            borehole::HoleOrientation {
                depth: 30.0,
                bearing: 0.0,
                inclination: -45.0,
            },
        ],
    );

    println!("{:#?}", dh123.oriented_measurements);
    println!("Alpha {:?}", args.alpha);
    println!("Beta {:?}", args.beta);
    println!("Inclination {:?}", args.inclination);
    println!("Bearing {:?}", args.bearing);
    println!("------------------\n");

    let oriented_measurement = BoreholeMeasurement::alpha_beta(
        args.bearing,
        args.inclination,
        args.alpha,
        args.beta,
        borehole::OrientationLine::Top,
    );

    println!(
        "Trend: {}, Plunge: {}",
        oriented_measurement.trend.round(),
        oriented_measurement.plunge.round()
    );
    println!(
        "Strike: {}, Dip: {}",
        oriented_measurement.strike.round(),
        oriented_measurement.dip.round()
    );
    assert!(
        oriented_measurement.trend.round() == 0.0 && oriented_measurement.plunge.round() == 45.0,
        "Expected trend 0.0 and plunge 45.0"
    );
}
