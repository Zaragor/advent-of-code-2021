use crate::extensions::import_file;

pub fn day2() -> i32 {
    let directions = import_file("src/input/day2.txt", |line| { 
        let mut words = line.split_whitespace();
        let direction = words.next();
        let distance = words.next().unwrap();
        match direction {
            Some("forward") => return Some((distance.parse::<i32>().unwrap(), 0)),
            Some("up") => return Some((0, -distance.parse::<i32>().unwrap())),
            Some("down") => return Some((0, distance.parse::<i32>().unwrap())),
            Some(&_) => panic!("First string not expected: {}", direction.unwrap()),
            None => return None
        };
     });

     let mut x = 0;
     let mut z = 0;

     for direction in directions {
         x = direction.unwrap().0 + x;
         z = direction.unwrap().1 + z;
     }
    return x * z;
}


pub fn day2_2() -> i32 {
    let direction_strings = import_file("src/input/day2.txt", |line| { return String::from(line) });
    let mut parsed_directions: Vec<Option<(i32, i32)>> = Vec::new();
    let mut aim = 0;
    for direction_string in direction_strings {
        let mut words = direction_string.split_whitespace();
        let direction = words.next();
        let distance = words.next().unwrap().parse::<i32>().unwrap();
        match direction {
            Some("forward") => parsed_directions.push( Some((distance, distance * aim))),
            Some("up") => aim = aim - distance,
            Some("down") => aim = aim + distance,
            Some(&_) => panic!("First string not expected: {}", direction.unwrap()),
            None => panic!("Empty direction string")
        };
    }
     let mut x = 0;
     let mut z = 0;

     for direction in parsed_directions {
         x = direction.unwrap().0 + x;
         z = direction.unwrap().1 + z;
     }
    return x * z;
}


#[cfg(test)]
mod day2_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}