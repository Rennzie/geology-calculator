use super::BHOrientationLine;
use crate::validation;
use core::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};
use na::{Matrix3, Vector3};
use nalgebra::Unit;
use serde::{Deserialize, Serialize};
use validation::error_if_out_of_range;

#[derive(Debug, Deserialize)]
pub struct RawMeasurement {
    pub depth: f64,
    pub alpha: f64,
    pub beta: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Plane {
    /// The strike of a planar structure in degrees
    /// Strike is the angle between the north and the line of intersection of the plane with the horizontal plane
    /// Strike is measured clockwise from north and has a value between 0° and 360°.
    pub strike: f64,
    /// The dip of a planar structure in degrees
    /// The dip is the angle between the horizontal plane and the plane of the structure.
    /// The dip is measured from the horizontal plane and has a value between 0° and 90°.
    pub dip: f64,
    /// The dip direction of a planar structure in degrees
    /// The dip direction is the angle between the north the direction of the dip. It is perpendicular to the strike in the clockwise direction..
    /// The dip direction is measured clockwise from north and a positive value between 0° and 360°.
    pub dip_direction: f64,
    /// The angle between North and the downward pointing pole (normal vector) projected to the horizontal.
    /// It can also be thought of as the azimuth of the pole to planar structure.
    /// The angle is measured clockwise from north and can be between 0° and 360°. (Trend equals strike −90°, and dip direction −180°.)
    pub trend: f64,
    /// The angle between the horizontal plane and the planar pole, i.e. the downward pointing normal vector of the plane.
    /// The value of the angle is positive and can be between 0° and 90°. (plunge equals 90°—dip)
    pub plunge: f64,
}

impl Plane {
    pub fn new(
        strike: f64,
        dip: f64,
        dip_direction: Option<f64>,
        trend: Option<f64>,
        plunge: Option<f64>,
    ) -> Self {
        error_if_out_of_range(&strike, 0.0, 360.0).unwrap();
        error_if_out_of_range(&dip, 0.0, 90.0).unwrap();

        let dip_direction = dip_direction.unwrap_or(dip_direction_from_strike(&strike));
        error_if_out_of_range(&dip_direction, 0.0, 360.0).unwrap();

        let plunge = plunge.unwrap_or(dip_from_plunge(&dip));
        error_if_out_of_range(&plunge, 0.0, 90.0).unwrap();

        // NOTE: this are incorrect. I need to calculate the normal vector to the plane defined by strike and dip
        let trend = trend.unwrap_or(strike + 90.0);
        // error_if_out_of_range(&trend, 0.0, 360.0).unwrap();

        Self {
            strike,
            dip,
            dip_direction,
            trend,
            plunge,
        }
    }

    /// Create a new `Plane` from oriented borehole measurements.
    #[must_use]
    pub fn alpha_beta(
        bearing: f64,
        inclination: f64,
        alpha: f64,
        beta: f64,
        orientation_line: BHOrientationLine,
    ) -> Self {
        let orient = Orient::new(bearing, inclination, alpha, beta, orientation_line);
        orient.into_plane()
    }
}

fn pole_to_plane(strike: f64, dip: f64, plunge: f64) -> Vector3<f64> {
    let strike = strike.to_radians();
    let dip = dip.to_radians();
    let plunge = plunge.to_radians();

    let rotation_matrix = Matrix3::new(
        strike.cos(),
        -strike.sin(),
        0.0,
        strike.sin(),
        strike.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    ) * Matrix3::new(
        1.0,
        0.0,
        0.0,
        0.0,
        dip.cos(),
        -dip.sin(),
        0.0,
        dip.sin(),
        dip.cos(),
    ) * Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, plunge.cos());

    let pole = Unit::new_normalize(rotation_matrix * Vector3::x());
    pole.into_inner()
}

/// Definitions from https://www.sciencedirect.com/science/article/pii/S0098300413000551
/// Internal values are in radians but comments are in degrees
#[derive(Clone, Copy, Debug)]
struct Orient {
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
    fn new(
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
    fn into_plane(self) -> Plane {
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

/// Get the dip direction from the strike using decimal degrees.
fn dip_direction_from_strike(strike: &f64) -> f64 {
    error_if_out_of_range(strike, 0.0, 360.0).unwrap();
    let dip_direction = strike + 90.0;

    if dip_direction > 360.0 {
        dip_direction - 360.0
    } else {
        dip_direction
    }
}

/// Get the strike from the trend using decimal degrees.
fn strike_from_trend(trend: &f64) -> f64 {
    error_if_out_of_range(trend, 0.0, 360.0).unwrap();
    let strike = trend + 90.0;

    if strike > 360.0 {
        strike - 360.0
    } else {
        strike
    }
}

/// Get the plunge from the dip using decimal degrees.
fn dip_from_plunge(plunge: &f64) -> f64 {
    error_if_out_of_range(plunge, 0.0, 90.0).unwrap();
    90.0 - plunge
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
        assert_eq!(plane.trend.round(), 0.0);
        assert_eq!(plane.plunge.round(), 45.0);
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
        assert_eq!(plane.trend.round(), 286.0);
        assert_eq!(plane.plunge.round(), 36.0);
    }
}
