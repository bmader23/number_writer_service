use rocket::response::content::Json;
use crate::model::phonetic_number::PhoneticNumber;

#[get("/phonetic_number_translation")]
pub fn phonetic_number_translation_readme() -> Json<String> {
    let response = "Please provide numeric string to translate to long form number\n
        Invalid characters will be cleaned from the result and appended to the cleaning_log\n
        A dollar sign at the start of the string will switch the output from numeric to monetary mode";
    return Json(response.to_string());
}

#[get("/phonetic_number_translation/<raw_numeric_value>")]
pub fn phonetic_number(raw_numeric_value: String) -> Json<PhoneticNumber> {
    let response = PhoneticNumber::from(raw_numeric_value);
    return Json(response);
}