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
    fn squared(&self) -> f32 {
        (self.x.powi(2)) + (self.y.powi(2))
    }

    /// Returns the absolute value (lengh) of the vector
    fn abs(&self) -> f32 {
        self.squared().sqrt()
    }

    fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Returns the distance between two points
    fn distance_to(&self, other: &Point) -> f32 {
        let diff = self.sub(other);
        diff.abs()
    }

    /// Returns the "game angles" in radians between two point
    fn angle_to(&self, other: &Point) -> f32 {
        let diff = other.sub(&self);
        diff.y.atan2(diff.x)
    }

    fn movement_to(&self, other: &Point) -> Point {
        let angle = self.angle_to(&other);
        Point {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn sub1() {
        let x = Point { x: 8.0, y: 6.0 };
        let y = Point { x: 2.0, y: 3.0 };
        assert_eq!(x.sub(&y), Point { x: 6.0, y: 3.0 });
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
        assert!(error.x < 0.001 );
        assert!(error.y < 0.001 );
    }

    #[test]
    fn movement_to2() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 5.0, y: 0.0 };
        let movement = x.movement_to(&y);
        let error = movement.sub(&Point { x: 1.0, y: 0.0 });
        assert!(error.x < 0.001 );
        assert!(error.y < 0.001 );
    }
    #[test]
    fn movement_to3() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 0.0, y: 3.0 };
        let movement = x.movement_to(&y);
        let error = movement.sub(&Point { x: 0.0, y: 1.0 });
        assert!(error.x < 0.001 );
        assert!(error.y < 0.001 );
    }


    #[test]
    fn movement_to4() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 0.0, y: -10.0 };
        let movement = x.movement_to(&y);
        let error = movement.sub(&Point { x: 0.0, y: -1.0 });
        assert!(error.x < 0.001 );
        assert!(error.y < 0.001 );
    }

    #[test]
    fn movement_to5() {
        let x = Point { x: 0.0, y: 0.0 };
        let y = Point { x: 10.0, y: 10.0 };
        let movement = x.movement_to(&y);
        // let error = movement.sub(&Point { x: 0.5, y: 0.5 });
        assert!(movement.abs() > 0.999999 );
        assert_eq!(movement.x, movement.y );
    }

}
