use std::fs;
use std::str::FromStr;
use std::fmt::Debug;
use std;

pub fn import_file<T>(file_name: &str) -> Vec<T>
where T:FromStr,
<T as FromStr>::Err:Debug
    {
    return fs::read_to_string(file_name)
    .expect("Something went wrong reading the file")
    .lines()
    .map(|line| { String::from(line).parse::<T>().unwrap() })
    .collect();
}