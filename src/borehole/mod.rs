pub mod measurement;
use self::measurement::{Plane, RawMeasurement};
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum BHOrientationLine {
    Top,
    Bottom,
}

impl Default for BHOrientationLine {
    fn default() -> Self {
        Self::Top
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BHOrientation {
    pub depth: f64,
    pub bearing: f64,
    pub inclination: f64,
}

pub struct Borehole {
    /// Oriented structural measurements with alpha and beta angles relative to the borehole `orientation_line`
    pub oriented_measurements: Vec<Plane>,
    /// The location of the orientation line on the borehole
    pub orientation_line: BHOrientationLine,
    /// A vector of whole depths with bearing and inclination.
    /// The fist value MUST have depth 0.0
    pub hole_orientation: Vec<BHOrientation>,
}

impl Borehole {
    pub fn new(
        orientation_line: BHOrientationLine,
        raw_measurements: Vec<RawMeasurement>,
        hole_orientation: Vec<BHOrientation>,
    ) -> Self {
        Self {
            oriented_measurements: map_measurements_to_depths(
                raw_measurements,
                &hole_orientation,
                &orientation_line,
            ),
            orientation_line,
            hole_orientation,
        }
    }
}

fn map_measurements_to_depths(
    raw_measurements: Vec<RawMeasurement>,
    raw_orientation: &[BHOrientation],
    orientation_line: &BHOrientationLine,
) -> Vec<Plane> {
    // error if the first raw_orientation depth is not 0.0
    if raw_orientation[0].depth != 0.0 {
        panic!("The first raw_orientation depth must be 0.0");
    }

    let mut depth_pairs: Vec<(f64, f64)> = vec![];
    let last_index = raw_orientation.len() - 1;
    for (i, measurement) in raw_orientation.iter().enumerate() {
        match i {
            0 => depth_pairs.push((
                measurement.depth,
                (raw_orientation[i + 1].depth - measurement.depth) / 2.0,
            )),
            _ if i == last_index => depth_pairs.push((
                (measurement.depth - (measurement.depth - raw_orientation[i - 1].depth) / 2.0),
                measurement.depth,
            )),
            _ if i < last_index => depth_pairs.push((
                (measurement.depth - (measurement.depth - raw_orientation[i - 1].depth) / 2.0),
                (measurement.depth + (raw_orientation[i + 1].depth - measurement.depth) / 2.0),
            )),
            _ => panic!("This should never happen"),
        }
    }

    raw_measurements
        .into_iter()
        .map(|measurement| {
            let index = depth_pairs.binary_search_by(|(first, last)| {
                if measurement.depth > *first && measurement.depth <= *last {
                    Ordering::Equal
                } else if measurement.depth <= *first {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            let index = index.unwrap();

            let orientation = &raw_orientation[index];
            Plane::alpha_beta(
                orientation.bearing,
                orientation.inclination,
                measurement.alpha,
                measurement.beta,
                *orientation_line,
            )
        })
        .collect::<Vec<Plane>>()
}

// note: there's something interesting about this approach but it doesn't fully satisfy the halfway between measurements test.
// fn map_measurements_to_depths_v2(
//     raw_measurements: Vec<RawMeasurement>,
//     raw_orientation: &[HoleOrientation],
// ) -> Vec<Plane> {
//     // error if the first raw_orientation depth is not 0.0
//     if raw_orientation[0].depth != 0.0 {
//         panic!("The first raw_orientation depth must be 0.0");
//     }

//     let measurements = raw_measurements
//         .into_iter()
//         .map(|measurement| {
//             let index = raw_orientation
//                 .binary_search_by(|m| {
//                     m.depth
//                         .partial_cmp(&measurement.depth)
//                         .unwrap_or(Ordering::Less)
//                 })
//                 .unwrap();

//             let orientation = &raw_orientation[if index == raw_orientation.len() {
//                 index - 1
//             } else {
//                 index
//             }];

//             Plane::new(
//                 orientation.bearing,
//                 orientation.inclination,
//                 measurement.alpha,
//                 measurement.beta,
//             )
//         })
//         .collect::<Vec<Plane>>();

//     println!("measurements: {measurements:?}");

//     measurements
// }
