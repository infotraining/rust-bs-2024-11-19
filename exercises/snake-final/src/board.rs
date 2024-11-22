use crate::snake::Point;

#[derive(Debug, PartialEq)]
pub struct Board {
    _width: u32,
    _height: u32,
    _apples: Vec<Point>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        Board {
            _width: width,
            _height: height,
            _apples: Vec::new(),
        }
    }

    pub fn width(&self) -> u32 {
        return self._width;
    }

    pub fn height(&self) -> u32 {
        return self._height;
    }

    pub fn add_apple(&mut self, apple: Point) {
        self._apples.push(apple);
    }

    pub fn apples(&self) -> &Vec<Point> {
        &self._apples
    }

    pub fn try_to_eat_apple(&mut self, point: Point) -> bool {
        if let Some(index) = self._apples.iter().position(|&p| p == point) {
            self._apples.remove(index);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod board_tests {
    use crate::board::Board;

    #[test]
    fn board_has_width_and_height() {
        let board = Board::new(10, 20);

        assert_eq!(board.width(), 10);
        assert_eq!(board.height(), 20);
    }
}

