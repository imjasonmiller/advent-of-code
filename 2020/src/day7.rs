use std::collections::HashMap;

type Bag = String;
type Content = (Bag, usize);
type Contents = Vec<Content>;
type Rules = HashMap<Bag, Contents>;

fn parse_rule(rule: &str) -> Option<(Bag, Contents)> {
    let (key, values) = rule.split_once("bags contain").unwrap();

    let key = key.trim();
    let values: Contents = values
        .split(",")
        .filter_map(|s| {
            if s.contains("no other bags") {
                return None;
            }

            let s = s
                .trim_end_matches(".")
                .trim_end_matches("bag")
                .trim_end_matches("bags")
                .trim();

            let (v, k) = s
                .split_once(" ")
                .map(|(v, k)| (v.parse::<usize>().unwrap_or(0), k))
                .unwrap();

            Some((k.to_owned(), v))
        })
        .collect();

    Some((key.to_owned(), values))
}

fn contains_shiny_gold(rules: &Rules, contents: &Contents) -> bool {
    if contents.iter().any(|(bag, _)| bag == "shiny gold") {
        return true;
    }

    contents.iter().any(|(bag, _)| {
        rules.get(bag).map_or(false, |more_contents| {
            contains_shiny_gold(rules, more_contents)
        })
    })
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let rules: Rules = input.lines().filter_map(parse_rule).collect();

    let count = rules
        .iter()
        .map(|(_, values)| contains_shiny_gold(&rules, values) as usize)
        .sum();

    Ok(count)
}

fn count_bags(rules: &Rules, contents: &Contents) -> usize {
    contents
        .iter()
        .map(|(bag, count)| count + count * count_bags(&rules, rules.get(bag).unwrap()))
        .sum()
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let rules: Rules = input.lines().filter_map(parse_rule).collect();

    let total = count_bags(&rules, rules.get("shiny gold").unwrap());

    Ok(total)
}
