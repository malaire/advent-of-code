use regex::Regex;
use std::collections::{HashMap, HashSet};

static INPUT_A: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

static INPUT_X: &str = include_str!("input");

fn main() {
    assert_eq!(solve(INPUT_A), 5);
    assert_eq!(solve(INPUT_X), 1977);

    println!("{:?}", solve(INPUT_X));
}

fn solve(input: &str) -> usize {
    let re_line = Regex::new(r"(?m)^([^(]+) \(contains (.+)\)$").unwrap();

    // appearance counts of each ingredient
    let mut all_ingredients: HashMap<String, usize> = HashMap::new();

    // possible ingredients of each allergen
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();

    for cap in re_line.captures_iter(input) {
        let mut ingredients: HashSet<String> = HashSet::new();
        for ingredient in cap[1].split(" ") {
            ingredients.insert(ingredient.to_owned());
            *all_ingredients.entry(ingredient.to_owned()).or_insert(0) += 1;
        }

        for allergen in cap[2].split(", ") {
            let prev = allergens.get(allergen).cloned();
            match prev {
                None => {
                    allergens.insert(allergen.to_owned(), ingredients.clone());
                }
                Some(prev) => {
                    allergens.insert(
                        allergen.to_owned(),
                        prev.intersection(&ingredients).cloned().collect(),
                    );
                }
            }
        }
    }

    let mut unsolved_allergens: HashMap<String, HashSet<String>> = allergens;
    let mut solved_allergens: HashMap<String, String> = HashMap::new();
    let mut solved_ingredients: HashSet<String> = HashSet::new();

    while unsolved_allergens.len() != 0 {
        unsolved_allergens.retain(|allergen, ingredients| {
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap().to_owned();
                solved_allergens.insert(allergen.to_owned(), ingredient.clone());
                solved_ingredients.insert(ingredient);
                false
            } else {
                true
            }
        });

        for (_, ingredients) in &mut unsolved_allergens.iter_mut() {
            *ingredients = (*ingredients)
                .difference(&solved_ingredients)
                .cloned()
                .collect();
        }
    }

    let mut sum = 0;
    for (ingredient, appearances) in all_ingredients.iter() {
        if !solved_ingredients.contains(ingredient) {
            sum += appearances;
        }
    }
    sum
}
