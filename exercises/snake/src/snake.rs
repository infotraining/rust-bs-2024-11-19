#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Snake {
    points: Vec<Point>,
    direction: Direction,
    is_alive: bool,
}

impl Snake {
    fn new(initial_points: Vec<(u32, u32)>) -> Self {
        let mut points = Vec::new();
        for point in initial_points {
            points.push(Point {
                x: point.0,
                y: point.1,
            });
        }
        Snake {
            points,
            direction: Direction::Up,
            is_alive: true,
        }
    }

    fn with_direction(mut self, direction: Direction) -> Snake {
        self.direction = direction;
        self
    }

    fn head(&self) -> &Point {
        &self.points[0]
    }

    fn direction(&self) -> Direction {
        Direction::Up
    }

    fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    fn next(&mut self, board: &mut Board) {
        let Point { x, y } = *self.head();

        let new_head = match self.direction {
            Direction::Up => Point { x: x, y: y - 1 },
            Direction::Down => Point { x: x, y: y + 1 },
            Direction::Left => Point { x: x - 1, y: y },
            Direction::Right => Point { x: x + 1, y: y },
        };

        if board.is_collision(&new_head) {
            self.is_alive = false;
        }

        if self.points.contains(&new_head) {
            self.is_alive = false;
        }

        self.points.insert(0, new_head.clone());
        
        if !board.try_to_eat_apple(&new_head)
        {
            self.points.pop();
        }

    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}

struct Board {
    width: u32,
    height: u32,

    apples: Vec<Point>
}

impl Board {
    fn is_collision(&self, snake_head: &Point) -> bool {
        snake_head.x == 0
            || snake_head.x >= self.width - 1
            || snake_head.y == 0
            || snake_head.y >= self.height - 1
    }

    fn try_to_eat_apple(&mut self, snake_head: &Point) -> bool {
        if let Some(index) = self.apples.iter().position(|p| p == snake_head) {
            self.apples.remove(index);
            return true;
        }

        false
    }

    pub fn add_apple(&mut self, apple: Point) {
        self.apples.push(apple);
    }

    pub fn apples(&self) -> &Vec<Point> {
        &self.apples
    }
}

#[cfg(test)]
mod tests_snake {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn board() -> Board {
        Board {
            width: 20,
            height: 10,
            apples: Vec::<Point>::new()
        }
    }

    #[test]
    fn snake_is_constructed_with_segments() {
        let snake = Snake::new(vec![(0, 0), (1, 0), (2, 0)]);

        assert_eq!(snake.head(), &Point { x: 0, y: 0 });
    }

    #[test]
    fn snake_has_default_direction_up() {
        let snake = Snake::new(vec![(0, 0), (1, 0), (2, 0)]);

        assert_eq!(snake.direction(), Direction::Up);
    }

    #[rstest]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Right, Snake::new(vec![(6, 5), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Left, Snake::new(vec![(4, 5), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Up, Snake::new(vec![(5, 4), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 7), (5, 6), (5, 5)]), Direction::Down, Snake::new(vec![(5, 8), (5, 7), (5, 6)]))]
    fn snake_moves_in_given_direction(
        mut board: Board,
        #[case] mut snake: Snake,
        #[case] direction: Direction,
        #[case] expected_snake: Snake,
    ) {
        snake.set_direction(direction);
        snake.next(&mut board);
        assert_eq!(snake, expected_snake.with_direction(direction));
    }

    #[rstest]
    #[case(vec![(10, 1)], Direction::Up)]
    #[case(vec![(10, 8)], Direction::Down)]
    #[case(vec![(1, 5)], Direction::Left)]
    #[case(vec![(18, 5)], Direction::Right)]
    fn snake_dies_when_hits_the_wall(
        mut board: Board,
        #[case] segments: Vec<(u32, u32)>,
        #[case] direction: Direction,
    ) {
        let mut snake = Snake::new(segments);
        assert!(snake.is_alive());
        snake.set_direction(direction);
        snake.next(&mut board);
        assert!(!snake.is_alive());
    }

    #[rstest]
    #[case(vec![(5, 5), (5, 6), (5, 7), (4, 7), (4, 6), (4, 5)], Direction::Left)]
    #[case(vec![(5, 5), (5, 6), (5, 7), (6, 7), (6, 6), (6, 5)], Direction::Right)]
    #[case(vec![(5, 5), (6, 5), (7, 5), (7, 4), (6, 4), (5, 4)], Direction::Up)]
    #[case(vec![(5, 5), (5, 6), (5, 7), (5, 4), (5, 3), (5, 2)], Direction::Down)]
    fn snake_dies_when_eats_itself(mut board: Board,
        #[case] segments: Vec<(u32, u32)>,
        #[case] direction: Direction,)
    {
        let mut snake = Snake::new(segments);
        assert!(snake.is_alive());
        snake.set_direction(direction);
        snake.next(&mut board);
        assert!(!snake.is_alive());
    }

    #[rstest]
    fn snake_grows_when_eats_apple(mut board: Board) {
        board.add_apple(Point{x: 5, y: 4});

        let mut snake = Snake::new(vec![(5, 5), (5, 6), (5, 7)]);
        snake.set_direction(Direction::Up);
        snake.next(&mut board);
        assert_eq!(snake, Snake::new(vec![(5, 4), (5, 5), (5, 6), (5, 7)]).with_direction(Direction::Up));
    }

    #[rstest]
    fn apple_is_consumed_when_snake_eats_it(mut board: Board) {
        // arrange
        board.add_apple(Point{x: 5, y: 4});
        assert_eq!(board.apples().len(), 1);

        // act
        let mut snake = Snake::new(vec![(5, 5), (5, 6), (5, 7)]);
        snake.set_direction(Direction::Up);
        snake.next(&mut board);
        assert_eq!(board.apples().len(), 0);
    }
}
