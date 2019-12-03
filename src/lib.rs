#[macro_use]
extern crate arrayref;

mod models;
mod star;
mod util;

pub use crate::star::Star;
// use crate::star::{
//     bv_to_luminosity, bv_to_radius, bv_to_temp, get_spectral_class, get_spectral_class_from_temp,
//     luminosity_to_mass,
// };
// 
// /**
//  * Creates a Star from a B-V value
//  */
// pub fn create_star_from_bv(bv: f64, luminosity_class: &str) -> Star {
//     let luminosity = bv_to_luminosity(bv, luminosity_class);
//     let temp = bv_to_temp(bv);
//     let spectral_class = &mut get_spectral_class_from_temp(temp);
// 
//     Star {
//         temp,
//         luminosity,
//         radius: bv_to_radius(bv, luminosity_class),
//         mass: luminosity_to_mass(luminosity),
//         spectral_class: spectral_class.to_string(),
//         luminosity_class: luminosity_class.to_owned(),
//         color: get_spectral_class(&spectral_class),
//     }
// }
