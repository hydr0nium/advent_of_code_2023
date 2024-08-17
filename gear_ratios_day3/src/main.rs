mod engine;
mod readin;

use engine::Engine;
use readin::read_input;

fn main() {
    let input: Vec<String> = read_input("./input.txt")
        .lines()
        .map(String::from)
        .collect();
    let engine = Engine::try_from(input).expect("Could not create engine schematic");
    let res = engine.sum_part_numbers();
    println!("Final Result: {res}");
}

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input: Vec<String> = read_input("./tests/test1.txt")
            .lines()
            .map(String::from)
            .collect();
        let engine = Engine::try_from(input).expect("Could not create engine schematic");
        let res = engine.sum_part_numbers();
        assert_eq!(res, 4361);
    }
    #[test]
    fn test_2() {
        let input: Vec<String> = read_input("./tests/test2.txt")
            .lines()
            .map(String::from)
            .collect();
        let engine = Engine::try_from(input).expect("Could not create engine schematic");
        let res = engine.sum_part_numbers();
        assert_eq!(res, 413);
    }
    #[test]
    fn test_3() {
        let input: Vec<String> = read_input("./tests/test3.txt")
            .lines()
            .map(String::from)
            .collect();
        let engine = Engine::try_from(input).expect("Could not create engine schematic");
        let res = engine.sum_part_numbers();
        assert_eq!(res, 925);
    }

    #[test]
    fn test_4() {
        let input: Vec<String> = read_input("./tests/test4.txt")
            .lines()
            .map(String::from)
            .collect();
        let engine = Engine::try_from(input).expect("Could not create engine schematic");
        let res = engine.sum_part_numbers();
        assert_eq!(res, 138);
    }

    #[test]
    fn test_5() {
        let input: Vec<String> = read_input("./tests/test5.txt")
            .lines()
            .map(String::from)
            .collect();
        let engine = Engine::try_from(input).expect("Could not create engine schematic");
        let res = engine.sum_part_numbers();
        assert_eq!(res, 4);
    }
}
