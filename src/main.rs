mod model;
mod test;
mod helper;
mod controller;

#[macro_use]
extern crate rocket;
extern crate rocket_codegen;

use controller::number_transform_controller::{phonetic_number,phonetic_number_translation_readme};
use controller::main_controller::main_index;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![phonetic_number,phonetic_number_translation_readme,main_index])
}