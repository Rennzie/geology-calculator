use crate::{
    borehole::{BHOrientationLine, Orient},
    utils::{dip_direction_from_strike, plunge_from_dip, trend_from_strike},
    validation::error_if_out_of_range,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Lineation {
    /// The angle (in degrees) between North and the downward pointing pole (normal vector) projected to the horizontal.
    /// It can also be thought of as the azimuth of the pole to a planar structure.
    /// The angle is measured clockwise from north and can be between 0° and 360°. (Trend equals strike −90°, and dip direction −180°.)
    pub trend: f64,
    /// The angle (in degrees) between the horizontal plane and the planar pole, i.e. the downward pointing normal vector of the plane.
    /// The value of the angle is positive and can be between 0° and 90°. (plunge equals 90°—dip)
    pub plunge: f64,
}

impl Lineation {
    pub fn new(trend: f64, plunge: f64) -> Self {
        error_if_out_of_range(&trend, 0.0, 360.0).unwrap();
        error_if_out_of_range(&plunge, 0.0, 90.0).unwrap();
        Self { trend, plunge }
    }
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
    #[serde(flatten)]
    pub pole: Lineation,
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

        let plunge = plunge.unwrap_or(plunge_from_dip(&dip));
        error_if_out_of_range(&plunge, 0.0, 90.0).unwrap();

        let trend = trend.unwrap_or(trend_from_strike(&strike));
        error_if_out_of_range(&trend, 0.0, 360.0).unwrap();

        Self {
            strike,
            dip,
            dip_direction,
            pole: Lineation::new(trend, plunge),
        }
    }

    /// Create a new `Plane` from oriented borehole measurements.
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
