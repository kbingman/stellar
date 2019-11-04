extern crate clap;
extern crate rand;
extern crate sha2;

use clap::{App, Arg};
use rand::prelude::*;

#[macro_use]
extern crate arrayref;

mod util;
mod models;

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

    let mut rng: StdRng = util::create_seeded_rng("missmodular");
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 1.0));
}
