use na::{Matrix3, Vector3};
use serde::Deserialize;
use std::{
    cmp::Ordering,
    f64::consts::{FRAC_PI_2, PI},
};

use crate::{
    structure::Plane,
    utils::{dip_direction_from_strike, dip_from_plunge, strike_from_trend},
    validation::error_if_out_of_range,
};

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

#[derive(Debug, Deserialize)]
pub struct RawMeasurement {
    pub depth: f64,
    pub alpha: f64,
    pub beta: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BHOrientation {
    pub depth: f64,
    pub bearing: f64,
    pub inclination: f64,
}

pub struct Borehole {
    /// Oriented structural measurements with alpha and beta angles (in degrees) relative to the borehole `orientation_line`
    pub oriented_measurements: Vec<Plane>,
    /// The location of the orientation line on the borehole
    pub orientation_line: BHOrientationLine,
    /// A vector of hole depths with bearing and inclination.
    /// The fist value MUST have depth=0.0
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

/// Definitions from https://www.sciencedirect.com/science/article/pii/S0098300413000551
/// Internal values are in radians but comments are in degrees
#[derive(Clone, Copy, Debug)]
pub struct Orient {
    /// The angle between North and the borehole trajectory projected to the horizontal.
    /// The angle is measured clockwise from north and has a positive value between 0° and 360°.
    bearing: f64,
    /// Is defined as the acute angle between the horizontal plane and the trajectory of the borehole.
    /// The angle is measured from the horizontal plane and has a value between 0° and 90°.
    /// It is negative if the borehole trajectory is pointing downwards.
    inclination: f64,
    /// The acute dihedral angle between the fracture plane and the trajectory of the borehole.
    /// The angle is restricted to be between 0° and 90°, where 90° corresponds to a fracture perpendicular to the borehole.
    alpha: f64,
    /// The angle from a reference line (in this paper defined as the line of the top of the roof of the borehole profile) to the lower inflexion point of the fracture trace on the borehole wall,
    ///  The angle is measured clockwise looking in the direction of the borehole trajectory and can hence be between 0° and 360°
    beta: f64,
}

impl Orient {
    pub fn new(
        bearing: f64,
        inclination: f64,
        alpha: f64,
        beta: f64,
        orientation_line: BHOrientationLine,
    ) -> Self {
        error_if_out_of_range(&bearing, 0.0, 360.0).unwrap();
        error_if_out_of_range(&inclination, -90.0, 90.0).unwrap();
        error_if_out_of_range(&alpha, 0.0, 90.0).unwrap();
        error_if_out_of_range(&beta, 0.0, 360.0).unwrap();

        let beta = match orientation_line {
            BHOrientationLine::Top => beta,
            BHOrientationLine::Bottom => match beta + 180.0 {
                // ensure beta is between 0 and 360
                x if x > 360.0 => x - 360.0,
                x => x,
            },
        };

        Self {
            bearing: bearing.to_radians(),
            inclination: inclination.to_radians(),
            alpha: alpha.to_radians(),
            beta: beta.to_radians(),
        }
    }

    /// Returns an oriented `Plane` while consuming the `Orient` struct.
    pub fn into_plane(self) -> Plane {
        let (trend, plunge) = self.trend_and_plunge();
        let strike = strike_from_trend(&trend.to_degrees());

        Plane::new(
            strike,
            dip_from_plunge(&plunge.to_degrees()),
            Some(dip_direction_from_strike(&strike)),
            Some(trend.to_degrees()),
            Some(plunge.to_degrees()),
        )
    }

    /// Returns the orientation of the pole to the measured plane (trend, plunge)
    fn trend_and_plunge(&self) -> (f64, f64) {
        let n_g = self.normal_g();
        let apparent_trend = (n_g.x / (n_g.x.powi(2) + n_g.y.powi(2)).sqrt()).acos();

        let mut trend = if n_g.y <= 0.0 {
            FRAC_PI_2 + apparent_trend
        } else {
            FRAC_PI_2 - apparent_trend
        };

        if trend < 0.0 {
            trend += PI * 2.0;
        }

        (trend, -n_g.z.asin())
    }

    /// The normal vector of the measured plane relative to the borehole
    fn normal_bh(&self) -> Vector3<f64> {
        let alpha = self.alpha;
        let beta = self.beta;

        let x = alpha.cos() * beta.cos();
        let y = alpha.cos() * beta.sin();
        let z = alpha.sin();

        Vector3::new(x, y, z)
    }

    /// The normal vector of the measured plane relative to the global coordinate system
    fn normal_g(&self) -> Vector3<f64> {
        let z_rot = self.z_rot();
        let y_rot = self.y_rot();
        let bh_normal = self.normal_bh();

        z_rot * y_rot * bh_normal
    }

    fn y_rot(&self) -> Matrix3<f64> {
        let i = FRAC_PI_2 - self.inclination;
        Matrix3::new(i.cos(), 0.0, i.sin(), 0.0, 1.0, 0.0, -i.sin(), 0.0, i.cos())
    }

    fn z_rot(&self) -> Matrix3<f64> {
        let b = FRAC_PI_2 - self.bearing;
        Matrix3::new(b.cos(), -b.sin(), 0.0, b.sin(), b.cos(), 0.0, 0.0, 0.0, 1.0)
    }
}

// ----- Tests -------
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Hole intersecting a plane perpendicular to the borehole axis
     * will have a trend and plunge equal to the bearing and inclination of the hole.
     * Note: Plunge will be positive while inclination will be negative by convention.
     * positive inclinations are reserved for upward drilling in underground settings.
     * _________________________________________________
     * \ (bearing=0.0, inclination=-45.0)
     *  \
     *   \     //
     *    \  // Shear plane (alpha=90.0, beta=180.0) = (trend=0.0, plunge=45.0)
     *     //
     *   // \
     * //    \
     *        \
     */
    #[test]
    fn orient_new_defaults() {
        let (trend, plunge) =
            Orient::new(0.0, -45.0, 90.0, 180.0, BHOrientationLine::Top).trend_and_plunge();

        assert_eq!(trend.to_degrees().round(), 0.0);
        assert_eq!(plunge.to_degrees().round(), 45.0);
    }

    #[test]
    fn orient_new_ori_bottom() {
        let (trend, plunge) =
            Orient::new(0.0, -45.0, 90.0, 0.0, BHOrientationLine::Bottom).trend_and_plunge();

        assert_eq!(trend.to_degrees().round(), 0.0);
        assert_eq!(plunge.to_degrees().round(), 45.0);
    }

    #[test]
    #[should_panic]
    fn orient_new_invalid_bearing() {
        let bad_bearing = 361.0;
        Orient::new(bad_bearing, -45.0, 90.0, 180.0, BHOrientationLine::Top);
    }

    #[test]
    #[should_panic]
    fn orient_new_invalid_inclination() {
        let bad_inclination = -91.0;
        Orient::new(0.0, bad_inclination, 90.0, 180.0, BHOrientationLine::Top);
    }

    #[test]
    #[should_panic]
    fn orient_new_invalid_alpha() {
        let bad_alpha = 361.0;
        Orient::new(0.0, -45.0, bad_alpha, 180.0, BHOrientationLine::Top);
    }

    #[test]
    #[should_panic]
    fn orient_new_invalid_beta() {
        let bad_beta = 361.0;
        Orient::new(0.0, -45.0, 90.0, bad_beta, BHOrientationLine::Top);
    }

    #[test]
    fn orient_into_plane_returns_plane() {
        let orient = Orient::new(0.0, -45.0, 90.0, 180.0, BHOrientationLine::Top);
        let plane = orient.into_plane();

        // assert_matches!(plane, Plane); - its unstable
        assert_eq!(plane.strike.round(), 90.0);
        assert_eq!(plane.dip.round(), 45.0);
        assert_eq!(plane.dip_direction.round(), 180.0);
        assert_eq!(plane.pole.trend.round(), 0.0);
        assert_eq!(plane.pole.plunge.round(), 45.0);
    }

    #[test]
    fn real_world_orient() {
        // From measurements conducted on Loulo 3 brownfields drill core in 2015. See test_data
        let orient = Orient::new(262.7, -55.3, 65.0, 230.0, BHOrientationLine::Top);

        let (trend, plunge) = orient.trend_and_plunge();
        assert_eq!(plunge.to_degrees().round(), 36.0);
        assert_eq!(trend.to_degrees().round(), 286.0);

        let plane = orient.into_plane();
        assert_eq!(plane.dip.round(), 54.0);
        assert_eq!(plane.strike.round(), 16.0);
        assert_eq!(plane.dip_direction.round(), 106.0);
        assert_eq!(plane.pole.trend.round(), 286.0);
        assert_eq!(plane.pole.plunge.round(), 36.0);
    }
}

// todo: add some tests
