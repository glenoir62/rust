fn find_element(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &element) in arr.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

pub fn option_test() {
    let numbers = vec![1, 2, 3, 4, 5];
    let target_value = 3;

    match find_element(&numbers, target_value) {
        Some(index) => println!("option_test Valeur trouvée à l'indice : {}", index),
        None => println!("option_test Valeur non trouvée"),
    }
}