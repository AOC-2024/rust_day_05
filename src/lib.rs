use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string, str::FromStr};
use regex::Regex;

pub fn sum_middle_pages(input_path: &str) -> u32 {
    let puzzle = read_puzzle(input_path);

    puzzle.find_ordered_updates()
        .iter()
        .map(|value| value[value.len() / 2])
        .fold(0, |acc, value| acc + value)
}

pub fn sum_re_ordered_middle_pages(input_path: &str) -> u32 {
    let puzzle = read_puzzle(input_path);

    puzzle.re_order_updates()
        .iter()
        .map(|value| value[value.len() / 2])
        .fold(0, |acc, value| acc + value)
}

fn read_puzzle(input_path: &str) -> Puzzle {

    let mut previous_pages: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut next_pages: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();


    read_to_string(input_path)
    .unwrap()
    .lines()
    .take_while(|line| !line.is_empty())
    .for_each(|item| {
        let regex = Regex::new("(?<first>[0-9]{2})\\|(?<second>[0-9]{2})").unwrap();
        regex.captures_iter(item).for_each(|value| {
            let first_number: u32 = FromStr::from_str(value.name("first").unwrap().as_str()).unwrap();
            let second_number: u32 = FromStr::from_str(value.name("second").unwrap().as_str()).unwrap();

            //Inserting first number infos
            let existing_left_value = previous_pages.get_mut(&second_number);
            if  existing_left_value.is_some() {
                existing_left_value.unwrap().push(first_number);                
            } else {
                previous_pages.insert(second_number, vec![first_number]);
            };

            let existing_right_value = next_pages.get_mut(&first_number);
            if  existing_right_value.is_some() {
                existing_right_value.unwrap().push(second_number);                
            } else {
                next_pages.insert(first_number, vec![second_number]);
            };
        
        });
    });

    
    read_to_string(input_path)
    .unwrap()
    .lines()
    .skip_while(|line| !line.is_empty())
    .skip(1)
    .for_each(|update| updates.push(update.split(",").map(|value| value.parse::<u32>().unwrap()).collect::<Vec<u32>>()));

    return Puzzle {
        previous_pages,
        next_pages,
        updates
    };
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Puzzle {
    next_pages: HashMap<u32, Vec<u32>>,
    previous_pages: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>
}

impl Puzzle {
    fn find_ordered_updates(&self) -> Vec<Vec<u32>> {
        self.updates
            .clone()
            .into_iter()
            .filter(|update| self.is_update_ordered(update))
            .collect::<Vec<Vec<u32>>>()
    }

    fn re_order_updates(&self) -> Vec<Vec<u32>> {
        self.updates
            .clone()
            .into_iter()
            .filter(|update| !self.is_update_ordered(update))
            .map(|update| self.re_order_update(&update))
            .collect::<Vec<Vec<u32>>>()
    }

    fn re_order_update(&self, update: &Vec<u32>) -> Vec<u32> {
        let mut graph = HashMap::new();
        let mut in_degree = HashMap::new();

        for &page in update {
            graph.entry(page).or_insert_with(Vec::new);
            in_degree.entry(page).or_insert(0);
        }

        for (&page, dependencies) in &self.previous_pages {
            for &dependency in dependencies {
                if update.contains(&page) && update.contains(&dependency) {
                    graph.entry(dependency).or_default().push(page);
                    *in_degree.entry(page).or_insert(0) += 1;
                }
            }
        }

        let mut queue: VecDeque<u32> = in_degree
            .iter()
            .filter(|&(_, &count)| count == 0)
            .map(|(&page, _)| page)
            .collect();

        let mut ordered = Vec::new();
        while let Some(page) = queue.pop_front() {
            ordered.push(page);

            if let Some(dependents) = graph.get(&page) {
                for &dependent in dependents {
                    if let Some(entry) = in_degree.get_mut(&dependent) {
                        *entry -= 1;
                        if *entry == 0 {
                            queue.push_back(dependent);
                        }
                    }
                }
            }
        }

        let remaining: HashSet<u32> = update.iter().cloned().collect();
        let ordered_set: HashSet<u32> = ordered.iter().cloned().collect();
        let orphans: Vec<u32> = remaining.difference(&ordered_set).cloned().collect();

        ordered.extend(orphans);
        ordered
    }

    fn is_update_ordered(&self, update: &Vec<u32>) -> bool {
        for index in 0..update.len() {
            let empty_vec: Vec<u32> = Vec::new();
            let current_page_number = update.get(index).unwrap();
            let next_pages_rules = self.next_pages.get(current_page_number).unwrap_or_else(|| &empty_vec);
            let previous_pages_rules = self.previous_pages.get(current_page_number).unwrap_or_else(|| &empty_vec);
            for next_pages_index in index + 1..update.len() {
                if !next_pages_rules.contains(update.get(next_pages_index).unwrap()) {
                    return false;
                }
            }

            for previous_pages_index in 0..index {
                if !previous_pages_rules.contains(update.get(previous_pages_index).unwrap()) {
                    return false;
                }
            
            } 
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_re_ordered_update() {
        let puzzle = read_puzzle("tests/resources/puzzle.txt");
        
        assert_eq!(puzzle.re_order_update(&vec![75, 97, 47, 61, 53]), vec![97, 75, 47, 61, 53]);
    }

    #[test]
    fn should_find_ordered_update_true() {
        let puzzle = read_puzzle("tests/resources/puzzle.txt");
        
        assert_eq!(puzzle.is_update_ordered(&vec![75, 47, 61, 53, 29]), true);
    }

    #[test]
    fn should_find_ordered_update_false() {
        let puzzle = read_puzzle("tests/resources/puzzle.txt");
        
        assert_eq!(puzzle.is_update_ordered(&vec![75, 97, 47, 61, 53]), false);
    }

    #[test]
    fn should_read_puzzle() {

        let mut expected_previous_pages = HashMap::new();
        expected_previous_pages.insert(53, vec![47, 48]);
        expected_previous_pages.insert(13, vec![97]);

        let mut expected_next_pages = HashMap::new();
        expected_next_pages.insert(47, vec![53]);
        expected_next_pages.insert(97, vec![13]);
        expected_next_pages.insert(48, vec![53]);

        assert_eq!(read_puzzle("tests/resources/light_puzzle.txt"), Puzzle {
            previous_pages: expected_previous_pages,
            next_pages: expected_next_pages,
            updates: vec![
                vec![75,47,61,53,29],
                vec![97,61,53,29,13]
            ]
        });
    }

}