use regex::Regex;
use std::{collections::HashMap, str::FromStr};
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumString)]
enum PassportCode {
    #[strum(serialize = "byr")]
    BirthYear,
    #[strum(serialize = "iyr")]
    IssueYear,
    #[strum(serialize = "eyr")]
    ExpirationYear,
    #[strum(serialize = "hgt")]
    Height,
    #[strum(serialize = "hcl")]
    HairColor,
    #[strum(serialize = "ecl")]
    EyeColor,
    #[strum(serialize = "pid")]
    PassportID,
    #[strum(serialize = "cid")]
    CountryID,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumString)]
enum HeightUnit {
    #[strum(serialize = "in")]
    Inches,
    #[strum(serialize = "cm")]
    Centimeters,
}

struct Passport(HashMap<PassportCode, String>);

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.split_whitespace().map(|entry| entry.split(':')).fold(
            HashMap::new(),
            |mut acc, item| {
                {
                    let mut iter = item.into_iter();
                    acc.insert(
                        iter.next().unwrap().parse().unwrap(),
                        iter.next().unwrap().into(),
                    );
                }
                acc
            },
        );

        Ok(Passport(map))
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|line| -> Passport { line.parse().unwrap() })
        .collect()
}

trait PassportFieldValidator {
    fn field(&self) -> PassportCode;
    fn validate(&self, value: Option<&String>) -> bool;
}

struct NumberRangePassportFieldValidator {
    min: usize,
    max: usize,
    field: PassportCode,
}

struct ExistanceFieldValidator {
    field: PassportCode,
}

impl PassportFieldValidator for ExistanceFieldValidator {
    fn field(&self) -> PassportCode {
        self.field
    }

    fn validate(&self, value: Option<&String>) -> bool {
        match value {
            Some(_) => true,
            _ => false,
        }
    }
}

struct HairColorValidator {}

impl PassportFieldValidator for HairColorValidator {
    fn field(&self) -> PassportCode {
        PassportCode::HairColor
    }

    fn validate(&self, value: Option<&String>) -> bool {
        match value {
            Some(s) => {
                s.starts_with('#')
                    && s.len() == 7
                    && s.chars().skip(1).all(|c| c.is_ascii_hexdigit())
            }

            _ => false,
        }
    }
}

struct PassportIdValidator {}

impl PassportFieldValidator for PassportIdValidator {
    fn field(&self) -> PassportCode {
        PassportCode::PassportID
    }

    fn validate(&self, value: Option<&String>) -> bool {
        match value {
            Some(s) => s.len() == 9 && s.chars().all(|c| c.is_ascii_digit()),
            _ => false,
        }
    }
}

struct EyeColorValidator {}

impl PassportFieldValidator for EyeColorValidator {
    fn field(&self) -> PassportCode {
        PassportCode::EyeColor
    }

    fn validate(&self, value: Option<&String>) -> bool {
        match value {
            Some(s) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s.as_str()),
            _ => false,
        }
    }
}

struct HeightValidator {}

lazy_static! {
    static ref REG: Regex = Regex::new(r"^(?P<size>\d+)(?P<units>cm|in)$").unwrap();
}

impl PassportFieldValidator for HeightValidator {
    fn field(&self) -> PassportCode {
        PassportCode::Height
    }

    fn validate(&self, value: Option<&String>) -> bool {
        match value {
            Some(v) => {
                let captures = REG.captures(v);
                match captures {
                    Some(caps) => {
                        let size = caps
                            .name("size")
                            .map(|a| a.as_str().parse::<usize>().ok())
                            .flatten();
                        let units = caps
                            .name("units")
                            .map(|unit| unit.as_str().parse::<HeightUnit>().ok())
                            .flatten();
                        match (size, units) {
                            (Some(150..=193), Some(HeightUnit::Centimeters)) => true,
                            (Some(59..=76), Some(HeightUnit::Inches)) => true,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
            None => false,
        }
    }
}

impl PassportFieldValidator for NumberRangePassportFieldValidator {
    fn field(&self) -> PassportCode {
        self.field
    }

    fn validate(&self, value: Option<&String>) -> bool {
        match value {
            Some(v) => {
                let parsed = v.parse::<usize>();
                match parsed.map(|p| p >= self.min && p <= self.max) {
                    Ok(true) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

#[aoc(day4, part1)]
fn solve_part_one(input: &[Passport]) -> usize {
    let required_properties: Vec<Box<dyn PassportFieldValidator>> = vec![
        Box::new(ExistanceFieldValidator {
            field: PassportCode::BirthYear,
        }),
        Box::new(ExistanceFieldValidator {
            field: PassportCode::Height,
        }),
        Box::new(ExistanceFieldValidator {
            field: PassportCode::EyeColor,
        }),
        Box::new(ExistanceFieldValidator {
            field: PassportCode::ExpirationYear,
        }),
        Box::new(ExistanceFieldValidator {
            field: PassportCode::PassportID,
        }),
        Box::new(ExistanceFieldValidator {
            field: PassportCode::IssueYear,
        }),
        Box::new(ExistanceFieldValidator {
            field: PassportCode::HairColor,
        }),
    ];

    count_valid_passports(input, &required_properties)
}

fn count_valid_passports(
    input: &[Passport],
    field_descriptors: &[Box<dyn PassportFieldValidator>],
) -> usize {
    input
        .into_iter()
        .filter(|&item| {
            field_descriptors.iter().all(|descriptor| {
                let val = item.0.get(&descriptor.field());
                descriptor.validate(val)
            })
        })
        .count()
}

#[aoc(day4, part2)]
fn solve_part_two(input: &[Passport]) -> usize {
    let required_properties: Vec<Box<dyn PassportFieldValidator>> = vec![
        Box::new(NumberRangePassportFieldValidator {
            field: PassportCode::BirthYear,
            max: 2002,
            min: 1920,
        }),
        Box::new(NumberRangePassportFieldValidator {
            field: PassportCode::ExpirationYear,
            max: 2030,
            min: 2020,
        }),
        Box::new(NumberRangePassportFieldValidator {
            field: PassportCode::IssueYear,
            max: 2020,
            min: 2010,
        }),
        Box::new(HeightValidator {}),
        Box::new(HairColorValidator {}),
        Box::new(EyeColorValidator {}),
        Box::new(PassportIdValidator {}),
    ];

    count_valid_passports(input, &required_properties)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let line = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

        let passports = input_generator(line);
        assert_eq!(passports.iter().count(), 4);
        assert_eq!(solve_part_one(&passports), 2);
    }
}
