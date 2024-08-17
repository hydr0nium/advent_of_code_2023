#[derive(Debug)]
pub struct Engine {
    lines: Vec<String>,
    line_length: i32,
    length: i32,
}

impl TryFrom<Vec<String>> for Engine {
    type Error = String;

    fn try_from(input: Vec<String>) -> Result<Self, Self::Error> {
        let input: Vec<String> = input
            .into_iter()
            .map(|line| line.trim().to_string())
            .collect();
        let linelen = input.first().expect("No lines found").len() as i32;
        for line in &input {
            if line.len() as i32 != linelen {
                return Err("Lines don't have same length!".to_string());
            }
        }
        Ok(Engine {
            line_length: linelen,
            length: input.len() as i32,
            lines: input,
        })
    }
}

impl Engine {
    pub fn get(self: &Engine, row: i32, column: i32) -> Result<char, &str> {
        if row >= self.length || row < 0 {
            return Err("Row Overflow");
        }
        if column >= self.line_length || column < 0 {
            return Err("Column Overflow");
        }
        let s = *self
            .lines
            .get(row as usize)
            .expect("This should not happen as rows is validated!")
            .chars()
            .collect::<Vec<char>>()
            .get(column as usize)
            .expect("This should not happen as column is validated!");
        Ok(s)
    }
    fn get_number_right(self: &Engine, row: i32, column: i32) -> u32 {
        let first_number = match self.get(row, column) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            _ => return 0,
        };
        let second_number = match self.get(row, column + 1) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            Ok('.') => return first_number.parse::<u32>().expect("Checked before (1)"),
            _ => "".to_string(),
        };
        let third_number = match self.get(row, column + 2) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            Ok('.') => {
                return (first_number + &second_number)
                    .parse::<u32>()
                    .expect("Checked before (2)")
            }
            _ => "".to_string(),
        };
        let number = first_number + &second_number + &third_number;
        if number.is_empty() {
            return 0;
        }
        number
            .parse::<u32>()
            .expect("Could not parse string to number (to the right)")
    }

    fn get_number_left(self: &Engine, row: i32, column: i32) -> u32 {
        let first_number = match self.get(row, column) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            _ => return 0,
        };
        let second_number = match self.get(row, column - 1) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            Ok('.') => return first_number.parse::<u32>().expect("Checked before (3)"),
            _ => "".to_string(),
        };
        let third_number = match self.get(row, column - 2) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            Ok('.') => {
                return (second_number + &first_number)
                    .parse::<u32>()
                    .expect("Checked before (4)")
            }
            _ => "".to_string(),
        };
        let number = third_number + &second_number + &first_number;
        if number.is_empty() {
            return 0;
        }
        number
            .parse::<u32>()
            .expect("Could not parse string to number (to the left)")
    }

    fn get_number_middle(self: &Engine, row: i32, column: i32) -> u32 {
        if self.get(row, column + 1) == Ok('.') {
            return self.get_number_left(row, column);
        }

        if self.get(row, column - 1) == Ok('.') {
            return self.get_number_right(row, column);
        }

        let first_number = match self.get(row, column - 1) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            _ => "".to_string(),
        };
        let second_number = match self.get(row, column) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            _ => "".to_string(),
        };
        let third_number = match self.get(row, column + 1) {
            Ok(s) if s.is_ascii_digit() => s.to_string(),
            _ => "".to_string(),
        };
        let number = first_number + &second_number + &third_number;
        if number.is_empty() {
            return 0;
        }
        number
            .parse::<u32>()
            .expect("Could not parse string to number (to the middle)")
    }

    fn get_number_top(self: &Engine, row: i32, column: i32) -> u32 {
        match self.get(row - 1, column) {
            Ok(c) if !c.is_ascii_digit() => {
                dbg!(self.get_number_left(row - 1, column - 1))
                    + dbg!(self.get_number_right(row - 1, column + 1))
            }
            Ok(c) if c.is_ascii_digit() => dbg!(self.get_number_middle(row - 1, column)),
            _ => 0,
        }
    }

    fn get_number_bottom(self: &Engine, row: i32, column: i32) -> u32 {
        match self.get(row + 1, column) {
            Ok(c) if !c.is_ascii_digit() => {
                dbg!(self.get_number_left(row + 1, column - 1))
                    + dbg!(self.get_number_right(row + 1, column + 1))
            }
            Ok(c) if c.is_ascii_digit() => dbg!(self.get_number_middle(row + 1, column)),
            _ => 0,
        }
    }

    fn get_part_number(self: &Engine, row: i32, column: i32) -> u32 {
        self.get_number_top(row, column)
            + dbg!(self.get_number_left(row, column - 1))
            + dbg!(self.get_number_right(row, column + 1))
            + self.get_number_bottom(row, column)
    }

    pub fn sum_part_numbers(self: &Engine) -> u32 {
        let mut sum = 0;

        for row in 0..self.length {
            for column in 0..self.line_length {
                let current = self
                    .get(row, column)
                    .expect("Overflow in iteration of sum_part_function");
                if current != '.' && !current.is_ascii_digit() {
                    let val = self.get_part_number(row, column);
                    sum += val;
                    println!("Part ({current}) sum: {val} at {row} {column}");
                }
            }
        }
        sum
    }
}
