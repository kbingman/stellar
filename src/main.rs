extern crate clap;
extern crate rand;
extern crate sha2;

#[macro_use]
extern crate arrayref;

mod models;
mod util;
mod star;

use clap::{App, Arg};
use rand::prelude::*;

use crate::models::SphereCoords;
use crate::util::spherical_to_cartesian;
use crate::star::{get_spectral_class, get_luminosity_class};

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .about("Build stars!")
        .author("K. Bingman")
        .arg(
            Arg::with_name("mass")
                .short("m")
                .long("mass")
                .value_name("MASS")
                .help("The stellar mass in Solar Masses")
                .takes_value(true),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let mass = matches.value_of("mass").unwrap();
    println!("Value for mass: {}", mass);

    // Returns a random integer or a random float with the range give
    let mut rng: StdRng = util::create_seeded_rng("missmodular");
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 1.0));

    let spec = get_spectral_class("B");
    println!("Class: {}", spec);

    let lum = get_luminosity_class("Ia");
    println!("Class: {}", lum);

    let sphere = SphereCoords { r: 0.0, i: 0.0, a: 0.0, };
    let coords = spherical_to_cartesian(sphere);
    println!("coords: {}, {}, {}", coords.x, coords.y, coords.z);
}
