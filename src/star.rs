// use crate::models::{Star};

// Returns the name for each Spectal Class
pub fn get_spectral_class (spectral_class: &str) -> &str {
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
pub fn get_luminosity_class (luminosity_class: &str) -> &str {
    match luminosity_class {
        "III" => "Giant",
        "Ib" => "Supergiant",
        "Iab" => "Supergiant",
        "Ia" => "Supergiant",
        "Ia+" => "Hypergiant",
        _ => "None",
    }
}
