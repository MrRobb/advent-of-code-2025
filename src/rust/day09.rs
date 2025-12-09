use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
enum Edge {
    Vertical { x: isize, min_y: isize, max_y: isize },
    Horizontal { min_x: isize, max_x: isize, y: isize },
}

impl Edge {
    fn from_points(p1: &Point, p2: &Point) -> Option<Self> {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);
        match (min_x == max_x, min_y == max_y) {
            (true, false) => Some(Self::Vertical { x: min_x, min_y, max_y }),
            (false, true) => Some(Self::Horizontal { min_x, max_x, y: min_y }),
            _ => None,
        }
    }

    const fn contains(&self, point: &Point) -> bool {
        match self {
            Self::Horizontal { min_x, max_x, y } => point.y == *y && *min_x <= point.x && point.x <= *max_x,
            Self::Vertical { x, min_y, max_y } => point.x == *x && *min_y <= point.y && point.y <= *max_y,
        }
    }

    const fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            // rect edge is horizontal, polygon edge is vertical
            (
                Self::Horizontal { min_x, max_x, y: h_y },
                Self::Vertical { x: v_x, min_y, max_y }
            ) |
            // rect edge is vertical, polygon edge is horizontal
            (
                Self::Vertical { x: v_x, min_y, max_y },
                Self::Horizontal { min_x, max_x, y: h_y }
            ) => {
                *min_x < *v_x && *v_x < *max_x && *min_y < *h_y && *h_y < *max_y
            },

            // parallel
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl Rect {
    const fn from_points(p1: &Point, p2: &Point) -> Self {
        Self {
            x1: p1.x,
            x2: p2.x,
            y1: p1.y,
            y2: p2.y,
        }
    }

    fn area(&self) -> isize {
        (self.x2.max(self.x1) - self.x1.min(self.x2) + 1) * (self.y2.max(self.y1) - self.y1.min(self.y2) + 1)
    }

    fn corners(&self) -> Vec<Point> {
        vec![
            Point { x: self.x1, y: self.y1 },
            Point { x: self.x1, y: self.y2 },
            Point { x: self.x2, y: self.y2 },
            Point { x: self.x2, y: self.y1 },
        ]
    }

    fn edges(&self) -> Vec<Edge> {
        self.corners()
            .iter()
            .circular_tuple_windows()
            .filter_map(|(p1, p2)| Edge::from_points(p1, p2))
            .collect()
    }
}

struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn contains_point(&self, point: &Point) -> bool {
        let mut is_inside = false;
        for edge in self
            .points
            .iter()
            .circular_tuple_windows()
            .filter_map(|(p1, p2)| Edge::from_points(p1, p2))
        {
            // Point on edge
            if edge.contains(point) {
                return true;
            }
            match edge {
                // Parallel to the ray
                Edge::Horizontal { .. } => (),
                Edge::Vertical { x, min_y, max_y } => {
                    // Crosses the ray
                    if min_y <= point.y && point.y < max_y && point.x < x {
                        is_inside = !is_inside;
                    }
                },
            }
        }
        is_inside
    }

    fn intersect_rect(&self, rect: &Rect) -> bool {
        self.points
            .iter()
            .circular_tuple_windows()
            .filter_map(|(p1, p2)| Edge::from_points(p1, p2))
            .any(|polygon_edge| rect.edges().iter().any(|edge| edge.intersects(&polygon_edge)))
    }

    fn contains_rect(&self, rect: &Rect) -> bool {
        rect.corners().iter().all(|p| self.contains_point(p)) && !self.intersect_rect(rect)
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .split('\n')
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            Point {
                x: a.parse().unwrap(),
                y: b.parse().unwrap(),
            }
        })
        .collect()
}

pub fn get_largest_rect_area(input: &str) -> isize {
    let points = parse_input(input);
    points
        .into_iter()
        .tuple_combinations()
        .map(|(p1, p2)| Rect::from_points(&p1, &p2))
        .map(|rect| rect.area())
        .max()
        .unwrap()
}

pub fn get_largest_rect_in_polygon_area(input: &str) -> isize {
    let polygon = Polygon {
        points: parse_input(input),
    };
    polygon
        .points
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| Rect::from_points(p1, p2))
        .filter(|rect| polygon.contains_rect(rect))
        .map(|rect| rect.area())
        .max()
        .unwrap()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day09.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", get_largest_rect_area(&input));
    println!("PART 2 = {}", get_largest_rect_in_polygon_area(&input));
    println!("Execution time: {:?}", now.elapsed());
}
