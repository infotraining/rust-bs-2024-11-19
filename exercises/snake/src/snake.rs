 #[derive(Debug, PartialEq)]
 struct Point {
    x: u32,
    y: u32
 }

 struct Snake { 
    points: Vec<Point>
 }

 impl Snake {
    fn new(initial_points: Vec<(u32, u32)>) -> Self {
        let mut points = Vec::new();
        for point in initial_points {
            points.push(Point{x: point.0, y: point.1});
        }
        Snake{points}
    }
       
    fn head(&self) -> &Point {
        &self.points[0]
    }
 }

#[test]
fn snake_is_constructed_with_segments() {
    let snake = Snake::new(vec![(0, 0), (1, 0), (2, 0)]);

    assert_eq!(snake.head(), &Point{x: 0, y: 0});
}