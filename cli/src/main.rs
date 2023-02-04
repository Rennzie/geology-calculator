use clap::Parser;
use core::{BHOrientation, BHOrientationLine, Borehole, RawMeasurement};
use std::fs::File;

// use crate::borehole::{measurement::RawMeasurement, Borehole};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to csv file containing borehole orientation data
    /// Expected format:
    /// depth,bearing,inclination
    #[arg(long)]
    dh_orientation: String,

    /// Path to csv file containing borehole measurements
    /// Expected format:
    /// depth,alpha,beta
    #[arg(long)]
    dh_measurements: String,

    /// Path to where the output CSV file should be written
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut ori_rdr = csv::Reader::from_path(args.dh_orientation).unwrap();
    let hole_orientations = ori_rdr
        .deserialize()
        .into_iter()
        .map(|result| {
            let record: BHOrientation = result.unwrap();
            record
        })
        .collect();

    let mut ori_rdr = csv::Reader::from_path(args.dh_measurements).unwrap();
    let raw_measurements = ori_rdr
        .deserialize()
        .into_iter()
        .map(|result| {
            let record: RawMeasurement = result.unwrap();
            record
        })
        .collect();

    let dh123 = Borehole::new(BHOrientationLine::Top, raw_measurements, hole_orientations);
    println!("{:#?}", dh123.oriented_measurements);

    let file = File::create(args.output.unwrap()).unwrap();
    let mut writer = csv::Writer::from_writer(file);
    for measurement in dh123.oriented_measurements {
        writer.serialize(measurement).unwrap();
    }
    writer.flush().unwrap();
}
