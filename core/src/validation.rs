pub fn error_if_out_of_range(value: &f64, min: f64, max: f64) -> Result<f64, String> {
    if value < &min || value > &max {
        Err(format!("Value {value} is out of range [{min}, {max}]"))
    } else {
        Ok(*value)
    }
}
