use rocket::response::content::Json;



#[get("/")]
pub fn main_index() -> Json<String> {
    let response = "Index:\nLong Form Number Endpoint: /phonetic_number_translation/<raw_numeric_value>";
    return Json(response.to_string());
}