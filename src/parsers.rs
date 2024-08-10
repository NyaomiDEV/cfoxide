pub(crate) fn alphabet_to_number(char: char) -> u8 {
    if ! char.is_alphabetic() { return 0; }

    let integer_value = char.to_ascii_lowercase() as u8;
    integer_value - 0x61
}

pub(crate) fn alphabet_to_month(char: char) -> u8 {
    match char.to_ascii_uppercase() {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'H' => 6,
        'L' => 7,
        'M' => 8,
        'P' => 9,
        'R' => 10,
        'S' => 11,
        'T' => 12,
        _ => 0
    }
}

pub(crate) fn month_to_alphabet(num: u8) -> char {
    match num {
        1 => 'A',
        2 => 'B',
        3 => 'C',
        4 => 'D',
        5 => 'E',
        6 => 'H',
        7 => 'L',
        8 => 'M',
        9 => 'P',
        10 => 'R',
        11 => 'S',
        12 => 'T',
        _ => '\0'
    }
}

pub(crate) fn number_to_alphabet(num: u8) -> char {
    if num > 25 { return 'a'; }
    
    (num + 0x41) as char
}