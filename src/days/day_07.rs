use std::collections::HashMap;

use regex::Regex;

type BagMap = HashMap<String, Vec<(String, u8)>>;
lazy_static! {
    static ref REG: Regex =
        Regex::new(r"^(?P<bag_type>\w+ \w+) bags contain (?P<inner_bags>.+)\.$").unwrap();
    static ref INNER_REG: Regex =
        Regex::new(r"^(?P<bag_count>\d+) (?P<bag_name>\w+ \w+) bag").unwrap();
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> BagMap {
    input
        .lines()
        .map(|line| {
            let captures = REG.captures(line)?;
            let bag_type = captures.name("bag_type")?.as_str();
            let inner_bags = captures.name("inner_bags")?.as_str();

            let inner_bag_tuples: Option<Vec<(String, u8)>> = match inner_bags {
                "no other bags" => Some(vec![]),
                _ => inner_bags
                    .split(',')
                    .map(|i| {
                        let z = INNER_REG.captures(i.trim())?;
                        let bag_count = z.name("bag_count")?.as_str().parse::<u8>().ok()?;
                        let bag_name = z.name("bag_name")?.as_str();
                        Some((bag_name.to_string(), bag_count))
                    })
                    .collect(),
            };

            Some((bag_type.to_string(), inner_bag_tuples?))
        })
        .collect::<Option<HashMap<String, Vec<(String, u8)>>>>()
        .unwrap()
}

#[aoc(day7, part1)]
fn solve_part_one(input: &BagMap) -> usize {
    fn is_match(map: &BagMap, bag: &String) -> bool {
        if bag == "shiny gold" {
            return true;
        }

        map[bag].iter().any(|(b, _)| is_match(map, b))
    }

    input.keys().filter(|k| is_match(input, k)).count() - 1
}

#[aoc(day7, part2)]
fn solve_part_two(input: &BagMap) -> usize {
    fn total_bags(map: &BagMap, bag: &str) -> usize {
        map[bag]
            .iter()
            .map(|(b, c)| (*c as usize) * total_bags(map, b.as_str()))
            .sum::<usize>()
    }

    total_bags(input, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let line = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        let generated = input_generator(line);
        let result = solve_part_one(&generated);

        assert_ne!(result, 4);
    }
}
