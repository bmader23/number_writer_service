use core::num;
use std::io::Cursor;

use rocket::http::ContentType;
use rocket::response::{self, Response, Responder};
use rocket::request::Request;

// TODO
// use serde::ser::SerializeStruct;
// use serde::{Serialize, Serializer};

macro_rules! phonetic_number_write_cleaning_entry {
    ($phonetic_result: ident, $exception_string: expr)=>{
        println!("{}", $exception_string);
        $phonetic_result.add_cleaning_entry(&$exception_string);
    }
}
macro_rules! phonetic_number_create_cleaning_entry{
    ($phonetic_result: ident) => {
        let cleaning_entry = format!("Invalid inclusion in parameter provided (non-specific) - Ignored");
        phonetic_number_write_cleaning_entry!($phonetic_result, cleaning_entry)
    };

    ($phonetic_result: ident, $exception_customizer: expr, $exception_position: expr,$exception_character: expr)=>{
        let cleaning_entry = format!("Invalid {} {{\\\"{}\\\"}} (position: {}) - Ignored", 
            $exception_customizer,
            $exception_character, 
            $exception_position);
            phonetic_number_write_cleaning_entry!($phonetic_result, cleaning_entry)
    }
    
}

use crate::helper::{number_parts_catalog::{
    find_exception_to_double_digit_pattern, 
    find_unique_naming_exception, 
    get_ones_place, 
    get_tens_place,
    get_magnitude_indicator,
    get_decimal_magnitude_indicator
}, string_modifications::concatenate_strings};
use crate::model::phonetic_number;

pub struct PhoneticNumber{
    pub raw_string_value: String,
    phonetic_string:String,
    numeric_string:String,
    dollar_integer: u64,
    cent_integer: u64,
    negative: bool,
    dollar_sign_included: bool,
    decimal_magnitude: u64,
    cleaning_log: String
}

impl<'a> PhoneticNumber{
    fn new(raw_string: String)->PhoneticNumber{
        return PhoneticNumber{
            raw_string_value: raw_string,
            phonetic_string: "".to_string(),
            numeric_string: "".to_string(),
            dollar_integer: 0,
            cent_integer: 0,
            negative: false,
            dollar_sign_included: false,
            decimal_magnitude:0,
            cleaning_log: "".to_string()
        }
    }

    pub fn get_numeric_string(&self) -> &str{
        return &self.numeric_string;
    }
    pub fn get_phonetic_string(&self) -> &str{
        return &self.phonetic_string;
    }
    pub fn get_cleaning_log(&self) -> &str{
        return &self.cleaning_log;
    }

    pub fn is_negative(&self) -> bool{
        return (self.dollar_integer > 0 || self.cent_integer > 0) && self.negative;
    }

    fn generate_phonetic_string_from_object(& mut self) {
        let mut return_val = "".to_string();
        if self.is_negative(){
            return_val.push_str("negative ");
        }
        return_val.push_str(&format!("{}",PhoneticNumber::translate_u64_to_phonetic(self.dollar_integer)));
        if self.dollar_sign_included{
            return_val.push_str(&format!(" dollar{}", &PhoneticNumber::generate_plural_suffix(self.dollar_integer)));
            
            if self.decimal_magnitude > 0{
                let cents = self.cent_integer * 100 / self.decimal_magnitude;
                return_val.push_str(&format!(", and {} cent{}", 
                    &PhoneticNumber::translate_u64_to_phonetic(cents),
                    &PhoneticNumber::generate_plural_suffix(self.cent_integer)
                ));
            }
        }else{
            if self.decimal_magnitude > 0{
                let mut decimal_magnitude_indicator = "".to_string();

                match get_decimal_magnitude_indicator(self.decimal_magnitude){
                    Some(x) => { 
                        decimal_magnitude_indicator.push_str(&x);
                     }
                    None => {}
                }

                return_val.push_str(&format!(", and {} {}{}", 
                    &PhoneticNumber::translate_u64_to_phonetic(self.cent_integer), 
                    decimal_magnitude_indicator,
                    PhoneticNumber::generate_plural_suffix(self.cent_integer)
                ));
            }
        }
        self.phonetic_string = return_val;
    }
    fn generate_numeric_string_from_object(& mut self) {
        let mut numeric_string = "".to_string();
        if self.is_negative(){ 
            numeric_string.push_str(&'-'.to_string());
        }
        if self.dollar_sign_included{
            numeric_string.push_str(&'$'.to_string());
        }
        numeric_string.push_str(&self.dollar_integer.to_string());

        if self.decimal_magnitude > 0{
            let mut leading_zeros = "".to_string();
            
            let mut magnitude_disparity = 0;
            if self.cent_integer > 0{
                magnitude_disparity = (self.decimal_magnitude * 10) / self.cent_integer;
            }

            while magnitude_disparity > 100{
                leading_zeros.push_str("0");
                magnitude_disparity /= 10;
            }
            numeric_string.push_str(&format!(".{}{}", leading_zeros, self.cent_integer.to_string()));
        }

        self.numeric_string = numeric_string;
    }

    fn set_decimal_magnitude(& mut self, new_val: u64){
        self.decimal_magnitude = new_val;
    }
    fn set_dollar_sign_included(& mut self, new_val: bool){
        self.dollar_sign_included = new_val;
    }
    fn set_negative(& mut self, new_val: bool){
        self.negative = new_val;
    }

    fn clear_dollar_integer(& mut self){
        self.dollar_integer = 0;
    }

    fn increment_dollar_integer(& mut self, increment_value: u64){
        self.dollar_integer += increment_value;
    }
    fn increment_cent_integer(& mut self, increment_value: u64){
        self.cent_integer += increment_value;
    }

    fn add_cleaning_entry(& mut self, add_entry: &str){
        if self.cleaning_log.len() > 0{
            self.cleaning_log.push_str("||");
        }
        self.cleaning_log.push_str(add_entry);
    }

    fn generate_plural_suffix(check_plural: u64) -> String{
        match check_plural{
            1 => {
                return "".to_string();
            }
            _not_singular => {
                return "s".to_string();       
            }
        }
    }

    fn translate_u64_to_phonetic(translatee: u64) -> String{
        let mut return_val = "".to_string();
        let mut processed_translatee = translatee;
        let mut i = 0;
        
        match find_unique_naming_exception(processed_translatee){
            Some(x) => { return x; }
            None => {}
        }

        loop {
        
            match PhoneticNumber::translate_three_digits(processed_translatee % 1000){
                Some(x) => {
                    match get_magnitude_indicator(i){
                        Some(magnitude) => {
                            if !x.eq("zero") || magnitude.is_empty(){
                                let mut prepend_string = format!("{}{}", &x, magnitude);
                                if !return_val.is_empty(){
                                    prepend_string.push_str(" ");
                                }
                                return_val = concatenate_strings(&prepend_string, &return_val);
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
            processed_translatee /= 1000;
            i+=1;

            if processed_translatee <= 0{
                break;
            }
        }
        
        return return_val;
    }
    fn translate_three_digits(translatee: u64) -> Option<String>{
        if translatee > 999{
            return None;
        }

        let mut return_string = "".to_string();
        let mut ones_place = "".to_string();
        let mut tens_place = "".to_string();
        let mut hundreds_place  = "".to_string();
        let mut processed_value = translatee;

        match find_exception_to_double_digit_pattern(processed_value % 100){
            Some(x) => {
                ones_place.push_str(&x);
                processed_value /= 100;
            }
            None => {
                match get_ones_place(processed_value % 10){
                    Some(x) => {ones_place.push_str(&x);}
                    None => {}
                }
                processed_value /= 10;
                
                match get_tens_place(processed_value % 10){
                    Some(x) => {tens_place.push_str(&x);}
                    None => {}
                }
                processed_value /= 10;
            }
        }
        
        match get_ones_place(processed_value){
            Some(x) => {hundreds_place.push_str(&x);}
            None => {}
        }

        if !hundreds_place.is_empty(){
            return_string.push_str(&format!("{} hundred", hundreds_place));
        }

        if !tens_place.is_empty(){
            if !return_string.is_empty(){
                return_string.push_str(" ")
            }
            return_string.push_str(&format!("{}", tens_place));
        }

        if !ones_place.is_empty(){
            if !return_string.is_empty(){
                return_string.push_str(" ")
            }
            return_string.push_str(&format!("{}", ones_place));
        }
        return Some(return_string);
    }

}

impl<'r> Responder<'r, 'static> for PhoneticNumber {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        let body_content = self.to_string();
        Response::build()
        .sized_body(body_content.len(), Cursor::new(body_content))
        .header(ContentType::JSON)
        .ok()
    }
}

// TODO
// impl Serialize for PhoneticNumber{
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         println!("Serializer");
//         let mut s = serializer.serialize_struct("phonetic_number", 3)?;
//         s.serialize_field("numeric_string", &self.numeric_string)?;
//         s.serialize_field("phonetic_string", &self.phonetic_string)?;
//         s.serialize_field("cleaning_log", &self.cleaning_log)?;
//         s.end()
//     }
// }

impl ToString for PhoneticNumber{
    fn to_string(&self) -> String {
        return format!("{{\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"}}",
            "raw_string", &self.raw_string_value,
            "numeric_string", &self.numeric_string,
            "phonetic_string", &self.phonetic_string,
            "cleaning_log", &self.cleaning_log
        )
    }
}
impl From<String> for PhoneticNumber{
    
    fn from(src_string: String) -> Self {
        let mut src_char_indices = src_string.char_indices();
        let mut magnitude_indicator = 1;
        let mut phonetic_result = PhoneticNumber::new(src_string.to_owned());

        let mut pending_leftbound_modifiers = Vec::<(usize,char)>::new();

        loop {
            let index = src_char_indices.next_back();
            if index == None{
                break;
            }

            match index{
                Some(x) if x.1.is_ascii_digit() => {
                    println!("Valid: {}",x.1);
                    match x.1.to_digit(10){
                        Some(y)=>{
                            phonetic_result.increment_dollar_integer( u64::from(y) * magnitude_indicator);
                            magnitude_indicator *= 10;
                            while pending_leftbound_modifiers.len() > 0{
                                let popped = pending_leftbound_modifiers.pop();
                                match popped{
                                    Some(p) => {
                                        phonetic_number_create_cleaning_entry!(phonetic_result, "left-bound modifier", p.0,p.1);
                                    }
                                    None => {}
                                }
                            }
                        }
                        None => {
                            println!("Failed to convert digit to integer value");
                        }
                    }
                }
                Some(x) if x.1.is_ascii_whitespace()=>{
                    phonetic_number_create_cleaning_entry!(phonetic_result, "whitespace", x.0, x.1);
                }
                Some(x) if x.1 == ','=>{
                    println!("Checking for valid comma at input string position {}", x.0);
                }
                Some(x) if x.1 == '.'=>{
                    if phonetic_result.decimal_magnitude == 0{
                        phonetic_result.set_decimal_magnitude(magnitude_indicator);
                        magnitude_indicator = 1;
                        phonetic_result.increment_cent_integer(phonetic_result.dollar_integer);
                        phonetic_result.clear_dollar_integer();
                    }
                    else{
                        phonetic_number_create_cleaning_entry!(phonetic_result, "decimal point", x.0,x.1);
                    }
                }
                Some(x) if x.1 == '$'=>{
                    pending_leftbound_modifiers.push(x);
                }
                Some(x) if x.1 == '-'=>{
                    pending_leftbound_modifiers.push(x);
                }
                Some(x) if x.1.is_ascii_alphabetic()=>{
                    phonetic_number_create_cleaning_entry!(phonetic_result, "alphabetic character", x.0,x.1);
                }
                Some(x)=> {
                    phonetic_number_create_cleaning_entry!(phonetic_result, "character (non-specific)", x.0,x.1);
                }
                None=>{
                    phonetic_number_create_cleaning_entry!(phonetic_result);
                }
            }
        }
        while pending_leftbound_modifiers.len() > 0 {
            let popped = pending_leftbound_modifiers.pop();
            match popped{
                Some(x) if x.1 == '-' => {
                    phonetic_result.set_negative(true);
                }
                Some(x) if x.1 == '$' => {
                    phonetic_result.set_dollar_sign_included(true);
                }
                Some(_not_defined_as_leftbound_modifier) => {}
                None => {}
            }
        }
        phonetic_result.generate_numeric_string_from_object();
        phonetic_result.generate_phonetic_string_from_object();
        return phonetic_result;
    }
}