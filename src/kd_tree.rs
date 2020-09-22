use num::one;
use num::traits::Num;

use crate::geometry::Point;

struct Tree<T: Num + Ord + Copy + Default> {
    main: TreeNode<T>,
    size: usize,
}

impl<T: Num + Ord + Copy + Default> Tree<T> {
    fn new(points: Vec<(Point<T>, usize)>) -> Self {
        let ld = points
            .iter()
            .fold((points[0].0.x, points[0].0.y), |acc, arg| {
                if acc.0 > arg.0.x && acc.1 > arg.0.y {
                    (arg.0.x, arg.0.y)
                } else if acc.0 > arg.0.x {
                    (arg.0.x, acc.1)
                } else if acc.1 > arg.0.y {
                    (acc.0, arg.0.y)
                } else {
                    acc
                }
            });
        let ru = points
            .iter()
            .fold((points[0].0.x, points[0].0.y), |acc, arg| {
                if acc.0 < arg.0.x && acc.1 < arg.0.y {
                    (arg.0.x, arg.0.y)
                } else if acc.0 < arg.0.x {
                    (arg.0.x, acc.1)
                } else if acc.1 < arg.0.y {
                    (acc.0, arg.0.y)
                } else {
                    acc
                }
            });
        let mut out = TreeNode {
            values: Vec::new(),
            borders: ((one(), one()), (one(), one())),
            left: None,
            right: None,
        };
        TreeNode::build(&mut out, &points, (ld, ru));
        Tree {
            main: out,
            size: points.len(),
        }
    }
}

// Implements a 2D-TreeNode
#[derive(Default)]
struct TreeNode<T: Num + Ord + Copy + Default> {
    values: Vec<usize>,
    borders: ((T, T), (T, T)),
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Num + Ord + Copy + Default> TreeNode<T> {
    fn query(&self, x1: T, y1: T, x2: T, y2: T) -> Vec<usize> {
        if x1 <= (self.borders.0).0
            && y1 <= (self.borders.0).1
            && (self.borders.1).0 <= x2
            && (self.borders.1).1 <= y2
        {
            return self.values.clone();
        }
        if x1 > (self.borders.1).0
            || y1 > (self.borders.1).1
            || x2 < (self.borders.0).0
            || y2 < (self.borders.0).1
        {
            return Vec::new();
        }
        let mut ans = Vec::new();
        ans.extend_from_slice(match &self.left {
            None => &[],
            Some(left) => (*left).query(x1, y1, x2, y2).as_slice().clone(),
        });
        ans.extend_from_slice(match &self.right {
            None => &[],
            Some(right) => (*right).query(x1, y1, x2, y2).clone().as_slice(),
        });
        return ans;
    }

    fn build(master: &mut Self, points: &Vec<(Point<T>, usize)>, borders: ((T, T), (T, T))) {
        let dir = (borders.1).1 - (borders.0).1 > (borders.1).0 - (borders.0).0;
        let mut left_vals = Vec::new();
        let mut right_vals = Vec::new();
        let two: T = one();
        let two: T = two + one();
        for &e in points {
            if dir {
                if ((borders.1).1 + (borders.0).1) / two > e.0.y {
                    left_vals.push(e);
                } else {
                    right_vals.push(e);
                }
            } else {
                if ((borders.1).0 + (borders.0).0) / two > e.0.x {
                    left_vals.push(e);
                } else {
                    right_vals.push(e);
                }
            }
        }
        let delta = match dir {
            true => ((borders.1).1 - (borders.0).1) / two,
            false => ((borders.1).0 - (borders.0).0) / two,
        };

        master.borders = borders;
        master.values = points.iter().map(|it| it.1).collect();
        if left_vals.len() > 0 {
            master.left = Some(Box::default());
            TreeNode::build(
                master.left.as_mut().unwrap(),
                &left_vals,
                match dir {
                    true => (borders.0, ((borders.1).0, (borders.1).0 + delta)),
                    false => (borders.0, ((borders.1).0 + delta, (borders.1).0)),
                },
            )
        } else {
            master.left = None;
        }
        if right_vals.len() > 0 {
            master.right = Some(Box::default());
            TreeNode::build(
                master.right.as_mut().unwrap(),
                &right_vals,
                match dir {
                    true => (((borders.0).0, (borders.0).1 + delta), borders.1),
                    false => (((borders.0).0 + delta, (borders.0).1), borders.1),
                },
            )
        } else {
            master.right = None;
        }
    }
}
