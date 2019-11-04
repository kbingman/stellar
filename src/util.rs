extern crate rand;
extern crate sha2;

use crate::models::{Coords, SphereCoords};
use rand::prelude::*;
use sha2::{Digest, Sha256};

// Turn a string into a string 64 characters in length
pub fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());

    format!("{:x}", hasher.result())
}

// Returns a new seed Random Number Generator with the given string
pub fn create_seeded_rng(text: &str) -> StdRng {
    let hash = create_hash(text);
    // turn the hashed string into the correct length and type required by SeedableRng
    let seed = array_ref!(hash.as_bytes(), 0, 32);

    SeedableRng::from_seed(*seed)
}

// spherical to 3-dimensional Cartesian coordinates
pub fn spherical_to_cartesian(sphere: SphereCoords) -> Coords {
    Coords {
        x: sphere.r * sphere.i.sin() * sphere.a.cos(),
        y: sphere.r * sphere.i.sin() * sphere.a.sin(),
        z: sphere.r * sphere.i.cos(),
    }
}

// generate temperature from B-
pub fn bv_to_temp(bv: f64) -> f64 {
    (4600.0 * ((1.0 / ((0.92 * bv) + 1.7)) + (1.0 / ((0.92 * bv) + 0.62))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_range() {
        let mut rng = create_seeded_rng("meimcounting");
        let num = rng.gen_range(0.0, 1.0);

        assert_eq!(num > 0.0, true);
        assert_eq!(num < 1.0, true);
    }

    #[test]
    fn sperical_coords() {
        let sphere = SphereCoords {
            r: 1.0,
            i: 1.0,
            a: 1.0,
        };
        let coords = spherical_to_cartesian(sphere);

        assert_eq!(coords.x, 0.4546487134128409);
        assert_eq!(coords.y, 0.7080734182735712);
        assert_eq!(coords.z, 0.5403023058681398);
    }
}
