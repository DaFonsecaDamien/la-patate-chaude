use shared::hash_cash_model::{MD5HashCashInput, MD5HashCashOutput};
use std::process::{Command};

pub struct HashCash {
    input: MD5HashCashInput,
    output: MD5HashCashOutput,
    counter: u64
}

impl HashCash {
    pub fn new(input: MD5HashCashInput)-> HashCash {
        return HashCash {
            input,
            output: MD5HashCashOutput { seed: 0, hashcode: "".parse().unwrap() },
            counter: 0
        }
    }

    pub fn run(mut self) -> MD5HashCashOutput {
        for _ in 0..u64::MAX {
            if self.verify() {
                return self.output;
            }
            self.counter += 1;
        }
        self.output
    }
    fn verify(&mut self) -> bool {
        let seed = format!("{:016X}", self.counter);
        let message = format!("{}{}", seed, self.input.message);
        let md5 = String::from_utf8(Command::new("md5")
            .arg("-qs")
            .arg(message)
            .output()
            .expect("failed to execute process")
            .stdout).unwrap();
        let hash = md5.to_string();
        if self.convert_to_binary_from_hex(&hash.to_uppercase()).starts_with(self.get_complexity_string(self.input.complexity).as_str()) {
            println!("\n{}{}", seed, self.input.message);
            self.output.seed = self.counter;
            self.output.hashcode = md5;
            return true;
        }
        return false;
    }

    fn get_complexity_string(&self, complexity: u32) -> String {
        let mut string = String::new();
        for _ in 0..complexity {
            string.push('0');
        }
        return string
    }

    fn convert_to_binary_from_hex(&self, hex: &str) -> String {
        return hex[0 ..]
            .chars()
            .map(|c| self.char_to_binary(c))
            .collect();
    }

    fn char_to_binary(&self, c: char) -> String {
        let b = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _  => "",
        };

        b.to_string()
    }
}