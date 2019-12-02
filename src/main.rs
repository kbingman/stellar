#[macro_use]
extern crate arrayref;

mod models;
mod star;
mod util;

// use clap::App;
use rand::prelude::StdRng;

// use crate::models::SphereCoords;
use star::create_star_from_bv;
use util::get_weighted_bv;

fn main() {
    // App::new("myapp")
    //     .version("1.0")
    //     .about("Build stars!")
    //     .author("K. Bingman");

    let mut rng: StdRng = util::create_seeded_rng("missmodular");

    for _ in 0..10 {
        let bv = get_weighted_bv(&mut rng);
        let star = create_star_from_bv(bv, "V");

        println!("B-V: {:.*}", 3, bv);
        println!("Mass: {:.*}", 3, star.mass);
        println!("Radius: {:.*}", 3, star.radius);
        println!("Luminosity: {:.*}", 3, star.luminosity);
        println!(
            "Spectral Class: {} {}",
            star.spectral_class, star.luminosity_class
        );
        println!("Color: {}", star.color);
        println!(" ");
    }
}
