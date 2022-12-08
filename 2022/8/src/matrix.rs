pub struct Matrix {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut data = Vec::with_capacity(width * height);

        for line in lines {
            for char in line.chars() {
                data.push(char as u8 - b'0');
            }
        }

        Self {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data
            .get(y * self.width + x)
            .copied()
            .unwrap_or_default()
    }
}
