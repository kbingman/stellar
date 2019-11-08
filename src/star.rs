use crate::models::Star;
// use std::fmt;

/**
 * Returns the name for each Spectal Class
 */
pub fn get_spectral_class(spectral_class: &str) -> &str {
    let spec: &str = &spectral_class.chars().next().unwrap().to_string();
    match spec {
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
    const LA1: f64 = 0.3302;
    const LB1: f64 = 1.8946;
    const LC1: f64 = 0.4347;

    // branch 2: 0.43 < mass < 2
    const LA2: f64 = 16.0;
    const LB2: f64 = 1.0;
    const LC2: f64 = 0.25;

    // branch 3: 2 < mass < 20
    const LA3: f64 = 50087.9;
    const LB3: f64 = 0.9038;
    const LC3: f64 = 0.25;

    // branch 4: 20 < mass < 55
    const LA4: f64 = 1233870.0;
    const LB4: f64 = 1.0;
    const LC4: f64 = 0.2857;

    // branch 5: mass > 55
    const LA5: f64 = 32000.0;

    match lum {
        l if l < LA1 => LB1 * lum.powf(LC1),
        l if l > LA1 && l < LA2 => LB2 * lum.powf(LC2),
        l if l > LA2 && l < LA3 => LB3 * lum.powf(LC3),
        l if l > LA3 && l < LA4 => LB4 * lum.powf(LC4),
        l if l > LA4 => lum * LA5,
        _ => 0.0,
    }
}

/**
 * Main Sequence specific functions
 * Generate luminosity from B-V
 *
 * pow(10, (ba1 + (bb1 * log(bc1 / (bv + bd1)))))
 */
pub fn ms_bv_to_radius(bv: f64) -> f64 {
    const BA1: f64 = -5.793;
    const BB1: f64 = 0.6729;
    const BC1: f64 = 7360.9;
    const BD1: f64 = 0.6411;

    10_f64.powf(BA1 + (BB1 * (BC1 / (bv + BD1)).ln()))
}

/**
 * Main Sequence specific functions
 * Generate luminosity from B-V
 *
 * pow(10, (ba2 + (bb2 * log(bc1 / (bv + bd1)))
 */
pub fn ms_bv_to_luminosity(bv: f64) -> f64 {
    const BA2: f64 = -26.63;
    const BB2: f64 = 3.083;
    const BC1: f64 = 7360.9;
    const BD1: f64 = 0.6411;

    10_f64.powf(BA2 + (BB2 * (BC1 / (bv + BD1)).ln()))
}

/**
 * Super Giant specific functions
 * generate radius from B-V
 *
 * pow(10, (ba2 + (bb2 * log(bc1 / (bv + bd1)))
 */
pub fn giant_bv_to_radius(bv: f64) -> f64 {
    let ba3 = 22.25;
    let bb3 = -2.502;
    let bc2 = 8627.59;
    let bd2 = 0.7920;

    10_f64.powf(ba3 + (bb3 * (bc2 / (bv + bd2).ln())))
}

/**
 * Super Giant specific functions
 * generate luminosity from B-V
 * pow(10, (ba2 + (bb2 * log(bc1 / (bv + bd1)))
 */
pub fn giant_bv_to_luminosity(bv: f64) -> f64 {
    let ba4 = 29.469;
    let bb4 = -3.2676;
    let bc2 = 8627.59;
    let bd2 = 0.7920;

    10_f64.powf(ba4 + (bb4 * (bc2 / (bv + bd2)).ln()))
}

/**
 * Creates a Star from a B-V value 
 */
pub fn create_star_from_bv(bv: f64) -> Star {
    let luminosity = ms_bv_to_luminosity(bv);
    let temp = bv_to_temp(bv);
    let spectral_class = &mut get_spectral_class_from_temp(temp);

    Star {
        temp,
        luminosity,
        radius: ms_bv_to_radius(bv),
        mass: luminosity_to_mass(luminosity),
        spectral_class: spectral_class.to_string(),
        luminosity_class: "V".to_string(),
        color: get_spectral_class(&spectral_class).to_string(),
    }
}

/**
 * Matches the temp to the Spectral Class of a star
 */
pub fn get_spectral_class_from_temp(temp: u32) -> String {
    match temp {
        t if t >= 2200 && t <= 3700 => format!("M{}", 10 - ((temp - 2200) / 150)),
        t if t >= 3700 && t <= 5200 => format!("K{}", 10 - ((temp - 3700) / 150)),
        t if t >= 5200 && t <= 6000 => format!("G{}", 10 - ((temp - 5200) / 80)),
        t if t >= 6000 && t <= 7500 => format!("F{}", 10 - ((temp - 6000) / 150)),
        t if t >= 7500 && t <= 10000 => format!("A{}", 10 - ((temp - 7500) / 250)),
        t if t >= 10000 && t <= 30000 => format!("B{}", 10 - ((temp - 10000) / 2000)),
        t if t >= 30000 && t <= 50000 => format!("O{}", 10 - ((temp - 30000) / 2500)),
        _ => format!("{}", 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminosity_class() {
        assert_eq!(get_luminosity_class("III"), "Giant");
        assert_eq!(get_luminosity_class("Ib"), "Supergiant");
    }

    #[test]
    fn g_main_sequence() {
        let star = create_star_from_bv(0.749927);

        assert_eq!(star.radius, 0.9470525016124451);
        assert_eq!(star.luminosity, 0.6357974611951837);
        assert_eq!(star.temp, 5436);
        assert_eq!(star.mass, 0.8929552548605576);
        assert_eq!(star.spectral_class, "G8");
        assert_eq!(star.color, "Yellow");
    }
    
    #[test]
    fn f_main_sequence() {
        let star = create_star_from_bv(0.32);

        assert_eq!(star.radius, 1.679415144317238);
        assert_eq!(star.luminosity, 8.773298365605115);
        assert_eq!(star.temp, 7337);
        assert_eq!(star.mass, 1.721039051301309);
        assert_eq!(star.spectral_class, "F2");
        assert_eq!(star.color, "Yellow-White");
    }

    #[test]
    fn a_main_sequence() {
        let star = create_star_from_bv(0.1715);

        assert_eq!(star.radius, 2.1781974741627175);
        assert_eq!(star.luminosity, 28.88098565224987);
        assert_eq!(star.temp, 8390);
        assert_eq!(star.mass, 2.095199296960982);
        assert_eq!(star.spectral_class, "A7");
        assert_eq!(star.color, "White");
    }

    #[test]
    fn m_main_sequence() {
        let star = create_star_from_bv(1.47);

        assert_eq!(star.radius, 0.4962065645977362);
        assert_eq!(star.luminosity, 0.03289981815369502);
        assert_eq!(star.temp, 3839);
        assert_eq!(star.mass, 0.42947861739265913);
        assert_eq!(star.spectral_class, "K10");
        assert_eq!(star.color, "Orange");
    }

    #[test]
    fn b_main_sequence() {
        let star = create_star_from_bv(-0.033);

        assert_eq!(star.radius, 3.413270590776482);
        assert_eq!(star.luminosity, 226.1375097189035);
        assert_eq!(star.temp, 10556);
        assert_eq!(star.mass, 3.504818142850233);
        assert_eq!(star.spectral_class, "B10");
        assert_eq!(star.color, "Blue-White");
    }

    #[test]
    fn o_main_sequence() {
        let star = create_star_from_bv(-0.31);

        assert_eq!(star.radius, 8.7546612237247);
        assert_eq!(star.luminosity, 16927.256163578622);
        assert_eq!(star.temp, 16991);
        assert_eq!(star.mass, 10.309057895579505);
        assert_eq!(star.spectral_class, "B7");
        assert_eq!(star.color, "Blue-White");
    }
}
