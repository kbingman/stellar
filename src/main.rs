extern crate clap;
extern crate rand;
extern crate sha2;

#[macro_use]
extern crate arrayref;

mod models;
mod star;
mod util;

use clap::{App, Arg};
use rand::prelude::*;

// use crate::models::SphereCoords;
use crate::star::{create_star_from_bv};
use crate::util::{get_weighted_bv};

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
    // Returns a random integer or a random float with the range give
    let mut rng: StdRng = util::create_seeded_rng("missmodular");

    for _ in 0..100 {
        let bv = get_weighted_bv(&mut rng);
        let star = create_star_from_bv(bv);
        println!("B-V: {:.*}", 3, bv);
        println!("Mass: {:.*}", 3, star.mass);
        println!("Radius: {:.*}", 3, star.radius);
        println!("Luminosity: {:.*}", 3, star.luminosity);
        println!("Spectral Class: {} {}", star.spectral_class,  star.luminosity_class);
        println!("Color: {}", star.color);
        println!(" ");
    }

}
