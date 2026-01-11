pub fn is_valid_iban(iban: &str) ->  Result<bool, String>  {

    let iban = iban.chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    if iban.len() < 15 || iban.len() > 34 {
        return Err("Invalid IBAN length".into());
    }


    let rearranged = iban[4..]
        .chars()
        .chain(iban[..4].chars());

    let mut remainder: u32 = 0;

    for ch in rearranged {
        match ch.to_ascii_uppercase() {
            '0'..='9' => {
                remainder = (remainder * 10 + (ch as u32 - '0' as u32)) % 97;
            }
            'A'..='Z' => {
                let value = ch.to_ascii_uppercase() as u32 - 'A' as u32 + 10;
                remainder = (remainder * 100 + value) % 97;
            }
            _ => return Err(format!("Invalid character in IBAN: {}", ch)),
        }
    }
    Ok(remainder == 1);
}