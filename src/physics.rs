use std::fmt;

// #############################
// #        GAME ANGLES        #
// #############################

// (-1, 1)    (0, 1)    (1, 1)
// (-1, 0)  <- 0, 0 ->  (1, 0)
// (-1,-1)    (0,-1)    (1,-1)

// 3/4 PI      PI/2      PI/4
// PI/-PI        o         0
// -3/4 PI     -PI/2     -PI/4

// #############################
// #        USER ANGLES        #
// #############################

// NW         North     NE
// West     <- 0, 0 ->  East
// SW         South    SE

// 315   0    45
// 270   o    90
// 225  180   135

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({0:.3}, {1:.3})", self.x, self.y)
    }
}

impl Point {
    pub fn squared(&self) -> f32 {
        (self.x.powi(2)) + (self.y.powi(2))
    }

    /// Returns the absolute value (lengh) of the vector
    pub fn abs(&self) -> f32 {
        self.squared().sqrt()
    }

    /// Returns the unit (normalized) Point
    pub fn unit(&self) -> Point {
        let length = self.abs();
        Point {
            x: self.x / length,
            y: self.y / length,
        }
    }

    /// Adds points
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtract points
    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Returns the distance between two points
    pub fn distance_to(&self, other: &Point) -> f32 {
        let diff = self.sub(other);
        diff.abs()
    }

    /// Returns the "game angles" in radians between two point
    pub fn angle_to(&self, other: &Point) -> f32 {
        let diff = other.sub(&self);
        diff.y.atan2(diff.x)
    }

    /// Returns the Point that "moves" in the diretion on destination with unitary length
    pub fn movement_to(&self, other: &Point) -> Point {
        let angle = self.angle_to(&other);
        Point {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn rotated(&self, radians: f32) -> Point {
        let cos = radians.cos();
        let sin = radians.sin();
        let x = (self.x * cos) - (self.y * sin);
        let y = (self.x * sin) + (self.y * cos);
        Point { x, y }
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Return the angle in User Angle
    pub fn user_angle(&self) -> f32 {
        if self.x == 0.0 && self.y == 0.0 {
            return 0.0
        }
        let mut angle = 90.0 - self.angle().to_degrees();
        while angle >= 360.0 {
            angle = angle - 360.0
        }
        while angle < 0.0 {
            angle = angle + 360.0
        }
        angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn similar_points(a: Point, b: Point) -> bool {
        let e1 = (a.x - b.x).abs();
        let e2 = (a.y - b.y).abs();
        println!("{}", e1);
        println!("{}", e2);
        (e1 < 0.000001) && (e2 < 0.000001)
    }

    #[test]
    fn squared1() {
        let x = Point { x: 0.0, y: 3.0 };
        assert_eq!(9.0, x.squared());
    }

    #[test]
    fn squared2() {
        let x = Point { x: 4.0, y: 0.0 };
        assert_eq!(16.0, x.squared());
    }

    #[test]
    fn squared3() {
        let x = Point { x: 4.0, y: 3.0 };
        assert_eq!(25.0, x.squared());
    }

    #[test]
    fn abs1() {
        let x = Point { x: 4.0, y: 3.0 };
        assert_eq!(5.0, x.abs());
    }

    #[test]
    fn abs2() {
        let x = Point { x: 8.0, y: 6.0 };
        assert_eq!(10.0, x.abs());
    }

    #[test]
    fn add1() {
        let x = Point { x: 8.0, y: 6.0 };
        let y = Point { x: 2.0, y: 3.0 };
        assert_eq!(x.add(&y), Point { x: 10.0, y: 9.0 });
    }

    #[test]
    fn add2() {
        let x = Point { x: 2.0, y: 6.0 };
        let y = Point { x: 5.0, y: 30.0 };
        assert_eq!(x.add(&y), Point { x: 7.0, y: 36.0 });
    }

    #[test]
    fn sub1() {
        let x = Point { x: 8.0, y: 6.0 };
        let y = Point { x: 2.0, y: 3.0 };
        assert_eq!(x.sub(&y), Point { x: 6.0, y: 3.0 });
    }

    #[test]
    fn sub2() {
        let x = Point { x: 2.0, y: 6.0 };
        let y = Point { x: 5.0, y: 30.0 };
        assert_eq!(x.sub(&y), Point { x: -3.0, y: -24.0 });
    }

    #[test]
    fn angle_to0() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 0.0, y: 0.0 };
        assert_eq!(x.angle_to(&y), 0.0);
    }

    #[test]
    fn angle_to1() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 1.0, y: 0.0 };
        assert_eq!(x.angle_to(&y), 0.0);
    }

    #[test]
    fn angle_to2() {
        let x = Point { x: 0.0, y: 1.0 };
        let y = Point { x: 1.0, y: 1.0 };
        assert_eq!(x.angle_to(&y), 0.0);
    }

    #[test]
    fn angle_to3() {
        let x = Point { x: -1.0, y: 1.0 };
        let y = Point { x: -1.0, y: -1.0 };
        assert_eq!(x.angle_to(&y), -1.0 * std::f32::consts::FRAC_PI_2);
    }

    #[test]
    fn angle_to4() {
        let x = Point { x: -1.0, y: -1.0 };
        let y = Point { x: 1.0, y: 1.0 };
        assert_eq!(x.angle_to(&y), std::f32::consts::FRAC_PI_4);
    }

    #[test]
    fn movement_to1() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: -5.0, y: 0.0 };
        let movement = x.movement_to(&y);
        let error = movement.sub(&Point { x: -1.0, y: 0.0 });
        assert!(error.x < 0.001);
        assert!(error.y < 0.001);
    }

    #[test]
    fn movement_to2() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 5.0, y: 0.0 };
        let movement = x.movement_to(&y);
        let error = movement.sub(&Point { x: 1.0, y: 0.0 });
        assert!(error.x < 0.001);
        assert!(error.y < 0.001);
    }
    #[test]
    fn movement_to3() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 0.0, y: 3.0 };
        let movement = x.movement_to(&y);
        let error = movement.sub(&Point { x: 0.0, y: 1.0 });
        assert!(error.x < 0.001);
        assert!(error.y < 0.001);
    }

    #[test]
    fn movement_to4() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 0.0, y: -10.0 };
        let movement = x.movement_to(&y);
        let error = movement.sub(&Point { x: 0.0, y: -1.0 });
        assert!(error.x < 0.001);
        assert!(error.y < 0.001);
    }

    #[test]
    fn movement_to5() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 10.0, y: 10.0 };
        let movement = x.movement_to(&y);
        assert!(movement.abs() > 0.999999);
        assert_eq!(movement.x, movement.y);
    }

    #[test]
    fn unit1() {
        let x = Point { x: 10.0, y: 0.0 };
        assert_eq!(x.unit(), Point { x: 1.0, y: 0.0 });
    }
    #[test]
    fn unit2() {
        let x = Point { x: 0.0, y: -23.0 };
        assert_eq!(x.unit(), Point { x: 0.0, y: -1.0 });
    }
    #[test]
    fn unit3() {
        let x = Point { x: -13.0, y: -13.0 };
        assert_eq!(
            x.unit(),
            Point {
                x: -0.70710677,
                y: -0.70710677
            }
        );
    }

    #[test]
    fn rotated1() {
        let x = Point { x: -10.0, y: 1.0 };
        assert!(similar_points(x.rotated(0.0), Point { x: -10.0, y: 1.0 }));
    }

    #[test]
    fn rotated2() {
        let x = Point { x: -10.0, y: 1.0 };
        assert!(similar_points(
            x.rotated(std::f32::consts::FRAC_PI_2),
            Point { x: -1.0, y: -10.0 }
        ));
    }

    #[test]
    fn rotated3() {
        let x = Point { x: -10.0, y: 1.0 };
        assert!(similar_points(
            x.rotated(std::f32::consts::PI),
            Point { x: 10.0, y: -1.0 }
        ));
    }

    #[test]
    fn user_angle0() {
        let x = Point { x: 0.0, y: 0.0 };
        assert_eq!(x.user_angle(), 0.0);
    }

    #[test]
    fn user_angle1() {
        let x = Point { x: 0.0, y: 0.0 };
        assert_eq!(x.user_angle(), 0.0);
    }

    #[test]
    fn user_angle2() {
        let x = Point { x: 1.0, y: 0.0 };
        assert_eq!(x.user_angle(), 90.0);
    }

    #[test]
    fn user_angle3() {
        let x = Point { x: -1.0, y: 0.0 };
        assert_eq!(x.user_angle(), 270.0);
    }

    #[test]
    fn user_angle4() {
        let x = Point { x: 0.0, y: 1.0 };
        assert_eq!(x.user_angle(), 0.0);
    }

    #[test]
    fn user_angle5() {
        let x = Point { x: 0.0, y: -1.0 };
        assert_eq!(x.user_angle(), 180.0);
    }
}
