```rust
use std::f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub slope: f64,
    pub intercept: f64,
}

impl Line {
    pub fn new(slope: f64, intercept: f64) -> Self {
        Line { slope, intercept }
    }

    pub fn from_points(p1: &Point, p2: &Point) -> Self {
        if (p1.x - p2.x).abs() < f64::EPSILON {
            // Vertical line, use a very large slope
            Line {
                slope: f64::MAX,
                intercept: p1.x, // For vertical lines, intercept is the x-coordinate
            }
        } else {
            let slope = (p2.y - p1.y) / (p2.x - p1.x);
            let intercept = p1.y - slope * p1.x;
            Line { slope, intercept }
        }
    }

    pub fn calculate_x(&self, y: f64) -> f64 {
        if self.slope.abs() < f64::EPSILON {
            // Horizontal line, no unique x for a given y
            f64::NAN
        } else if self.slope.abs() >= f64::MAX / 2.0 {
            // Vertical line, return the x-intercept
            self.intercept
        } else {
            (y - self.intercept) / self.slope
        }
    }

    pub fn calculate_y(&self, x: f64) -> f64 {
        if self.slope.abs() >= f64::MAX / 2.0 {
            // Vertical line, no unique y for a given x
            f64::NAN
        } else {
            self.slope * x + self.intercept
        }
    }

    pub fn intersection_with(&self, other: &Line) -> Option<Point> {
        // Check if lines are parallel
        if (self.slope - other.slope).abs() < f64::EPSILON {
            return None;
        }

        // Handle vertical lines
        if self.slope.abs() >= f64::MAX / 2.0 {
            let x = self.intercept;
            let y = other.calculate_y(x);
            return Some(Point { x, y });
        }

        if other.slope.abs() >= f64::MAX / 2.0 {
            let x = other.intercept;
            let y = self.calculate_y(x);
            return Some(Point { x, y });
        }

        // Calculate intersection for non-vertical lines
        let x = (other.intercept - self.intercept) / (self.slope - other.slope);
        let y = self.slope * x + self.intercept;
        Some(Point { x, y })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lane {
    pub start_point: Point,
    pub end_point: Point,
}

impl Lane {
    pub fn new(start_point: Point, end_point: Point) -> Self {
        Lane {
            start_point,
            end_point,
        }
    }

    pub fn length(&self) -> f64 {
        self.start_point.distance_to(&self.end_point)
    }

    pub fn to_line(&self) -> Line {
        Line::from_points(&self.start_point, &self.end_point)
    }

    pub fn midpoint(&self) -> Point {
        self.start_point.midpoint(&self.end_point)
    }

    pub fn angle(&self) -> f64 {
        let dx = self.end_point.x - self.start_point.x;
        let dy = self.end_point.y - self.start_point.y;
        dy.atan2(dx)
    }

    pub fn parallel_to(&self, other: &Lane, threshold: f64) -> bool {
        let self_line = self.to_line();
        let other_line = other.to_line();
        
        // Handle vertical lines
        if self_line.slope.abs() >= f64::MAX / 2.0 && other_line.slope.abs() >= f64::MAX / 2.0 {
            return true;
        }
        
        (self_line.slope - other_line.slope).abs() < threshold
    }
}
```