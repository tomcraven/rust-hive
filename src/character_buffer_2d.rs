pub struct CharacterBuffer2D {
    pub buffer: Vec<char>,

    width: usize,
    height: usize,
}

impl CharacterBuffer2D {
    pub fn new(width: usize, height: usize) -> CharacterBuffer2D {
        let mut this = CharacterBuffer2D {
            buffer: Vec::new(),
            width: width,
            height: height,
        };
        this.buffer.resize(width * height, ' ');
        this
    }

    pub fn set(&mut self, x: usize, y: usize, val: char) {
        self.buffer[x + (y * self.width)] = val;
    }

    pub fn set_column(&mut self, col: usize, val: char) {
        for y in 0..self.height {
            self.set(col, y, val);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.buffer[x + (y * self.width)]
    }

    pub fn to_string(&self) -> String {
        self.buffer.clone().into_iter().collect()
    }
}

#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod tests {
    use super::*;

    #[test]
    fn set() {
        let mut buffer = CharacterBuffer2D::new(10, 10);
        buffer.set(0, 0, 'a');
        assert!(buffer.get(0, 0) == 'a');
    }

    #[test]
    #[should_panic]
    fn set_out_of_bounds() {
        let mut buffer = CharacterBuffer2D::new(10, 10);
        buffer.set(20, 20, 'a');
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds() {
        let buffer = CharacterBuffer2D::new(10, 10);
        buffer.get(20, 20);
    }
}
