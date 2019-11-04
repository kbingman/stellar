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