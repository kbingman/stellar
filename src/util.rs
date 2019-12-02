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
 * Generate weighted Luminosity classes
 *
 * Creates a properly distributed set of Luminosity classes based on
 * local distribution of Giant (type III) and Supergiant (type I)
 * and Main Sequence dwarf stars(type V)
 */
pub fn get_weighted_luminosity_class(rng: &mut StdRng) -> String {
    let lum_factor = rng.gen_range(0.0, 100.0);
    (match lum_factor {
        // l if l <= 0.00025 => "0",
        l if l <= 0.00025 => "Ia",
        l if l >= 0.00025 && l <= 0.0005 => "I",
        // l if l >= 0.0005 && l <= 1.0 => "II",
        l if l >= 0.0005 && l <= 8.5 => "III",
        // l if l >= 1.0 && l <= 8.5 => "III",
        l if l >= 8.5 && l <= 100.0 => "V",
        _ => "V",
    })
    .to_string()
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
        b if b >= 0.01 && b <= 0.13 => rng.gen_range(-0.3, -0.02),
        b if b >= 0.13 && b <= 0.73 => rng.gen_range(-0.02, 0.3),
        b if b >= 0.73 && b <= 3.73 => rng.gen_range(0.3, 0.58),
        b if b >= 3.73 && b <= 11.33 => rng.gen_range(0.58, 0.81),
        b if b >= 11.33 && b <= 22.43 => rng.gen_range(0.81, 1.40),
        b if b >= 22.43 && b <= 100.0 => rng.gen_range(1.40, 3.17),
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        //  for _ in 0..10 {
        //    let bv = get_weighted_bv(&mut rng);
        //    println!("bv: {}", bv);
        //  }
        assert_eq!(bv, 1.6354379193425592);
    }

    #[test]
    fn weighted_luminosity_flags() {
        // struct Types<'a> {
        //     type_i: &'a u32,
        //     type_ia: &'a u32,
        //     type_ii: &'a u32,
        //     type_iii: &'a u32,
        //     type_iv: &'a u32,
        //     type_v: &'a u32,
        //     type_vi: &'a u32,
        // }

        let mut rng = create_seeded_rng("thestarsmydestination");
        let mut type_o = 0;
        let mut type_i = 0;
        let mut type_ia = 0;
        let mut type_ii = 0;
        let mut type_iii = 0;
        // let mut type_iv = 0;
        let mut type_v = 0;
        // let mut type_vi = 0;

        for _ in 0..500000 {
            let mkk = get_weighted_luminosity_class(&mut rng);
            // println!("{}, bkk: {}", i, mkk);
            if mkk == "O" {
                type_o = type_o + 1;
            }
            if mkk == "I" {
                type_i = type_i + 1;
            }
            if mkk == "Ia" {
                type_ia = type_ia + 1;
            }
            if mkk == "II" {
                type_ii = type_ii + 1;
            }
            if mkk == "III" {
                type_iii = type_iii + 1;
            }
            if mkk == "V" {
                type_v = type_v + 1;
            }
        }
        assert_eq!(type_o, 0);
        assert_eq!(type_i, 2);
        assert_eq!(type_ia, 2);
        assert_eq!(type_ii, 0);
        assert_eq!(type_iii, 42674);
        assert_eq!(type_v, 457322);
    }
}
