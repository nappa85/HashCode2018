extern crate petgraph;

use std::env;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::Ordering;

use petgraph::graph::DiGraph;
use petgraph::algo::astar;

pub struct Grid {
    rows: u64,
    cols: u64,
    vehicles: Vec<Vehicle>,
    rides: Vec<Ride>,
    bonus: u64,
    steps: u64,
}

impl Grid {
    pub fn new(rows: u64, cols: u64, vehicles: usize, rides: usize, bonus: u64, steps: u64) -> Grid {
        let mut v = Vec::with_capacity(vehicles);

        for i in 0..vehicles {
            v.push(Vehicle::new(i as u64));
        }

        Grid {
            rows: rows,
            cols: cols,
            vehicles: v,
            rides: Vec::with_capacity(rides),
            bonus: bonus,
            steps: steps,
        }
    }

    pub fn add_ride(&mut self, r: usize, s: String) {
        let p: Vec<&str> = s.split(' ').collect();
        assert!(p.len() == 6, format!("Wrong input {}", s));

        self.rides.push(Ride::new(r as u64, p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap(), p[3].parse().unwrap(), p[4].parse().unwrap(), p[5].parse().unwrap()));
    }

    fn run(&self) {
        let mut g = DiGraph::new();

        let mut nodes = Vec::new();
        for r in 0..self.rides.len() {
            nodes.push(g.add_node(r));
        }
        //let's add a virtual node
        let root = nodes.push(g.add_node(self.rides.len()));

        for r in 0..self.rides.len() {
            match self.rides[r].get_root_weight(self.steps) {
                Some((t_min, t_max, p)) => { g.extend_with_edges(&[(root, nodes[r], (t_min, t_max, p))]); }
                None => {},
            }

            for rr in 0..self.rides.len() {
                if r == rr {
                    continue;
                }

                match self.rides[r].get_weight(&self.rides[rr], self.steps) {
                    Some((t_min, t_max, p)) => { g.extend_with_edges(&[(nodes[r], nodes[rr], (t_min, t_max, p))]); }
                    None => {},
                }
            }
        }

        let test = astar(g, root, |node| self.is_goal(node), |edge| self.get_edge_cost(edge), |node| self.estimate_cost(node));
        println!("{:?", test);
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(d: &str) -> Result<Self, Self::Err> {
        let s = d.to_owned();
        let p: Vec<&str> = s.split(' ').collect();
        if p.len() != 6 {
            Err(format!("Wrong input {}", d))
        }
        else {
            Ok(Grid::new(p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap(), p[3].parse().unwrap(), p[4].parse().unwrap(), p[5].parse().unwrap()))
        }
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut s = self.vehicles[0].to_string();

        for n in 1..self.vehicles.len() {
            s = format!("{}\n{}", s, self.vehicles[n].to_string());
        }

        s
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Ride {
    index: u64,
    start: Intersection,
    end: Intersection,
}

impl Ride {
    pub fn new(index: u64, a: u64, b: u64, x: u64, y: u64, s: u64, f: u64) -> Ride {
        Ride {
            index: index,
            start: Intersection::new(a, b, s),
            end: Intersection::new(x, y, f),
        }
    }

    pub fn get_weight(&self, r: &Ride, max_steps: u64) -> Option<(u64, u64, u64)> {
        let step = self.start.t + Intersection::get_distance(&self.start, &self.end);//meanwhile
        let mut time = Intersection::get_distance(&self.end, &r.start);
        let mut points = 0;

        if step + time < r.start.t {
            time += r.start.t - (step + time);
        }

        let distance = Intersection::get_distance(&r.start, &r.end);
        time += distance;
        points += distance;

        if step + time > r.end.t {
            None
        }
        else if step + time > max_steps {
            None
        }
        else {
            Some((r.start.t, r.end.t - distance, points))
        }
    }

    pub fn get_root_weight(&self, max_steps: u64) -> Option<(u64, u64, u64)> {
        let step = 0;
        let mut time = Intersection::get_distance(&Intersection::new(0, 0, 0), &self.start);
        let mut points = 0;

        if step + time < self.start.t {
            time += self.start.t - (step + time);
        }

        let distance = Intersection::get_distance(&self.start, &self.end);
        time += distance;
        points += distance;

        if step + time > self.end.t {
            None
        }
        else if step + time > max_steps {
            None
        }
        else {
            Some((self.start.t, self.end.t - distance, points))
        }
    }
}

impl ToString for Ride {
    fn to_string(&self) -> String {
        format!("start: {}\nend: {}", self.start.to_string(), self.end.to_string())
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Intersection {
    x: u64,
    y: u64,
    t: u64,
}

impl Intersection {
    pub fn new(x: u64, y: u64, t: u64) -> Intersection {
        Intersection {
            x: x,
            y: y,
            t: t,
        }
    }

    fn get_distance(a: &Intersection, b: &Intersection) -> u64 {
        ((a.x as i64 - b.x as i64).abs() + (a.y as i64 - b.y as i64).abs()) as u64
    }
}

impl ToString for Intersection {
    fn to_string(&self) -> String {
        format!("x: {} y: {} t: {}", self.x, self.y, self.t)
    }
}

pub struct Vehicle {
    index: u64,
    runs: Vec<u64>,
    pos: Intersection,
    cur_ride: Option<Ride>
}

impl Vehicle {
    pub fn new(index: u64) -> Vehicle {
        Vehicle {
            index: index,
            runs: Vec::new(),
            pos: Intersection::new(0, 0, 0),
            cur_ride: None,
        }
    }
}

impl ToString for Vehicle {
    fn to_string(&self) -> String {
        let mut s = self.runs.len().to_string();

        for n in 0..self.runs.len() {
            s = format!("{} {}", s, self.runs[n].to_string());
        }

        s
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 3, "Usage: <file.in> <file.out>");

    let fin = File::open(args.get(1).unwrap()).unwrap();
    let file = BufReader::new(&fin);
    let mut iter = file.lines();

    let mut g:Grid = iter.next().unwrap().unwrap().parse().unwrap();//orribile
    for i in 0..g.rides.capacity() {
        g.add_ride(i, iter.next().unwrap().unwrap());
    }

    g.run();

    let mut fout = File::create(args.get(2).unwrap()).unwrap();
    fout.write_all(&g.to_string().into_bytes()).unwrap();
}
