// Basic Stellar model
pub struct Star {
    pub mass: f64,                //·mass·of·star·in·M_sol¬
    pub radius: f64,              //·radius·of·star·in·R_sol¬
    pub luminosity: f64,          //·luminosity·of·star·in·L_sol¬
    pub temp: u32,                //·temperature·of·star·in·K¬
    pub spectral_class: String,   //·spectral·class·of·star¬
    pub luminosity_class: String, //·luminosity·class·of·star¬
    pub color: String,
    // pub coords: Coords,         //·coordinates·of·star·wrt·center¬
    // pub age: f64,               //·age·of·the·star·in·Gyr¬
    // pub lifespan: f64,          //·calculated·main·sequence·lifespan·of·the·star·in·Gyr¬
    // pub flags,                  //·flags·added·during·aging·pass¬
}

impl Star {
    pub fn create_from_bv(bv: f64, luminosity_class: &str) -> Star {
        let luminosity = Star::bv_to_luminosity(bv, luminosity_class);
        let temp = Star::bv_to_temp(bv);
        let spectral_class = &mut Star::get_spectral_class_from_temp(temp);

        Star {
           luminosity,
           temp, 
           radius: Star::bv_to_radius(bv, luminosity_class),
           mass: Star::luminosity_to_mass(luminosity),
           spectral_class: spectral_class.to_string(),
           luminosity_class: luminosity_class.to_owned(),
           color: Star::get_spectral_class(&spectral_class),
        }
    }

    fn bv_to_luminosity(bv: f64, luminosity_class: &str) -> f64 {
        match luminosity_class {
            l if l == "I" && -0.27 <= bv && bv <= 0.76 => Star::bv_to_luminosity_supergiant(bv),
            l if l == "I" && 0.76 <= bv && bv <= 1.80 => Star::bv_to_luminosity_supergiant_a(bv),
            l if l == "III" && 0.86 <= bv && bv <= 1.33 => Star::bv_to_luminosity_giant(bv),
            // "V" => bv_to_luminosity_ms(bv),
            // Defaults to Type V, main sequence
            _ => Star::bv_to_luminosity_ms(bv),
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

    fn bv_to_temp(bv: f64) -> u32 {
        (4600.0 * ((1.0 / ((0.92 * bv) + 1.7)) + (1.0 / ((0.92 * bv) + 0.62)))).round() as u32
    }

    fn luminosity_to_mass(lum: f64) -> f64 {
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

    fn bv_to_radius(bv: f64, luminosity_class: &str) -> f64 {
        match luminosity_class {
            l if l == "I" && -0.27 <= bv && bv <= 0.76 => Star::bv_to_radius_supergiant(bv),
            l if l == "I" && 0.76 <= bv && bv <= 1.80 => Star::bv_to_radius_supergiant_a(bv),
            l if l == "III" && 0.86 <= bv && bv <= 1.33 => Star::bv_to_radius_giant(bv),
            // "V" => bv_to_radius_ms(bv),
            // Defaults to Type V, main sequence
            _ => Star::bv_to_radius_ms(bv),
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

    // Returns spectral Class
    fn get_spectral_class_from_temp(temp: u32) -> String {
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

    // Spectral Class
    fn get_spectral_class(spectral_class: &str) -> String {
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
}

/**
 * Returns the name for each Spectal Class
 */

/**
 * Returns the name for each Spectral Class
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
        assert_eq!(Star::bv_to_radius(0.94, "V"), 0.7765868702366475);
    }

    #[test]
    fn bv_to_radius_giant_test() {
        assert_eq!(Star::bv_to_radius(0.94, "III"), 9.521522463680254);
    }

    #[test]
    fn bv_to_radius_supergiant_test() {
        assert_eq!(Star::bv_to_radius(1.24, "I"), 425.34510011063895);
    }

    #[test]
    fn bv_to_luminosity_ms_test() {
        assert_eq!(Star::bv_to_luminosity(0.94, "V"), 0.25612550220548286);
    }

    #[test]
    fn bv_to_luminosity_giant_test() {
        assert_eq!(Star::bv_to_luminosity(0.94, "III"), 48.816483341024586);
    }

    #[test]
    fn bv_to_luminosity_supergiant_test() {
        assert_eq!(Star::bv_to_luminosity(1.24, "I"), 51264.93316516539);
    }
}
