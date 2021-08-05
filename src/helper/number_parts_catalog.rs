pub fn find_unique_naming_exception(full_number: u64) ->Option<String>{
    let return_str;
    match full_number{
        0 => {return_str = "zero";  }
        _not_unique_exception => { return None; }
    }
    return Some(return_str.to_string());
}

pub fn find_exception_to_double_digit_pattern(double_digit: u64) -> Option<String>{
    let return_str; 
    match double_digit{
        11 => {return_str = "eleven";   }
        12 => {return_str = "twelve";   }
        13 => {return_str = "thirteen"; }
        14 => {return_str = "fourteen";  }
        15 => {return_str = "fifteen";  }
        16 => {return_str = "sixteen";   }
        17 => {return_str = "seventeen"; }
        18 => {return_str = "eighteen"; }
        19 => {return_str = "nineteen";  }
        _not_an_exception=> { return None; }
    }
    return Some(return_str.to_string());
}

pub fn get_ones_place(single_digit: u64) -> Option<String>{
    let return_str; 
    match single_digit{
        0 => {return_str = "";      }
        1 => {return_str = "one";   }
        2 => {return_str = "two";   }
        3 => {return_str = "three"; }
        4 => {return_str = "four";  }
        5 => {return_str = "five";  }
        6 => {return_str = "six";   }
        7 => {return_str = "seven"; }
        8 => {return_str = "eight"; }
        9 => {return_str = "nine";  }
        _not_single_digit=>{ return None; }
    }
    return Some(return_str.to_string());
}

pub fn get_tens_place(single_digit: u64) -> Option<String>{
    let return_str; 
    match single_digit{
        0 => {return_str = "";          }
        1 => {return_str = "ten";       }
        2 => {return_str = "twenty";    }
        3 => {return_str = "thirty";    }
        4 => {return_str = "forty";     }
        5 => {return_str = "fifty";     }
        6 => {return_str = "sixty";     }
        7 => {return_str = "seventy";   }
        8 => {return_str = "eighty";    }
        9 => {return_str = "ninety";    }
        _not_single_digit=>{ return None;}
    }
    return Some(return_str.to_string());
}

pub fn get_magnitude_indicator(three_digit_iteration: u8) -> Option<String>{
    let return_str;
    match three_digit_iteration {
        0 => { return_str = "";             }
        1 => { return_str = " thousand";     }
        2 => { return_str = " million";      }
        3 => { return_str = " billion";      }
        4 => { return_str = " trillion";     }
        5 => { return_str = " quadrillion";  }
        _three_digit_iteration_not_found => { return None; }
    }
    return Some(return_str.to_string());
}

pub fn get_decimal_magnitude_indicator(single_digit_iteration: u64) -> Option<String>{
    let return_str;
    match single_digit_iteration {
        1 => { return_str = "";                    }
        10 => { return_str = "tenths";              }
        100 => { return_str = "hundredths";           }
        1000 => { return_str = "thousandths";          }
        10000 => { return_str = "ten thousandths";      }
        100000 => { return_str = "hundred thousandths";  }
        1000000 => { return_str = "millionths"}
        _three_digit_iteration_not_found => { return_str = "very high decimal place...ths"; }
    }
    return Some(return_str.to_string());
}