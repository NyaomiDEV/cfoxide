mod omocodia;
mod parsers;

use omocodia::{omocodia, rev_omocodia, omocodia_bitmask};
use parsers::{alphabet_to_month, alphabet_to_number, month_to_alphabet, number_to_alphabet};

use std::{fmt::Display, str::FromStr};

pub struct CF([u8; 10]);

// Structure of a CF - 74 bits
// x = month (4)
// y = gender marker (1) 0 = girl 1 = boy
// PADDING GOES HERE | omo(7)  | namesurname(30)                | year(7) | x    |day(5) | y | ml(5) | mlnum(10)  | chk(5)|
//              ...0 | 0000000 | 000001111100000111110000011111 | 1110000 | 0000 | 11111 | 0 | 00000 | 1111111111 | 00000 |

#[derive(Debug)]
pub struct CFParseStringError;

impl CF {
    pub fn new(mut unpacked: u128) -> Self {
        let mut cf: [u8; 10] = [0; 10];
        for i in &mut cf {
            *i = unpacked as u8 & u8::MAX;
            unpacked >>= 8;
        }

        CF(cf)
    }

    pub fn unpack(&self) -> u128 {
        let mut unpacked: u128 = 0;
        for i in self.0.iter().rev() {
            unpacked <<= 8;
            unpacked |= *i as u128;
        }

        unpacked
    }

    pub fn get_name_surname_block(&self) -> String {
        let mut unpacked: u32 = (((self.0[7] as u32) << 24) | ((self.0[6] as u32) << 16) | ((self.0[5] as u32) << 8) | self.0[4] as u32) >> 5;
        let mut str = String::new();

        for _ in 0..=5 {
            let num = (unpacked & 0x1F) as u8;
            str = number_to_alphabet(num).to_string() + &str;
            unpacked >>= 5;
        }

        str
    }

    pub fn get_year(&self) -> u8 {
        (((((self.0[4] as u16) << 8) | self.0[3] as u16) >> 6) & 0x7F) as u8
    }

    pub fn get_month(&self) -> u8 {
        (self.0[3] >> 2) & 0x1F
    }

    pub fn get_day(&self) -> u8 {
        let day: u8 = (((((self.0[3] as u16) << 8) | self.0[2] as u16) >> 5) & 0x1F) as u8;
        if day > 39 { day - 40 } else { day }
    }

    pub fn get_gender(&self) -> bool {
        (self.0[2] >> 4) & 1 == 1
    }

    pub fn get_municipality(&self) -> String {
        let unpacked: u32 = (((self.0[2] as u32) << 16) | ((self.0[1] as u32) << 8) | self.0[0] as u32) >> 5;
        number_to_alphabet(((unpacked >> 10) as u8) & 0x1F).to_string() + &format!("{:0>3}", unpacked & 0x3FF)
    }

    pub fn get_check_letter(&self) -> String {
        number_to_alphabet(self.0[0] & 0x1F).to_string()
    }

    pub fn empty() -> Self {
        CF([0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    }
}

impl FromStr for CF {
    type Err = CFParseStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 16 || !s.chars().map(|x| x.is_alphanumeric()).fold(true, |b, n| if !n { n } else { b }) {
            return Err(CFParseStringError)
        }

        let mut c: Vec<char> = s.chars().collect();
        c.reverse();
        let mut unpacked: u128;

        // Omocodia bitmask
        let omocodia_bitmask = omocodia_bitmask(s);
        unpacked = omocodia_bitmask as u128;

        // Name and surname
        for _ in 0..=5 {
            unpacked <<= 5;
            unpacked |= c.pop().map(alphabet_to_number).unwrap_or_default() as u128;
        }
        
        // Year
        unpacked <<= 7;
        {
            let (oa, ob) = ((omocodia_bitmask >> 6) & 1, (omocodia_bitmask >> 5) & 1);
            unpacked |= u128::from_str(
                [
                    if oa == 1 { {
                        c.pop().map(rev_omocodia)
                    } } else { c.pop() },
                    if ob == 1 { {
                        c.pop().map(rev_omocodia)
                    } } else { c.pop() }
                ]
                .iter()
                .map(|x| x.unwrap_or_default())
                .collect::<String>().as_str())
                .unwrap_or_default();
        }

        // Month
        unpacked <<= 4;
        unpacked |= c.pop().map(alphabet_to_month).unwrap_or_default() as u128;

        // Day
        let (day, gender) = {
            let (oa, ob) = ((omocodia_bitmask >> 4) & 1, (omocodia_bitmask >> 3) & 1);
            let day_and_gender = u128::from_str(
            [
                if oa == 1 { {
                    let char = c.pop();
                    char.map(rev_omocodia)
                } } else { c.pop() },
                if ob == 1 { {
                    let char = c.pop();
                    char.map(rev_omocodia)
                } } else { c.pop() }
            ]
            .iter()
            .map(|x| x.unwrap_or_default())
            .collect::<String>().as_str())
            .unwrap_or_default();
            if day_and_gender > 39 {
                (day_and_gender - 40, false)
            } else {
                (day_and_gender, true)
            }
        };
        unpacked <<= 5;
        unpacked |= day;

        // Gender marker
        unpacked <<= 1;
        unpacked |= if gender {1} else {0};
        
        // Municipality letter
        unpacked <<= 5;
        unpacked |= c.pop().map(alphabet_to_number).unwrap_or_default() as u128;

        // Municipality Numbers
        unpacked <<= 10;
        {
            let (oa, ob, oc) = ((omocodia_bitmask >> 2) & 1, (omocodia_bitmask >> 1) & 1, omocodia_bitmask & 1);
            unpacked |= u128::from_str(
                [
                    if oa == 1 { {
                        c.pop().map(rev_omocodia)
                    } } else { c.pop() },
                    if ob == 1 { {
                        c.pop().map(rev_omocodia)
                    } } else { c.pop() },
                    if oc == 1 { {
                        c.pop().map(rev_omocodia)
                    } } else { c.pop() }
                ]
                .iter()
                .map(|x| x.unwrap_or_default())
                .collect::<String>().as_str())
                .unwrap_or_default();
        }

        // Checkletter
        unpacked <<= 5;
        unpacked |= c.pop().map(alphabet_to_number).unwrap_or_default() as u128;

        Ok(CF::new(unpacked))
    }
}

impl Display for CF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut unpacked = self.unpack();
        let mut str;

        // get the alphabet bitmask right here
        let mut bitmask = (unpacked >> 67) as u16;

        // Checkletter
        {
            let num = (unpacked & 0x1F) as u8;
            str = number_to_alphabet(num).to_string();
            unpacked >>= 5;
        }

        // Municipality numbers
        {
            let mut _str = format!("{:0>3}", unpacked & 0x3FF);
            while let Some(char) = _str.pop() {
                if bitmask & 1 == 1 {
                    str = omocodia(char).to_string() + &str;
                } else {
                    str = char.to_string() + &str;
                }

                bitmask >>= 1;
            }

            unpacked >>= 10;
        }

        // Municipality letter
        {
            let num = (unpacked & 0x1F) as u8;
            str = number_to_alphabet(num).to_string() + &str;
            unpacked >>= 5;
        }

        // Gender marker
        let gender = unpacked & 1 == 1;
        unpacked >>= 1;

        // Day
        {
            let day_and_gender = if gender { 0 } else { 40 } + (unpacked & 0x1F);
            let mut _str = format!("{:0>2}", day_and_gender);
            while let Some(char) = _str.pop() {
                if bitmask & 1 == 1 {
                    str = omocodia(char).to_string() + &str;
                } else {
                    str = char.to_string() + &str;
                }

                bitmask >>= 1;
            }

            unpacked >>= 5;
        }

        // Month
        {
            let num = unpacked & 0xF;
            str = month_to_alphabet(num as u8).to_string() + &str;
            unpacked >>= 4;
        }

        // Year
        {
            let year = unpacked & 0x7F;
            let mut _str = format!("{:0>2}", year);
            while let Some(char) = _str.pop() {
                if bitmask & 1 == 1 {
                    str = omocodia(char).to_string() + &str;
                } else {
                    str = char.to_string() + &str;
                }

                bitmask >>= 1;
            }

            unpacked >>= 7;
        }

        // Name and surname
        for _ in 0..=5 {
            let num = (unpacked & 0x1F) as u8;
            str = number_to_alphabet(num).to_string() + &str;
            unpacked >>= 5;
        }

        write!(f, "{}", str)
    }
}
