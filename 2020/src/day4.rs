use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let count = input
        .split("\n\n")
        .filter(|s| {
            let fields = s
                .lines()
                .flat_map(|line| line.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            validate_passport_1(&fields[..])
        })
        .count();

    Ok(count)
}

fn validate_passport_1(fields: &[&str]) -> bool {
    let seen = HashSet::<String>::from_iter(fields.iter().map(|s| s[..3].to_string()));
    (seen.len() == 8) || (seen.len() == 7 && !seen.contains("cid"))
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let count = input
        .split("\n\n")
        .filter(|s| {
            let fields = s
                .lines()
                .flat_map(|line| line.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            validate_passport_2(&fields[..])
        })
        .count();

    Ok(count)
}

fn validate_passport_2(fields: &[&str]) -> bool {
    let seen = HashSet::<String>::from_iter(fields.iter().map(|s| s[..3].to_string()));
    let contains_fields = (seen.len() == 8) || (seen.len() == 7 && !seen.contains("cid"));

    if !contains_fields {
        return false;
    }

    fields.iter().all(|s| match &s[..3] {
        "byr" => valid_birth_year(&s[4..]),
        "iyr" => valid_issue_year(&s[4..]),
        "eyr" => valid_expiration_year(&s[4..]),
        "hgt" => valid_height(&s[4..]),
        "hcl" => valid_hair_color(&s[4..]),
        "ecl" => valid_eye_color(&s[4..]),
        "pid" => valid_passport_id(&s[4..]),
        "cid" => true,
        _ => false,
    })
}

fn valid_birth_year(s: &str) -> bool {
    s.parse::<u32>().map_or(false, |n| n >= 1920 && n <= 2002)
}

fn valid_issue_year(s: &str) -> bool {
    s.parse::<u32>().map_or(false, |n| n >= 2010 && n <= 2020)
}

fn valid_expiration_year(s: &str) -> bool {
    s.parse::<u32>().map_or(false, |n| n >= 2020 && n <= 2030)
}

fn valid_height(s: &str) -> bool {
    if s.len() <= 2 {
        return false;
    }

    match &s[s.len() - 2..] {
        "cm" => s[..s.len() - 2]
            .parse::<u32>()
            .map_or(false, |n| n >= 150 && n <= 193),
        "in" => s[..s.len() - 2]
            .parse::<u32>()
            .map_or(false, |n| n >= 59 && n <= 76),
        _ => false,
    }
}

fn valid_hair_color(s: &str) -> bool {
    s.len() == 7
        && s.starts_with("#")
        && s.bytes()
            .skip(1)
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn valid_eye_color(s: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)
}

fn valid_passport_id(s: &str) -> bool {
    s.len() == 9 && s.bytes().all(|b| matches!(b, b'0'..=b'9'))
}
