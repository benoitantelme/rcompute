use std::fmt;

pub struct Calculator {
    pub a: [[u32; 3]; 3],
    pub b: [[u32; 3]; 3],
}

impl Calculator {
    pub fn new(a: [[u32; 3]; 3], b: [[u32; 3]; 3]) -> Self {
        Self { a, b }
    }
}

pub fn get_print(matrix: [[u32; 3]; 3]) -> String {
    let mut printed: String = String::new();
    for row in matrix {
        for value in row {
            printed.push_str(&format!("{} ", value));
        }
        printed.push('\n');
    }
    printed
}

impl fmt::Display for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a is\n{}\n", get_print(self.a))?;
        write!(f, "b is\n{}\n", get_print(self.b))?;
        Ok(())
    }
}
