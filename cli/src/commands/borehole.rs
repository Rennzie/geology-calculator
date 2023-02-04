use clap::Args;
use geocalc::{BHOrientation, BHOrientationLine, Borehole as GCBorehole, RawMeasurement};
use std::fs::File;

#[derive(Args)]
pub struct Borehole {
    /// Path to csv file containing borehole orientation data
    /// Expected format:
    /// depth,bearing,inclination
    #[arg(long)]
    pub dh_orientation: String,

    /// Path to csv file containing borehole measurements
    /// Expected format:
    /// depth,alpha,beta
    #[arg(long)]
    pub dh_measurements: String,

    /// Path to where the output CSV file should be written
    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn borehole(cmd: Borehole) {
    let mut ori_rdr = csv::Reader::from_path(cmd.dh_orientation).unwrap();
    let hole_orientations = ori_rdr
        .deserialize()
        .into_iter()
        .map(|result| {
            let record: BHOrientation = result.unwrap();
            record
        })
        .collect();

    let mut ori_rdr = csv::Reader::from_path(cmd.dh_measurements).unwrap();
    let raw_measurements = ori_rdr
        .deserialize()
        .into_iter()
        .map(|result| {
            let record: RawMeasurement = result.unwrap();
            record
        })
        .collect();

    let dh123 = GCBorehole::new(BHOrientationLine::Top, raw_measurements, hole_orientations);
    println!("{:#?}", dh123.oriented_measurements);

    match cmd.output {
        Some(path) => {
            let file = File::create(&path).unwrap();
            let mut writer = csv::Writer::from_writer(file);

            #[rustfmt::skip]
            writer.write_record(["strike", "dip", "dip_direction", "pole.trend", "pole.plunge"]).unwrap();
            for measurement in dh123.oriented_measurements {
                writer
                    .write_record([
                        measurement.strike.to_string(),
                        measurement.dip.to_string(),
                        measurement.dip_direction.to_string(),
                        measurement.pole.trend.to_string(),
                        measurement.pole.plunge.to_string(),
                    ])
                    .unwrap();
            }
            writer.flush().unwrap();
            println!("Output written to: {path}")
        }
        None => {
            println!(
                "No output file specified.\nUse the --output flag to specify an output file.\nExiting."
            );
            std::process::exit(0);
        }
    }
}
