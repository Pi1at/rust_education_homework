// new type pattern
pub mod snils {
    use std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    };

    /// SNILS format is "123-456-789 12"
    /// where the first 9 characters can be any digits and the final two are a checksum
    #[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
    pub struct Snils([u8; 11]);

    impl FromStr for Snils {
        type Err = Box<dyn std::error::Error>;
        /// check for str len < 15
        /// Filterig out '-' and ' ' and after accept only "aaabbbcccdd..." strings
        /// where a,b,c,d - ascii digits
        /// with checksum dd
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.len() < 15 {
                let as_digits = s
                    .split(&['-', ' '])
                    .flat_map(|s| s.chars())
                    .filter(|&c| c.is_ascii_digit())
                    .map(|c| c as u8 - b'0')
                    .collect::<Vec<u8>>();
                if as_digits.len() == 11 {
                    // 0..9 9..11
                    let dd = (as_digits[9] * 10 + as_digits[10]) as u32;
                    let checksum = {
                        let ms = (1..=9)
                            .map(|pos| as_digits[9 - pos] as u32 * pos as u32)
                            .sum::<u32>()
                            % 101;
                        if ms == 100 {
                            0
                        } else {
                            ms
                        }
                    };
                    if dd == checksum {
                        return Ok(Self(as_digits.try_into().unwrap()));
                    }
                }
            }
            Err("Invalid SNILS format".into())
        }
    }

    impl Display for Snils {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let mut groups = self
                .0
                .chunks(3)
                .map(|chunk| chunk.iter().map(|d| (d + b'0') as char).collect::<String>());
            write!(
                f,
                "{}-{}-{} {}",
                groups.next().unwrap(),
                groups.next().unwrap(),
                groups.next().unwrap(),
                groups.next().unwrap(),
            )
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::snils::Snils;
    #[test]
    fn parsing_works() {
        let result = "112-233-445 95".parse::<Snils>().unwrap();
        println!("{}", result);
    }
}
