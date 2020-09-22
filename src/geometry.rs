use num::traits::identities::zero;
use num::traits::Num;
use std::ops::{Add, Sub};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T: Num + Ord + Clone + Copy> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Line<T: Num + Ord + Clone + Copy> {
    pub p1: Point<T>,
    pub p2: Point<T>,
}

impl<T: Num + Ord + Clone + Copy> Point<T> {
    pub fn dist_sqr(&self, o: &Point<T>) -> T {
        (self.x - o.x) * (self.x - o.x) + (self.y - o.y) * (self.y - o.y)
    }

    pub fn cross(&self, o: &Point<T>) -> T {
        self.x * o.y - self.y * o.x
    }

    pub fn dot(&self, o: &Point<T>) -> T {
        self.x * o.x + self.y * o.y
    }

    pub fn orientation(&self, o: &Point<T>) -> i8 {
        let tmp = self.cross(o);
        if tmp < zero() {
            -1
        } else if tmp > zero() {
            1
        } else {
            0
        }
    }
}

impl<T: Num + Ord + Clone + Copy> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Num + Ord + Clone + Copy> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Num + Ord + Clone + Copy> Line<T> {
    pub fn intersects(&self, o: &Line<T>) -> bool {
        (o.p1 - self.p1).orientation(&(self.p2 - self.p1))
            * (o.p2 - self.p1).orientation(&(self.p2 - self.p1))
            < 0
            && (self.p1 - o.p1).orientation(&(o.p2 - o.p1))
                * (self.p2 - o.p1).orientation(&(o.p2 - o.p1))
                < 0
    }
}

pub fn convex_hull<T: Num + Copy + Ord>(I: &Vec<Point<T>>) -> Vec<(usize, Point<T>)> {
    let mut P = I.into_iter().enumerate().collect::<Vec<_>>();
    let n = P.len();
    let mut k: usize = 0;
    P.sort();
    let mut H: Vec<(usize, Point<T>)> = vec![
        (
            0,
            Point {
                x: zero(),
                y: zero()
            }
        );
        2 * n
    ];
    let deref = |it: (usize, &Point<_>)| (it.0, *it.1);
    for i in 0..n {
        while k >= 2 && (H[k - 1].1 - H[k - 1].1).cross(&(*P[i].1 - H[k - 1].1)) <= zero() {
            k -= 1;
        }
        H[k] = deref(P[i]);
        k += 1;
    }
    let t = k + 1;
    for i in (n - 2)..0 {
        while k >= t && (H[k - 1].1 - H[k - 1].1).cross(&(*P[i].1 - H[k - 1].1)) <= zero() {
            k -= 1;
        }
        H[k] = deref(P[i]);
        k += 1;
    }
    H
}

impl<T: Num + Copy + Ord + fmt::Debug> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:?} {:?}", self.x, self.y)
    }
}

impl<T: Num + Copy + Ord + fmt::Debug> fmt::Display for Line<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{} {}", self.p1, self.p2)
    }
}
