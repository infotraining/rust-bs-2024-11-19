 #[derive(Debug, PartialEq)]
 struct Point {
    x: u32,
    y: u32
 }

 #[derive(Debug, PartialEq)]
 enum Direction {
    Up,
    Down,
    Left, 
    Right
 }
 
 #[derive(Debug, PartialEq)]
 struct Snake { 
    points: Vec<Point>,
    direction: Direction
 }

 impl Snake {
    fn new(initial_points: Vec<(u32, u32)>) -> Self {
        let mut points = Vec::new();
        for point in initial_points {
            points.push(Point{x: point.0, y: point.1});
        }
        Snake{points, direction: Direction::Up}
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

    fn next(&mut self) {
        let Point{x, y} = *self.head();

        let new_head = match self.direction {
            Direction::Up => Point{x: x, y: y - 1},
            Direction::Down => Point{x: x, y: y + 1},
            Direction::Left => Point{x: x - 1, y: y},
            Direction::Right => Point{x: x + 1, y: y}
        };

        self.points.insert(0, new_head);
        self.points.pop();
    }
 }

#[test]
fn snake_is_constructed_with_segments() {
    let snake = Snake::new(vec![(0, 0), (1, 0), (2, 0)]);

    assert_eq!(snake.head(), &Point{x: 0, y: 0});
}

#[test]
fn snake_has_default_direction_up() {
    let snake = Snake::new(vec![(0, 0), (1, 0), (2, 0)]);

    assert_eq!(snake.direction(), Direction::Up);
}

#[test]
fn snake_moves_in_given_direction() {
    let mut snake = Snake::new(vec![(5, 5), (5, 6), (5, 7)]);
    snake.set_direction(Direction::Up);
    snake.next();
    assert_eq!(snake, Snake::new(vec![(5, 4), (5, 5), (5, 6)]));
}