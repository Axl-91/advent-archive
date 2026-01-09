use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(
    input_reader: BufReader<File>,
    input_map: &mut HashMap<u32, Vec<u32>>,
) -> Vec<Vec<u32>> {
    let mut lines = input_reader.lines();
    let mut input_vector = Vec::<Vec<u32>>::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let line_splitted = line
            .split('|')
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let key = line_splitted[0];
        let value = line_splitted[1];

        input_map.entry(key).or_default().push(value);
    }

    while let Some(Ok(line)) = lines.next() {
        let line_splitted = line
            .split(',')
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        input_vector.push(line_splitted);
    }

    input_vector
}

fn get_valid_vectors(
    input_map: &HashMap<u32, Vec<u32>>,
    input_vector: &Vec<Vec<u32>>,
) -> Vec<Vec<u32>> {
    let mut valid_vectors: Vec<Vec<u32>> = Vec::new();

    for vec in input_vector {
        let mut valid_vec = true;

        for i in 0..vec.len() {
            let key = vec[i];
            for next_val in vec.iter().skip(i + 1) {
                let key_values = input_map.get(&key);

                if let Some(values) = key_values {
                    valid_vec &= values.contains(next_val)
                } else {
                    valid_vec = false;
                }
            }
            if !valid_vec {
                break;
            }
        }
        if valid_vec {
            valid_vectors.push(vec.clone());
        }
    }
    valid_vectors
}

fn part1(input_map: &HashMap<u32, Vec<u32>>, input_vector: &Vec<Vec<u32>>) {
    let mut result = 0;
    let valid_vectors = get_valid_vectors(input_map, input_vector);

    for vec in valid_vectors {
        result += vec[vec.len() / 2];
    }
    println!("RESULT: {}", result);
}

fn part2(input_map: &HashMap<u32, Vec<u32>>, input_vector: &Vec<Vec<u32>>) {
    let mut result = 0;
    let valid_vectors = get_valid_vectors(input_map, input_vector);
    let invalid_vectors: Vec<Vec<u32>> = input_vector
        .iter()
        .filter(|vec| !valid_vectors.contains(vec))
        .cloned()
        .collect();

    for vec in invalid_vectors {
        let mut middle: i8 = (vec.len() / 2) as i8;
        let mut len_expected = vec.len() - 1;
        let mut aux_vec = vec.clone();
        let mut middle_value = 0;

        while middle >= 0 {
            let possible_values: Vec<u32> = aux_vec
                .iter()
                .copied()
                .filter(|val| input_map.get(val).map_or(0, |v| v.len()) >= len_expected)
                .collect();

            for value in possible_values.clone() {
                let possible_vector: Vec<u32> = aux_vec
                    .clone()
                    .into_iter()
                    .filter(|&x| x != value)
                    .collect();

                if possible_vector
                    .iter()
                    .all(|val| input_map.get(&value).is_some_and(|v| v.contains(val)))
                {
                    aux_vec.retain(|&v| v != value);
                    middle_value = value;
                    break;
                }
            }

            middle -= 1;
            len_expected -= 1;
        }
        result += middle_value;
    }
    println!("RESULT: {}", result);
}

fn main() {
    let mut input_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let file = File::open("input").unwrap();
    let input_reader = BufReader::new(file);

    let input_vector = parse_input(input_reader, &mut input_map);

    part1(&input_map, &input_vector);
    part2(&input_map, &input_vector);
}
