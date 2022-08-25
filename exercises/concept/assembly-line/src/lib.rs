// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed_f = speed as f64;
    
    match speed {
        0..=4 => speed_f * 221.0 * 1.0,
        5..=8 => speed_f * 221.0 * 0.9,
        9..=10 => speed_f * 221.0 * 0.77,
        _ => -1.0
    }
}
pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    production_rate_per_hour(speed) as u32 / 60
}