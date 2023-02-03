use core::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};
use na::{Matrix3, Vector3};

use crate::validation;
use validation::error_if_out_of_range;

use super::OrientationLine;

pub struct RawMeasurement {
    pub depth: f64,
    pub alpha: f64,
    pub beta: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub strike: f64,
    pub dip: f64,
    pub dip_direction: f64,
    pub trend: f64,
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

        let dip_direction = dip_direction.unwrap_or(strike + 90.0);
        // NOTE: these are incorrect. I need to calculate the normal vector to the plane defined by strike and dip
        let trend = trend.unwrap_or(strike + 90.0);
        let plunge = plunge.unwrap_or(90.0 - dip);

        error_if_out_of_range(&dip_direction, 0.0, 360.0).unwrap();
        error_if_out_of_range(&trend, 0.0, 360.0).unwrap();
        error_if_out_of_range(&plunge, 0.0, 90.0).unwrap();

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
        orientation_line: OrientationLine,
    ) -> Self {
        let orient = Orient::new(bearing, inclination, alpha, beta, orientation_line);
        orient.into_plane()
    }
}

/// Definitions from https://www.sciencedirect.com/science/article/pii/S0098300413000551
/// Internal values are in radians but comments are in degrees
#[derive(Clone, Copy, Debug)]
struct Orient {
    /// The angle between North and the borehole trajectory projected to the horizontal.
    /// The angle is measured clockwise from north and has a value between 0° and 360°.
    bearing: f64,
    /// Is defined as the acute angle between the horizontal plane and the trajectory of the borehole.
    /// The angle is measured from the horizontal plane and has a value between 0° and 90°.
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
        orientation_line: OrientationLine,
    ) -> Self {
        error_if_out_of_range(&bearing, 0.0, 360.0).unwrap();
        error_if_out_of_range(&inclination, -90.0, 90.0).unwrap();
        error_if_out_of_range(&alpha, 0.0, 90.0).unwrap();
        error_if_out_of_range(&beta, 0.0, 360.0).unwrap();

        let bearing = match orientation_line {
            OrientationLine::Top => bearing,
            OrientationLine::Bottom => bearing - 180.0,
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
        let strike = if (trend - FRAC_PI_2) <= 0.0 {
            trend + FRAC_PI_2
        } else {
            trend - FRAC_PI_3
        };

        let dip_direction = if (strike - PI) <= 0.0 {
            strike + PI
        } else {
            strike - PI
        };

        Plane::new(
            strike.to_degrees(),
            plunge.to_degrees(),
            Some(dip_direction.to_degrees()),
            Some(trend.to_degrees()),
            Some(plunge.to_degrees()),
        )
    }

    /// Returns the orientation of the pole to the measured plane (trend, plunge)
    fn trend_and_plunge(&self) -> (f64, f64) {
        let n_g = self.normal_g();
        let apparent_trend = (n_g.x / (n_g.x.powi(2) + n_g.y.powi(2)).sqrt()).acos();

        let trend = if n_g.y <= 0.0 {
            FRAC_PI_2 + apparent_trend
        } else {
            FRAC_PI_2 - apparent_trend
        };

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

// /**
// * _________________________________________________
// * \ (bearing=0.0, inclination=45.0)
// *  \
// *   \     //
// *    \  // Shear plane (alpha=90.0, beta=180.0) = (trend=0.0, plunge=45.0)
// *     //
// *   // \
// * //    \
// *        \
// */
// todo: add tests
