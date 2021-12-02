use std::fs;

pub fn import_file<T, F>(file_name: &str, map_file: F) -> Vec<T>
where F : Fn(&str) -> T
    {
    return fs::read_to_string(file_name)
    .expect("Something went wrong reading the file")
    .lines()
    .map(map_file)
    .collect();
}

pub fn import_file_int(file_name: &str) -> Vec<i32> {
    return import_file(file_name,  |line| { return String::from(line).parse::<i32>().unwrap() });
}