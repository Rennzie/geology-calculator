use crate::validation::error_if_out_of_range;

/// Get the dip direction from the strike using decimal degrees.
pub fn dip_direction_from_strike(strike: &f64) -> f64 {
    error_if_out_of_range(strike, 0.0, 360.0).unwrap();
    clockwise_from_input(strike, 90.0, 0.0, 360.0)
}

/// Get the strike from the trend using decimal degrees.
pub fn strike_from_trend(trend: &f64) -> f64 {
    error_if_out_of_range(trend, 0.0, 360.0).unwrap();
    clockwise_from_input(trend, 90.0, 0.0, 360.0)
}

/// Get the trend from the strike using decimal degrees.
pub fn trend_from_strike(strike: &f64) -> f64 {
    error_if_out_of_range(strike, 0.0, 360.0).unwrap();
    clockwise_from_input(strike, 270.0, 0.0, 360.0)
}

pub fn clockwise_from_input(input: &f64, add: f64, min: f64, max: f64) -> f64 {
    error_if_out_of_range(input, min, max).unwrap();

    let output = input + add;
    if output > max {
        output - max
    } else {
        output
    }
}

/// Get the plunge from the dip using decimal degrees.
pub fn dip_from_plunge(plunge: &f64) -> f64 {
    get_perpendicular_angle(plunge)
}

/// Get the plunge from the dip using decimal degrees.
pub fn plunge_from_dip(dip: &f64) -> f64 {
    get_perpendicular_angle(dip)
}

/// Get the perpendicular angle to the input angle using decimal degrees.
/// Ensures the angle is within the range of 0.0 to 90.0.
pub fn get_perpendicular_angle(angle: &f64) -> f64 {
    error_if_out_of_range(angle, 0.0, 90.0).unwrap();
    90.0 - angle
}
