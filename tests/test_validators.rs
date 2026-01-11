
    use swiss_qrust::validators::is_valid_iban;

    #[test]
    fn test_valid_iban(){
        const IBAN: &str = "CH93 0076 2011 6238 5295 7";

        let result = is_valid_iban(IBAN);

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_iban(){
        const IBAN: &str = "CH44 0871 0000 0033 1272 0007";

        let result = is_valid_iban(IBAN).unwrap();

        assert!(!result, "Expected '{}' to be invalid, but got true", IBAN);
    }
