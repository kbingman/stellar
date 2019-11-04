// use crate::models::{Star};
use rand::prelude::*;

// Returns the name for each Spectal Class
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

// Returns the name for each Spectal Class
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

// Generate weighted B-V
pub fn get_weighted_bv(mut rng: StdRng) -> f64 {
    let bvfac = rng.gen_range(0.0, 100.0);
    let mut bv = 0.0;
    if bvfac <= 0.01 {
        bv = rng.gen_range(-0.33, -0.3);
    } else if (bvfac >= 0.01) && (bvfac <= 0.13) {
        bv = rng.gen_range(-0.3, -0.02);
    } else if (bvfac >= 0.13) && (bvfac <= 0.73) {
        bv = rng.gen_range(-0.02, 0.3);
    } else if (bvfac >= 0.73) && (bvfac <= 3.73) {
        bv = rng.gen_range(0.3, 0.58);
    } else if (bvfac >= 3.73) && (bvfac <= 11.33) {
        bv = rng.gen_range(0.58, 0.81);
    } else if (bvfac >= 11.33) && (bvfac <= 23.43) {
        bv = rng.gen_range(0.81, 1.40);
    } else if (bvfac >= 23.43) && (bvfac <= 100.0) {
        bv = rng.gen_range(1.40, 3.17);
    }
    return bv;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::create_seeded_rng;

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
    fn bv() {
        let rng = create_seeded_rng("miseryofbodiesofcities");
        let bv = get_weighted_bv(rng);

        assert_eq!(bv, 1.6354379193425592);
    }
}
