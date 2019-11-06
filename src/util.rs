extern crate rand;
extern crate sha2;

use crate::models::{Coords, SphereCoords};
use rand::prelude::*;
use sha2::{Digest, Sha256};

/**
 * Turn a string into a string 64 characters in length
 */
fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());

    format!("{:x}", hasher.result())
}

/**
 * Returns a new seed Random Number Generator with the given string
 */
pub fn create_seeded_rng(text: &str) -> StdRng {
    let hash = create_hash(text);
    // turn the hashed string into the correct length and type required by SeedableRng
    let seed = array_ref!(hash.as_bytes(), 0, 32);

    SeedableRng::from_seed(*seed)
}

/**
 * spherical to 3-dimensional Cartesian coordinates
 */
pub fn spherical_to_cartesian(sphere: SphereCoords) -> Coords {
    Coords {
        x: sphere.r * sphere.i.sin() * sphere.a.cos(),
        y: sphere.r * sphere.i.sin() * sphere.a.sin(),
        z: sphere.r * sphere.i.cos(),
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

    #[test]
    fn weighted_bv() {
        let mut rng = create_seeded_rng("miseryofbodiesofcities");
        let bv = get_weighted_bv(&mut rng);

        assert_eq!(bv, 1.6354379193425592);
    }
}
