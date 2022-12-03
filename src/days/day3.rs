use io;
use std::collections::HashSet;

pub fn day3(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day3, which is a valid file path.");
    }

    let total_cost = sum_backpack_rearrangement_costs(&args[0]);

    println!("The total cost to rearrange is {total_cost}");

    let badge_total = sum_group_badge_costs(&args[0]);

    println!("The sum of badge values is {badge_total}");
}

fn sum_group_badge_costs(filename: &str) -> u32 {
    let mut costs: u32 = 0;
    io::foreach_linegroup(filename, 3, |group| {
        // transform all rucksacks of the group into sets
        let mut group_sets: Vec<HashSet<char>> = Vec::new();
        for s in group {
            group_sets.push(to_set(s));
        }

        // fold all sets into one result set
        let all_inter_set = group_sets.iter().fold(group_sets[0].clone(), |lhs, rhs| {
            lhs.intersection(&rhs).cloned().collect()
        });

        if let Some(common_char) = all_inter_set.iter().nth(0) {
            costs += to_priority(common_char.clone())
                .unwrap_or_else(|| panic!("Not a valid character: {}", common_char));
        } else {
            panic!("Group does not have any overlapping items: {:?}", group);
        }
    });
    costs
}

fn sum_backpack_rearrangement_costs(filename: &str) -> u32 {
    let mut sum: u32 = 0;
    io::foreach_line(filename, |line| {
        let mid = line.len() / 2;
        let (first_comp, second_comp) = line.split_at(mid);
        // find common character
        let common_char = find_first_common(first_comp, second_comp).unwrap_or_else(|| {
            panic!(
                "Compartments did not have overlapping items! {} and {}",
                first_comp, second_comp
            )
        });

        // convert character to priority and add to sum
        let p = to_priority(common_char)
            .unwrap_or_else(|| panic!("Not a valid character: {}", common_char));
        sum += p;
    });
    sum
}

fn to_priority(c: char) -> Option<u32> {
    let upper_offset = c.is_uppercase() as u32 * 26;
    if c.is_digit(10) {
        None
    } else {
        match c.to_digit(36) {
            Some(d) => Some(d - 9 + upper_offset),
            None => None,
        }
    }
}

fn to_set(iterable: &str) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for v in iterable.chars() {
        set.insert(v);
    }
    set
}

fn find_first_common(lhs: &str, rhs: &str) -> Option<char> {
    let lhs_set = to_set(lhs);
    let rhs_set = to_set(rhs);

    match lhs_set.intersection(&rhs_set).nth(0) {
        Some(c) => Some(c.clone()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_priority() {
        assert_eq!(Some(1u32), to_priority('a'));
        assert_eq!(Some(2u32), to_priority('b'));
        assert_eq!(Some(3u32), to_priority('c'));
        assert_eq!(Some(4u32), to_priority('d'));
        assert_eq!(Some(5u32), to_priority('e'));
        assert_eq!(Some(24u32), to_priority('x'));
        assert_eq!(Some(25u32), to_priority('y'));
        assert_eq!(Some(26u32), to_priority('z'));

        assert_eq!(Some(26 + 1u32), to_priority('A'));
        assert_eq!(Some(26 + 2u32), to_priority('B'));
        assert_eq!(Some(26 + 3u32), to_priority('C'));
        assert_eq!(Some(26 + 4u32), to_priority('D'));
        assert_eq!(Some(26 + 5u32), to_priority('E'));
        assert_eq!(Some(26 + 24u32), to_priority('X'));
        assert_eq!(Some(26 + 25u32), to_priority('Y'));
        assert_eq!(Some(26 + 26u32), to_priority('Z'));

        assert_eq!(None, to_priority('0'));
        assert_eq!(None, to_priority('1'));
        assert_eq!(None, to_priority('2'));
        assert_eq!(None, to_priority('8'));
        assert_eq!(None, to_priority('9'));
        assert_eq!(None, to_priority('Ä'));
        assert_eq!(None, to_priority('ö'));
    }

    #[test]
    fn test_find_first_common() {
        assert_eq!(Some('E'), find_first_common("ABCDE", "EFGHI"));
        assert_eq!(Some('C'), find_first_common("abCdef", "ghiCjkkC"));
        assert_eq!(Some('p'), find_first_common("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
        assert_eq!(
            Some('L'),
            find_first_common("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
        );
        assert_eq!(Some('P'), find_first_common("PmmdzqPrV", "vPwwTWBwg"));

        assert_eq!(None, find_first_common("ABC", "DEF"));
    }
}
