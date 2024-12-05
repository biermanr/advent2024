use std::path::Path;
use std::collections::{HashMap,HashSet,VecDeque};

fn parse_inputs(text: &str) -> (HashMap<&str, HashSet<&str>>, Vec<Vec<&str>>) {
    let rules_vec: Vec<(&str, &str)> = text
        .lines()
        .take_while(|l| l.len() > 0)
        .map(|l| l.split_once('|').unwrap())
        .collect();

    let mut rules: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (prior,latter) in rules_vec {
        rules.entry(prior).or_insert(HashSet::new()).insert(latter);
    }

    let updates: Vec<Vec<&str>> = text
        .lines()
        .skip_while(|l| l.len() > 0)
        .skip(1)
        .map(|l| l.split(',').collect::<Vec<&str>>())
        .collect();

    (rules, updates)
}

fn validate_updates<'a>(rules: &'a HashMap<&'a str, HashSet<&'a str>>, updates: &'a Vec<Vec<&'a str>>, keep_valid: bool) -> Vec<&'a Vec<&'a str>> {
    let mut valid_updates = vec![];

    for update in updates {
        let mut valid = true;
        for i in 0..update.len()-1 {
            for j in i+1..update.len() {
                let prior_page = &update[i];
                let latter_page = &update[j];
                if let Some(subsequent_pages) = rules.get(latter_page) {
                    if subsequent_pages.contains(prior_page) { valid = false; }
                }
            }
        }
        if valid && keep_valid { valid_updates.push(update); }
        if !valid && !keep_valid { valid_updates.push(update); }
    }

    valid_updates
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let (rules, updates) = parse_inputs(&text);
    let valid_updates = validate_updates(&rules, &updates, true);

    // sum of middle pages of valid updates
    valid_updates.iter().map(|u| u[u.len()/2].parse::<u32>().unwrap()).sum()
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let (rules, updates) = parse_inputs(&text);
    let invalid_updates = validate_updates(&rules, &updates, false);

    // order the updates 
    let mut ordered_updates: Vec<Vec<&str>> = vec![];

    for update in invalid_updates {
        let mut unordered_update:VecDeque<&str> = update.clone().into();
        let mut ordered_update: Vec<&str> = vec![];
        while unordered_update.len() > 0 {
            let mut valid = true;
            let prior_page = unordered_update.pop_front().unwrap();
            for latter_page in &unordered_update {
                if let Some(subsequent_pages) = rules.get(latter_page) {
                    if subsequent_pages.contains(prior_page) { 
                        valid = false; 
                        break;
                    }
                }
            }

            if valid {
                ordered_update.push(prior_page);
            } else {
                unordered_update.push_back(prior_page);
            }

        }

        ordered_updates.push(ordered_update);
    }

    // sum of middle pages of valid updates
    ordered_updates.iter().map(|u| u[u.len()/2].parse::<u32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_test_file() -> (tempfile::TempDir, File, PathBuf) {

        let test_input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part1() {
        let (_d, _f, test_path) = create_test_file();
        let result = part1(&test_path);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 123);
    }
}
