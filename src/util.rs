use std::time::Duration;


pub fn duration_to_millis(d: &Duration) -> f64 {
    ((d.as_secs() as f64 )* 1000f64) + 
        ((d.subsec_micros() as f64) / 1000f64)
}

