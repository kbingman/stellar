use crate::models::Star;
use rand::prelude::*;
// use std::fmt;

/**
 * Returns the name for each Spectal Class
 */
pub fn get_spectral_class(spectral_class: &str) -> &str {
    match spectral_class {
        "O" => "Blue",
        "B" => "Blue-White",
        "A" => "White",
        "F" => "Yellow-White",
        "G" => "Yellow",
        "K" => "Orange",
        "M" => "Red",
        _ => "None",
    }
}

/**
 * Returns the name for each Spectal Class
 */
pub fn get_luminosity_class(luminosity_class: &str) -> &str {
    match luminosity_class {
        "III" => "Giant",
        "Ib" => "Supergiant",
        "Iab" => "Supergiant",
        "Ia" => "Supergiant",
        "Ia+" => "Hypergiant",
        _ => "None",
    }
}

/**
 * Generate weighted B-V
 *
 * Creates a properly distributed set of B-V Colour indices
 */
pub fn get_weighted_bv(rng: &mut StdRng) -> f64 {
    let bvfac = rng.gen_range(0.0, 100.0);
    match bvfac {
        b if b <= 0.01 => rng.gen_range(-0.33, -0.3),
        b if (b >= 0.01) && (b <= 0.13) => rng.gen_range(-0.3, -0.02),
        b if (b >= 0.13) && (b <= 0.73) => rng.gen_range(-0.02, 0.3),
        b if (b >= 0.73) && (b <= 3.73) => rng.gen_range(0.3, 0.58),
        b if (b >= 3.73) && (b <= 11.33) => rng.gen_range(0.58, 0.81),
        b if (b >= 11.73) && (b <= 22.43) => rng.gen_range(0.81, 1.40),
        b if (b >= 22.43) && (b <= 100.0) => rng.gen_range(1.40, 3.17),
        _ => 0.0,
    }
}

/**
 * Generate temperature from B-V
 */
pub fn bv_to_temp(bv: f64) -> u32 {
    (4600.0 * ((1.0 / ((0.92 * bv) + 1.7)) + (1.0 / ((0.92 * bv) + 0.62)))).round() as u32
}

/**
 * Generate mass from luminosity
 */
pub fn luminosity_to_mass(lum: f64) -> f64 {
    // branch 1: mass < 0.43
    let la1: f64 = 0.3302;
    let lb1: f64 = 1.8946;
    let lc1: f64 = 0.4347;
    // branch 2: 0.43 < mass < 2
    let la2: f64 = 16.0;
    let lb2: f64 = 1.0;
    let lc2: f64 = 0.25;
    // branch 3: 2 < mass < 20
    let la3: f64 = 50087.9;
    let lb3: f64 = 0.9038;
    let lc3: f64 = 0.25;
    // branch 4: 20 < mass < 55
    let la4: f64 = 1233870.0;
    let lb4: f64 = 1.0;
    let lc4: f64 = 0.2857;
    // branch 5: mass > 55
    let la5: f64 = 32000.0;

    match lum {
        l if l < la1 => lb1 * lum.powf(lc1),
        l if (l > la1) && (l < la2) => lb2 * lum.powf(lc2),
        l if (l > la2) && (l < la3) => lb3 * lum.powf(lc3),
        l if (l > la3) && (l < la4) => lb4 * lum.powf(lc4),
        l if l > la4 => lum * la5,
        _ => 0.0,
    }
}

/**
 * Main Sequence specific functions
 * Generate luminosity from B-V
 * pow(10, (ba1 + (bb1 * log(bc1 / (bv + bd1)))))
 */
pub fn ms_bv_to_radius(bv: f64) -> f64 {
    let ba1 = -5.793;
    let bb1 = 0.6729;
    let bc1 = 7360.9;
    let bd1 = 0.6411;

    10_f64.powf(ba1 + (bb1 * (bc1 / (bv + bd1)).ln()))
}

/**
 * Main Sequence specific functions
 * Generate luminosity from B-V
 * pow(10, (ba2 + (bb2 * log(bc1 / (bv + bd1)))
 */
pub fn ms_bv_to_luminosity(bv: f64) -> f64 {
    let ba2 = -26.63;
    let bb2 = 3.083;
    let bc1 = 7360.9;
    let bd1 = 0.6411;

    10_f64.powf(ba2 + (bb2 * (bc1 / (bv + bd1)).ln()))
}

/**
 * Super Giant specific functions
 * generate radius from B-V
 * pow(10, (ba2 + (bb2 * log(bc1 / (bv + bd1)))
 */
pub fn giant_bv_to_radius(bv: f64) -> f64 {
    let base: f64 = 10.0;
    let ba3 = 22.25;
    let bb3 = -2.502;
    let bc2 = 8627.59;
    let bd2 = 0.7920;

    base.powf(ba3 + (bb3 * (bc2 / (bv + bd2).ln())))
}

/**
 * Super Giant specific functions
 * generate luminosity from B-V
 * pow(10, (ba2 + (bb2 * log(bc1 / (bv + bd1)))
 */
pub fn giant_bv_to_luminosity(bv: f64) -> f64 {
    let base: f64 = 10.0;
    let ba4 = 29.469;
    let bb4 = -3.2676;
    let bc2 = 8627.59;
    let bd2 = 0.7920;

    base.powf(ba4 + (bb4 * (bc2 / (bv + bd2)).ln()))
}

pub fn create_star_from_bv(bv: f64) -> Star {
    let luminosity = ms_bv_to_luminosity(bv);
    let temp = bv_to_temp(bv);
    Star {
        temp: temp,
        luminosity: luminosity,
        radius: ms_bv_to_radius(bv),
        mass: luminosity_to_mass(luminosity),
        spectral_class: get_spectral_class_from_temp(temp),
    }
}

pub fn get_spectral_class_from_temp(temp: u32) -> u32 {
    let mut subdiv = 0;
    // let mut result = "";
    if (temp >= 2200) && (temp <= 3700) { // M-class stars
        subdiv = 10 - ((temp - 2200) / 150);
        // result = format!("M {}", subdiv);
    } else if (temp >= 3700) && (temp <= 5200) { // K-class stars
        subdiv = 10 - ((temp - 3700) / 150);
        // format!("K {}", subdiv);
    } else if (temp >= 5200) && (temp <= 6000) { // G-class stars
        subdiv = 10 - ((temp - 5200) / 80);
        // format!("G {}", subdiv);
    } else if (temp >= 6000) && (temp <= 7500) { // F-class stars
        subdiv = 10 - ((temp - 6000) / 150);
        // format!("F {}", subdiv);
    } else if (temp >= 7500) && (temp <= 10000) { // A-class stars
        subdiv = 10 - ((temp - 7500) / 250);
        // format!("A {}", subdiv);
    } else if (temp >= 10000) && (temp <= 30000) { // B-class stars
        subdiv = 10 - ((temp - 10000) / 2000);
        // format!("B {}", subdiv);
    } else if (temp >= 30000) && (temp <= 55000) { // O-class stars
        subdiv = 10 - ((temp - 30000) / 2500);
        // format!("O {}", subdiv);
    }
    return subdiv;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spectral_class() {
        assert_eq!(get_spectral_class("O"), "Blue");
        assert_eq!(get_spectral_class("B"), "Blue-White");
        assert_eq!(get_spectral_class("Z"), "None");
    }

    #[test]
    fn luminosity_class() {
        assert_eq!(get_luminosity_class("III"), "Giant");
        assert_eq!(get_luminosity_class("Ib"), "Supergiant");
    }

    #[test]
    fn main_sequence() {
        let star = create_star_from_bv(0.749927);

        assert_eq!(star.radius, 0.9470525016124451);
        assert_eq!(star.luminosity, 0.6357974611951837);
        assert_eq!(star.temp, 5436);
        assert_eq!(star.mass, 0.8929552548605576);
        assert_eq!(star.spectral_class, 8);
    }
}
