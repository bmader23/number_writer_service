use rocket::response::content::Json;
use crate::model::phonetic_number::PhoneticNumber;



#[get("/phonetic_number_translation/<raw_numeric_value>")]
pub fn phonetic_number(raw_numeric_value: String) -> Json<PhoneticNumber> {
    let response = PhoneticNumber::from(raw_numeric_value);
    return Json(response);
}