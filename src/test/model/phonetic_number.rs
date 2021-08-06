pub mod from{
        
    use crate::model::phonetic_number::PhoneticNumber;


    #[test]
    pub fn triplicate_zeros_silenced_if_other_values_present(){
        let test_val = PhoneticNumber::from("1000".to_string());
        
        assert_eq!(test_val.get_numeric_string(), "1000");
        assert_eq!(test_val.get_phonetic_string(), "one thousand");
    }

    #[test]
    pub fn triplicate_zeros_are_zero_if_no_other_values_present(){
        let test_val = PhoneticNumber::from("000".to_string());
        
        assert_eq!(test_val.get_numeric_string(), "0");
        assert_eq!(test_val.get_phonetic_string(), "zero");
    }
    
    #[test]
    pub fn zero_dollars(){
        let test_val = PhoneticNumber::from("0".to_string());
        
        assert_eq!(test_val.get_numeric_string(), "0");
        assert_eq!(test_val.get_phonetic_string(), "zero");
    }
    

    #[test]
    pub fn decimal_with_no_whole_number(){
        let test_val = PhoneticNumber::from(".1000".to_string());
        
        assert_eq!(test_val.get_numeric_string(), "0.1000");
        assert_eq!(test_val.get_phonetic_string(), "zero, and one thousand ten thousandths");
    }

    #[test]
    pub fn clean_invalid_characters(){
        let test_val = PhoneticNumber::from("1/0d0f1".to_string());

        assert_eq!(test_val.get_numeric_string(), "1001");
        assert_eq!(test_val.get_phonetic_string(), "one thousand one");
    }

    #[test]
    pub fn singular_decimal(){
        let test_val = PhoneticNumber::from("1.0001".to_string());

        assert_eq!(test_val.get_numeric_string(), "1.0001");
        assert_eq!(test_val.get_phonetic_string(), "one, and one ten thousandth");
    }

    #[test]
    pub fn plural_decimal(){
        let test_val = PhoneticNumber::from("1.1983".to_string());

        assert_eq!(test_val.get_numeric_string(), "1.1983");
        assert_eq!(test_val.get_phonetic_string(), "one, and one thousand nine hundred eighty three ten thousandths");
    }

    #[test]
    pub fn decimal_leading_zeros(){
        let test_val = PhoneticNumber::from("1.0093".to_string());

        assert_eq!(test_val.get_numeric_string(), "1.0093");
        assert_eq!(test_val.get_phonetic_string(), "one, and ninety three ten thousandths");
    }

    #[test]
    pub fn zero_decimal(){
        let test_val = PhoneticNumber::from("0.000".to_string());

        assert_eq!(test_val.get_numeric_string(), "0.0");
        
        //This is a matter of taste... electing to read out the single decimal value rather than just "zero" if no value in decimal
        assert_eq!(test_val.get_phonetic_string(), "zero, and zero thousandths");
    }

    #[test]
    pub fn decimal_trailing_zeros(){
        let test_val = PhoneticNumber::from("0.100".to_string());

        assert_eq!(test_val.get_numeric_string(), "0.100");
        
        //This is a matter of taste... electing to read out the full decimal value rather than just "zero, and one tenth"
        assert_eq!(test_val.get_phonetic_string(), "zero, and one hundred thousandths");
    }

    #[test]
    pub fn singular_dollar(){
        let test_val = PhoneticNumber::from("$1".to_string());

        assert_eq!(test_val.get_numeric_string(), "$1");
        assert_eq!(test_val.get_phonetic_string(), "one dollar");
    }

    #[test]
    pub fn plural_dollars(){
        let test_val = PhoneticNumber::from("$2".to_string());

        assert_eq!(test_val.get_numeric_string(), "$2");
        assert_eq!(test_val.get_phonetic_string(), "two dollars");
    }

    #[test]
    pub fn whole_cents(){
        let test_val = PhoneticNumber::from("$0.26".to_string());

        assert_eq!(test_val.get_numeric_string(), "$0.26");
        assert_eq!(test_val.get_phonetic_string(), "zero dollars, and twenty six cents");
    }

    #[test]
    pub fn fractions_of_cents(){
        let test_val = PhoneticNumber::from("$0.26123".to_string());

        assert_eq!(test_val.get_numeric_string(), "$0.26123");
        assert_eq!(test_val.get_phonetic_string(), "zero dollars, and twenty six cents");
    }

    #[test]
    pub fn singular_cent(){
        let test_val = PhoneticNumber::from("$0.01".to_string());

        assert_eq!(test_val.get_numeric_string(), "$0.01");
        assert_eq!(test_val.get_phonetic_string(), "zero dollars, and one cent");
    }

    #[test]
    pub fn negative_non_monetary(){
        let test_val = PhoneticNumber::from("-123030.26".to_string());

        assert_eq!(test_val.get_numeric_string(), "-123030.26");
        assert_eq!(test_val.get_phonetic_string(), "negative one hundred twenty three thousand thirty, and twenty six hundredths");
    }

    #[test]
    pub fn negative_zero_should_just_be_zero(){
        let test_val = PhoneticNumber::from("-0".to_string());

        assert_eq!(test_val.get_numeric_string(), "0");
        assert_eq!(test_val.get_phonetic_string(), "zero");
    }

    #[test]
    pub fn negative_zero_with_decimal_should_be_negative(){
        let test_val = PhoneticNumber::from("-0.2".to_string());

        assert_eq!(test_val.get_numeric_string(), "-0.2");
        assert_eq!(test_val.get_phonetic_string(), "negative zero, and two tenths");
    }

    #[test]
    pub fn negative_dollars(){
        let test_val = PhoneticNumber::from("-$123030.26".to_string());

        assert_eq!(test_val.get_numeric_string(), "-$123030.26");
        assert_eq!(test_val.get_phonetic_string(), "negative one hundred twenty three thousand thirty dollars, and twenty six cents");
    }

    #[test]
    pub fn invalid_dollar_does_not_convert_to_monetary(){
        let test_val = PhoneticNumber::from("-1$23030.26".to_string());

        assert_eq!(test_val.get_numeric_string(), "-123030.26");
        assert_eq!(test_val.get_phonetic_string(), "negative one hundred twenty three thousand thirty, and twenty six hundredths");
    }

    #[test]
    pub fn invalid_negative_does_not_convert_to_negative(){
        let test_val = PhoneticNumber::from("$123-030.26".to_string());

        assert_eq!(test_val.get_numeric_string(), "$123030.26");
        assert_eq!(test_val.get_phonetic_string(), "one hundred twenty three thousand thirty dollars, and twenty six cents");
    }

    #[test]
    pub fn commas_have_no_impact_on_number_strings(){
        let test_val = PhoneticNumber::from("1,,,,,,,2.3,,,,4".to_string());

        assert_eq!(test_val.get_numeric_string(), "12.34");
        assert_eq!(test_val.get_phonetic_string(), "twelve, and thirty four hundredths");
    }

    #[test]
    pub fn additional_decimals_only_count_furthest_right(){
        let test_val = PhoneticNumber::from("1432.43.53.21".to_string());

        assert_eq!(test_val.get_numeric_string(), "14324353.21");
        assert_eq!(test_val.get_phonetic_string(), "fourteen million three hundred twenty four thousand three hundred fifty three, and twenty one hundredths");
    }
}