use rand::Rng;

pub fn generate(name_length: usize, extensions: &Vec<&'static str>) -> String {
    let mut name = generate_random_string(name_length);
    if extensions.len() == 1 {
        let extension = extensions[0];
        name.push_str(&format!(".{}", &extension));
    }

    name
}

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
