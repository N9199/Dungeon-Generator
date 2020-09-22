use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt;
use std::iter::repeat_with;
use std::vec::Vec;

use num::pow::pow;

use crate::geometry::{convex_hull, Line, Point};

#[derive(Debug)]
pub struct Dungeon {
    g: Graph,         // Dungeon graph
    nodes: Vec<Node>, // Each node is a sub-graph
}

#[derive(Debug)]
struct Node {
    g: Graph,                          // Sub-graph
    edges: Vec<(usize, usize, usize)>, // (Index to Node A, outgoing Graph node, ingoing Node A Graph node)
    inner_edges: Vec<usize>,
}

#[derive(Debug)]
struct Graph {
    size: usize,
    g: Vec<Vec<usize>>, // Adjacency List
}

impl Dungeon {
    pub fn new(n: usize, p: f64, v: Vec<(usize, f64)>) -> Dungeon {
        let nodes: Vec<Node> = v.iter().map(|it| Node::new(it.0, it.1)).collect();
        let connections = nodes.iter().enumerate().fold(
            Vec::new(),
            |mut acc: Vec<(usize, usize)>, arg: (usize, &Node)| {
                acc.append(&mut arg.1.edges(arg.0));
                return acc;
            },
        );
        Dungeon {
            g: Graph::new(n, p), // Separate Graph into planar sub-graphs via throwing the nodes into the plane and for each edges check if it intersects other edges, if it doesn't add it.
            nodes: nodes,
        }
    }
}

impl Node {
    pub fn new(n: usize, p: f64) -> Node {
        let between = Uniform::new_inclusive(0., 1.);
        let bound = 1e3 as i128;
        println!("{}", bound);
        let range = Uniform::new_inclusive(0, bound);
        let radius = (50 * bound) / ((n) as i128);
        let radius_sqr = pow(radius, 2);
        eprintln!("{}, {}", radius, radius_sqr);
        let mut rng = rand::thread_rng();
        let points = repeat_with(|| (range.sample(&mut rng), range.sample(&mut rng)))
            .map(|(x, y)| Point { x, y })
            .take(n)
            .collect::<Vec<_>>();
        let mut graph: Vec<Vec<usize>> = repeat_with(|| Vec::new()).take(n).collect();
        let hull = convex_hull(&points).iter().map(|it| it.0).collect();
        // Maybe look for a data structure which improves the average time of the following part, from n^2 to (build complexity)+n*(query complexity), with (query complexity)~|query result|+log(n), or something similar
        let mut lines: Vec<Line<i128>> = Vec::new();
        let mut pairs = (0..n).fold(Vec::new(), |mut acc1, arg1| {
            acc1.extend_from_slice(
                (arg1 + 1..n)
                    .fold(Vec::new(), |mut acc2, arg2| {
                        acc2.push((arg1, arg2));
                        acc2
                    })
                    .as_mut_slice(),
            );
            acc1
        });
        pairs.shuffle(&mut rng);
        let mut set = HashSet::new();
        for po in pairs {
            let i = po.0;
            let j = po.1;
            if (points[i].dist_sqr(&points[j]) < radius_sqr)
                && ((between.sample(&mut rng) < p || !set.contains(&i) || !set.contains(&j))
                    && lines.iter().fold(true, |acc: bool, arg: &Line<_>| {
                        acc && !Line {
                            p1: points[i],
                            p2: points[j],
                        }
                        .intersects(arg)
                    }))
            {
                //eprintln!("{}", points[i].dist_sqr(&points[j]));
                graph[i].push(j);
                graph[j].push(i);
                lines.push(Line {
                    p1: points[i],
                    p2: points[j],
                });
                set.insert(i);
                set.insert(j);
            }
        }
        println!("{} {}", (&points).len(), (&lines).len());
        eprintln!("{} {}", (&points).len(), (&lines).len());
        for p in points {
            println!("{}", p);
        }
        for l in lines {
            println!("{}", l);
        }

        Node {
            g: Graph { size: n, g: graph },
            edges: Vec::new(),
            inner_edges: hull,
        }
    }

    pub fn edges(&self, i: usize) -> Vec<(usize, usize)> {
        self.inner_edges.iter().map(|it| (*it, i)).collect()
    }
}

impl Graph {
    pub fn new(n: usize, p: f64) -> Graph {
        let mut graph = Graph {
            size: n,
            g: Vec::new(),
        };
        let p = p.max(0.0).min(1.0);
        graph.g.resize(n, Vec::new());
        let between = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        for i in 0..n {
            for j in (i + 1)..n {
                if between.sample(&mut rng) > p {
                    graph.g[i].push(j);
                    graph.g[j].push(i);
                }
            }
        }
        graph
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

impl fmt::Display for Dungeon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let w = 9;
        write!(
            f,
            "Dungeon:\n{:>w1$}\nNodes:\n{:<w2$}",
            self.g,
            self.nodes.iter().fold(String::new(), |acc, arg| acc
                + format!("{:<width$}", arg, width = w).as_str()),
            w1 = w,
            w2 = w + 4
        )
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(width) = f.width() {
            writeln!(
                f,
                "{:>w$} {:?}\n{:>w$}",
                "Edges:",
                self.edges,
                self.g,
                w = width
            )
        } else {
            writeln!(f, "Edges: {:?}\n{}", self.edges, self.g)
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(width) = f.width() {
            write!(
                f,
                "{:>w1$}\n{:>w2$}: {}\n",
                "Graph:",
                "Size",
                self.size,
                w1 = width,
                w2 = width + 4,
            )?;
            for (i, v) in self.g.iter().enumerate() {
                writeln!(f, "{:>w$}: {:?}", i, v, w = width + 4)?
            }
            Ok(())
        } else {
            for (i, v) in self.g.iter().enumerate() {
                writeln!(f, "{}: {:?}", i, v)?
            }
            Ok(())
        }
    }
}
