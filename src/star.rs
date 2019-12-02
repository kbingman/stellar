use crate::models::Star;

/**
 * Returns the name for each Spectal Class
 */
pub fn get_spectral_class(spectral_class: &str) -> String {
    let spec: &str = &spectral_class.chars().next().unwrap().to_string();
    (match spec {
        "O" => "Blue",
        "B" => "Blue-White",
        "A" => "White",
        "F" => "Yellow-White",
        "G" => "Yellow",
        "K" => "Orange",
        "M" => "Red",
        "D" => "Degenerate",
        _ => "None",
    })
    .to_string()
}

/**
 * Returns the name for each Spectal Class
 */
pub fn get_luminosity_class(luminosity_class: &str) -> String {
    (match luminosity_class {
        "III" => "Giant",
        "Ib" => "Supergiant",
        "Iab" => "Supergiant",
        "Ia" => "Supergiant",
        "Ia+" => "Hypergiant",
        _ => "",
    })
    .to_string()
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
pub fn bv_to_radius(bv: f64, luminosity_class: &str) -> f64 {
    match luminosity_class {
        "I" => bv_to_radius_supergiant(bv),
        "Ia" => bv_to_radius_supergiant_a(bv),
        "III" => bv_to_radius_giant(bv),
        // "V" => bv_to_radius_ms(bv),
        // Defaults to Type V, main sequence
        _ => bv_to_radius_ms(bv),
    }
}

// Type V
fn bv_to_radius_ms(bv: f64) -> f64 {
    const BA1: f64 = -5.793;
    const BB1: f64 = 0.6729;
    const BC1: f64 = 7360.9;
    const BD1: f64 = 0.6411;

    10_f64.powf(BA1 + (BB1 * (BC1 / (bv + BD1)).ln()))
}

// Type III
fn bv_to_radius_giant(bv: f64) -> f64 {
    const BA3: f64 = 22.25;
    const BB3: f64 = -2.502;
    const BC3: f64 = 8527.0;
    const BD3: f64 = 0.7920;

    10_f64.powf(BA3 + (BB3 * (BC3 / (bv + BD3)).ln()))
}

// Type I
fn bv_to_radius_supergiant(bv: f64) -> f64 {
    const BA2: f64 = 8.417;
    const BB2: f64 = 0.7129;
    const BC2: f64 = 8261.0;
    const BD2: f64 = 0.7491;

    10_f64.powf(BA2 + (BB2 * (BC2 / (bv + BD2)).ln()))
}

// Type Ia
fn bv_to_radius_supergiant_a(bv: f64) -> f64 {
    const BA2: f64 = 12.71;
    const BB2: f64 = -1.21;
    const BC2: f64 = 8261.0;
    const BD2: f64 = 0.7491;

    10_f64.powf(BA2 + (BB2 * (BC2 / (bv + BD2)).ln()))
}

/**
 * Main Sequence specific functions
 * Generate luminosity from B-V
 *
 * pow(10, (ba2 + (bb2 * log(bc1 / (bv + bd1)))
 * log10(L) = −26.63 + L⊙ +3.083 ln 􏰀7360.9 ((B − V) + 0.6411)−1􏰁
 */
pub fn bv_to_luminosity(bv: f64, luminosity_class: &str) -> f64 {
    match luminosity_class {
        "I" => bv_to_luminosity_supergiant(bv),
        "Ia" => bv_to_luminosity_supergiant_a(bv),
        "III" => bv_to_luminosity_giant(bv),
        // "V" => bv_to_luminosity_ms(bv),
        // Defaults to Type V, main sequence
        _ => bv_to_luminosity_ms(bv),
    }
}

// Type V
fn bv_to_luminosity_ms(bv: f64) -> f64 {
    const BA1: f64 = -26.63;
    const BB1: f64 = 3.083;
    const BC1: f64 = 7360.9;
    const BD1: f64 = 0.6411;

    10_f64.powf(BA1 + (BB1 * (BC1 / (bv + BD1)).ln()))
}

// Type III
fn bv_to_luminosity_giant(bv: f64) -> f64 {
    const BA1: f64 = 29.469;
    const BB1: f64 = -3.2676;
    const BC1: f64 = 8527.59;
    const BD1: f64 = 0.7920;

    10_f64.powf(BA1 + (BB1 * (BC1 / (bv + BD1)).ln()))
}

// Type I
fn bv_to_luminosity_supergiant(bv: f64) -> f64 {
    const BA1: f64 = 1.7881;
    const BB1: f64 = 0.3112;
    const BC1: f64 = 8261.19;
    const BD1: f64 = 0.7491;

    10_f64.powf(BA1 + (BB1 * (BC1 / (bv + BD1)).ln()))
}

// Type Ia
fn bv_to_luminosity_supergiant_a(bv: f64) -> f64 {
    const BA1: f64 = 10.392;
    const BB1: f64 = -0.682;
    const BC1: f64 = 8261.19;
    const BD1: f64 = 0.7491;

    10_f64.powf(BA1 + (BB1 * (BC1 / (bv + BD1)).ln()))
}

/**
 * Creates a Star from a B-V value
 */
pub fn create_star_from_bv(bv: f64, luminosity_class: &str) -> Star {
    let luminosity = bv_to_luminosity(bv, luminosity_class);
    let temp = bv_to_temp(bv);
    let spectral_class = &mut get_spectral_class_from_temp(temp);

    Star {
        temp,
        luminosity,
        radius: bv_to_radius(bv, luminosity_class),
        mass: luminosity_to_mass(luminosity),
        spectral_class: spectral_class.to_string(),
        luminosity_class: luminosity_class.to_owned(),
        color: get_spectral_class(&spectral_class),
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
    fn bv_to_radius_ms_test() {
        assert_eq!(bv_to_radius(0.94, "V"), 0.7765868702366475);
    }

    #[test]
    fn bv_to_radius_giant_test() {
        assert_eq!(bv_to_radius(0.94, "III"), 9.521522463680254);
    }

    #[test]
    fn bv_to_radius_supergiant_test() {
        assert_eq!(bv_to_radius(1.24, "I"), 227306018637405.8);
    }

    #[test]
    fn bv_to_luminosity_ms_test() {
        assert_eq!(bv_to_luminosity(0.94, "V"), 0.25612550220548286);
    }

    #[test]
    fn bv_to_luminosity_giant_test() {
        assert_eq!(bv_to_luminosity(0.94, "III"), 48.816483341024586);
    }

    #[test]
    fn bv_to_luminosity_supergiant_test() {
        assert_eq!(bv_to_luminosity(1.24, "I"), 24038.47325813354);
    }

    #[test]
    fn g_main_sequence() {
        let star = create_star_from_bv(0.749927, "V");

        assert_eq!(star.radius, 0.9470525016124451);
        assert_eq!(star.luminosity, 0.6357974611951837);
        assert_eq!(star.temp, 5436);
        assert_eq!(star.mass, 0.8929552548605576);
        assert_eq!(star.spectral_class, "G8");
        assert_eq!(star.color, "Yellow");
    }

    #[test]
    fn f_main_sequence() {
        let star = create_star_from_bv(0.32, "V");

        assert_eq!(star.radius, 1.679415144317238);
        assert_eq!(star.luminosity, 8.773298365605115);
        assert_eq!(star.temp, 7337);
        assert_eq!(star.mass, 1.721039051301309);
        assert_eq!(star.spectral_class, "F2");
        assert_eq!(star.color, "Yellow-White");
    }

    #[test]
    fn a_main_sequence() {
        let star = create_star_from_bv(0.1715, "V");

        assert_eq!(star.radius, 2.1781974741627175);
        assert_eq!(star.luminosity, 28.88098565224987);
        assert_eq!(star.temp, 8390);
        assert_eq!(star.mass, 2.095199296960982);
        assert_eq!(star.spectral_class, "A7");
        assert_eq!(star.color, "White");
    }

    #[test]
    fn m_main_sequence() {
        let star = create_star_from_bv(1.47, "V");

        assert_eq!(star.radius, 0.4962065645977362);
        assert_eq!(star.luminosity, 0.03289981815369502);
        assert_eq!(star.temp, 3839);
        assert_eq!(star.mass, 0.42947861739265913);
        assert_eq!(star.spectral_class, "K10");
        assert_eq!(star.color, "Orange");
    }

    #[test]
    fn g_giant() {
        let star = create_star_from_bv(0.63, "III");

        assert_eq!(star.radius, 3.0568967802362197, "radius");
        assert_eq!(star.luminosity, 11.07018795890346, "luminosity");
        assert_eq!(star.temp, 5853);
        assert_eq!(star.mass, 1.8240584414821335);
        assert_eq!(star.spectral_class, "G2");
        assert_eq!(star.color, "Yellow");
    }

    #[test]
    fn b_main_sequence() {
        let star = create_star_from_bv(-0.033, "V");

        assert_eq!(star.radius, 3.413270590776482);
        assert_eq!(star.luminosity, 226.1375097189035);
        assert_eq!(star.temp, 10556);
        assert_eq!(star.mass, 3.504818142850233);
        assert_eq!(star.spectral_class, "B10");
        assert_eq!(star.color, "Blue-White");
    }

    #[test]
    fn o_main_sequence() {
        let star = create_star_from_bv(-0.31, "V");

        assert_eq!(star.radius, 8.7546612237247);
        assert_eq!(star.luminosity, 16927.256163578622);
        assert_eq!(star.temp, 16991);
        assert_eq!(star.mass, 10.309057895579505);
        assert_eq!(star.spectral_class, "B7");
        assert_eq!(star.color, "Blue-White");
    }
}
