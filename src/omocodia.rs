// Omocodia

pub(crate) fn omocodia(char: char) -> char {
    match char {
        '0' => 'L',
        '1' => 'M',
        '2' => 'N',
        '3' => 'P',
        '4' => 'Q',
        '5' => 'R',
        '6' => 'S',
        '7' => 'T',
        '8' => 'U',
        '9' => 'V',
        _ => '\0'
    }
}

pub(crate) fn rev_omocodia(char: char) -> char {
    match char {
        'L' => '0',
        'M' => '1',
        'N' => '2',
        'P' => '3',
        'Q' => '4',
        'R' => '5',
        'S' => '6',
        'T' => '7',
        'U' => '8',
        'V' => '9',
        _ => '\0'
    }
}

pub(crate) fn omocodia_bitmask(s: &str) -> u8 {
    let mut bitmask = 0;

    let mut chars: Vec<char> = s.chars().collect();
    let keep = [
        false,false,false,false,false,false,
        true,true,false,true,true,false,true,true,true,false
    ];

    chars = chars
        .into_iter()
        .enumerate()
        .filter(|&x| *keep.get(x.0).unwrap_or(&false))
        .map(|x| x.1)
        .collect();

    for char in chars {
        bitmask <<= 1;
        if char.is_alphabetic() {
            bitmask |= 1;
        }
    }
    bitmask
}

// 