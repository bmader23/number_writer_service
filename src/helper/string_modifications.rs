pub fn concatenate_strings(prepended_string: &str, appended_string: &str) -> String{
    let mut new_string = prepended_string.to_string();
    new_string.push_str(appended_string);
    return new_string;
}