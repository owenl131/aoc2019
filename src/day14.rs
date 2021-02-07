use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn split_space(x: &str) -> (i128, &str) {
    if let [qty, ingr] = x.split(" ").collect::<Vec<&str>>()[..] {
        (qty.parse().unwrap(), ingr)
    } else {
        panic!("Invalid input")
    }
}

fn toposort(ingr: &str,
            recipies: &HashMap<&str, HashSet<(&str, i128, i128)>>,
            result: &mut Vec<String>) {
    if result.contains(&ingr.to_string()) {
        return;
    }
    // all requirements for ingr should appear before ingr
    if let Some(set) = recipies.get(&ingr) {
        for (sub_ingr, _, _) in set {
            if !result.contains(&sub_ingr.to_string()) {
                toposort(sub_ingr, recipies, result);
            }
        }
    }
    result.push(ingr.to_owned());
}

fn can_support(fuel: i128, 
                requirements: &HashMap<&str, HashSet<(&str, i128, i128)>>,
                topo: &Vec<String>) -> bool {
    let mut amounts: HashMap<String, i128> = HashMap::new();
    for ingr in topo {
        amounts.insert(ingr.to_string(), 0);
    }
    amounts.insert("FUEL".to_string(), fuel);

    for ingr in topo {
        let num = *amounts.get(ingr).unwrap() as f32;
        // println!("{} {}", ingr, num);
        if let Some(set) = requirements.get(ingr.as_str()) {
            for (sub_ingr, qty, prod_qty) in set {
                if let Some(x) = amounts.get_mut(&sub_ingr.to_string()) {
                    *x += (num / *prod_qty as f32).ceil() as i128 * *qty; 
                }
            }
            amounts.remove(ingr);
        }
    }

    *amounts.get(&"ORE".to_string()).unwrap() <= 1000000000000
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let recipies: Vec<&str> = contents.trim().split('\n').collect();
    let mut requirements: HashMap<&str, HashSet<(&str, i128, i128)>> = HashMap::new();

    for recipe in recipies {
        if let [ingredients, product] = recipe.split(" => ").collect::<Vec<&str>>()[..] {
            let (prod_qty, prod_name) = split_space(product);
            let mut set = HashSet::new();
            for ingr in ingredients.split(", ") {
                let (qty, ingr_name) = split_space(ingr);
                set.insert((ingr_name, qty, prod_qty));
            }
            requirements.insert(prod_name, set);
        }
    }
    println!("{:?}", requirements);

    // topological sort
    let mut topo: Vec<String> = Vec::new();
    toposort("FUEL", &requirements, &mut topo);
    for ingr in requirements.keys() {
        toposort(ingr, &requirements, &mut topo);
    }
    topo.reverse();
    println!("{:?}", topo);

    let mut lower_bound: i128 = 0;
    let mut upper_bound: i128 = 1000000000000000;

    while lower_bound < upper_bound {
        let mid = (lower_bound + upper_bound + 1) / 2;
        if can_support(mid, &requirements, &topo) {
            lower_bound = mid;
        } else {
            upper_bound = mid - 1;
        }
        println!("{} {}", lower_bound, upper_bound);
    }

}
