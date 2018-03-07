
use std::env;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::PartialEq;
use std::cmp::Ordering;

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

    pub fn recursively_get_best_path(&self, paths: &mut HashMap<(u64, u64), Vec<u64>>, step: u64, position: &Intersection, rides: Vec<Ride>, points: u64, path: Vec<u64>) {
        //print!("Remaining rides after path {:?}: ", path);
        //for r in 0..rides.len() {
            //print!("{} ", rides[r].index);
        //}
        //print!("\n");

        //here we can tune the deepness of the algorithm
        if (rides.len() == 0) || (path.len() >= 2) {
            //println!("Inserting path {:?} with {} points on {} steps", path, points, step);
            paths.insert((points, step), path.clone());
        }
        else {
            for r in 0..rides.len() {
                match position.get_points(step, self.steps, self.bonus, &rides[r]) {
                    Some((p, t)) => {
                        let mut temp_r = rides.clone();
                        temp_r.remove(r);
                        let mut temp_p = path.clone();
                        temp_p.push(rides[r].index);
                        self.recursively_get_best_path(paths, step + t, &rides[r].end, temp_r, points + p, temp_p);
                    },
                    None => {
                        if path.len() > 0 {
                            //println!("Inserting path {:?} with {} points on {} steps", path, points, step);
                            paths.insert((points, step), path.clone());
                        }
                    },
                }
            }
        }
    }

    pub fn run(&mut self) {
        for step in 0..self.steps {
            for v in 0..self.vehicles.len() {
                if !self.vehicles[v].is_free() {
                    //println!("Vehicle {} is busy", v);
                    continue;
                }

                //println!("Vehicle {} is free", v);

                //print!("Possible rides for vehicle {}: ", v);
                //for r in &self.rides {
                    //print!("{} ", r.index);
                //}
                //print!("\n");

                let mut paths: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
                self.recursively_get_best_path(&mut paths, step, &self.vehicles[v].pos, self.rides.clone(), 0, Vec::new());
                let mut ranks:Vec<&(u64, u64)> = paths.keys().collect();
                ranks.sort_by(|&&(p1, t1), &&(p2, t2)| {
                    match p1.cmp(&p2) {
                        Ordering::Equal => match t1.cmp(&t2) {
                            Ordering::Equal => Ordering::Equal,
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                        },
                        Ordering::Less => Ordering::Greater,
                        Ordering::Greater => Ordering::Less,
                    }
                });

                //print!("Possible paths: ");
                //for (&(p, t), vec) in &paths {
                    //print!("{:?} with {} points and {} steps ", vec, p, t);
                //}
                //print!("\n");

                match ranks.first() {
                    Some(&&(p, t)) => {
                        let path = &paths[&(p, t)];
                        //println!("Best path for vehicle {} seems to be {:?} with {} points in {} steps", v, path, p, t);
                        for r in 0..self.rides.len() {
                            if self.rides[r].index == path[0] {
                                self.vehicles[v].set_ride(self.rides.swap_remove(r));
                                break;
                            }
                        }
                    },
                    None => {
                        //println!("Vehicle {} hasn't viable rides", v);
                    },
                }

//                 let mut rides:HashMap<(u64, u64), usize> = HashMap::new();
//                 for r in 0..self.rides.len() {
//                     //for this Vehicle this ride isn't feasible
//                     match self.vehicles[v].get_points(step, self.steps, self.bonus, &self.rides[r]) {
//                         Some((p, t)) => {
//                             //println!("Ride {} would ends at {}", r, t);
//                             rides.insert((p, t), r);
//                         },
//                         None => {
//                             continue;
//                         },
//                     }
//                 }
// 
//                 let mut ranks:Vec<&(u64, u64)> = rides.keys().collect();
//                 ranks.sort_by(|&&(p1, t1), &&(p2, t2)| {
// //                     match p1.cmp(&p2) {
// //                         Ordering::Equal => match t1.cmp(&t2) {
// //                             Ordering::Equal => Ordering::Equal,
// //                             Ordering::Less => Ordering::Greater,
// //                             Ordering::Greater => Ordering::Less,
// //                         },
// //                         Ordering::Less => Ordering::Greater,
// //                         Ordering::Greater => Ordering::Less,
// //                     }
//                     match p1.cmp(&p2) {
//                         Ordering::Equal => Ordering::Equal,
//                         Ordering::Less => Ordering::Greater,
//                         Ordering::Greater => Ordering::Less,
//                     }
//                 });
//                 match ranks.first() {
//                     Some(&&(p, t)) => {
//                         self.vehicles[v].set_ride(self.rides.swap_remove(rides[&(p, t)]));
//                     },
//                     None => {
//                         //println!("Vehicle {} hasn't viable rides", v);
//                     },
//                 }
            }
        }
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

#[derive(PartialEq, Clone)]
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
}

impl ToString for Ride {
    fn to_string(&self) -> String {
        format!("start: {}\nend: {}", self.start.to_string(), self.end.to_string())
    }
}

#[derive(PartialEq, Clone)]
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

    pub fn get_distance(a: &Intersection, b: &Intersection) -> u64 {
        ((a.x as i64 - b.x as i64).abs() + (a.y as i64 - b.y as i64).abs()) as u64
    }

    pub fn get_points(&self, step: u64, max_step: u64, bonus: u64, r: &Ride) -> Option<(u64, u64)> {
        let mut time = Intersection::get_distance(&self, &r.start);
        let mut points = 0;

        if step + time <= r.start.t {
            time += r.start.t - (step + time);
            points += bonus;
        }

        let distance = Intersection::get_distance(&r.start, &r.end);
        time += distance;
        points += distance;

        if step + time > r.end.t {
            //println!("Discarding ride {} because cannot end in time ({} > {})", r.index, step + time, r.end.t);
            None
        }
        else if step + time > max_step {
            //println!("Discarding ride {} because cannot end in time ({} > {})", r.index, step + time, max_step);
            None
        }
        else {
            //println!("Ride {} gives {} points in {} steps (ends at step {} <= {})", r.index, points, time, step + time, max_step);
            Some((points, time))
        }
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

    pub fn get_remaining_time(&self) -> u64 {
        match self.cur_ride {
            Some(ref ride) => {
                Intersection::get_distance(&self.pos, &ride.end)
            },
            None => 0,
        }
    }

    pub fn get_start_distance(&self, r: &Ride) -> u64 {
        Intersection::get_distance(&self.pos, &r.start)
    }

    pub fn get_end_distance(&self, r: &Ride) -> u64 {
        Intersection::get_distance(&self.pos, &r.end)
    }

//     pub fn get_points(&self, step: u64, max_step: u64, bonus: u64, r: &Ride) -> Option<(u64, u64)> {
//         let mut time = self.get_start_distance(r);
//         let mut points = 0;
// 
//         if step + time <= r.start.t {
//             time += r.start.t - (step + time);
//             points += bonus;
//         }
// 
//         let distance = Intersection::get_distance(&r.start, &r.end);
//         time += distance;
//         points += distance;
// 
//         if step + time > r.end.t {
//             //println!("Discarding ride {} because cannot end in time ({} > {})", r.index, time, r.end.t);
//             None
//         }
//         else if step + time > max_step {
//             //println!("Discarding ride {} because cannot end in time ({} > {})", r.index, time, max_step);
//             None
//         }
//         else {
//             Some((points, time))
//         }
//     }

    pub fn is_free(&mut self) -> bool {
        if self.cur_ride.is_some() {
            self.pos.t -= 1;
            //println!("Vehicle {} moved, {} moves to end", self.index, self.pos.t);
            if self.pos.t == 0 {
                match self.cur_ride {
                    Some(ref r) => {
                        self.pos.x = r.end.x;
                        self.pos.y = r.end.y;
                    },
                    None => {
                        assert!(false, "How is it even possible?");
                    },
                }
                self.cur_ride = None;
                true
            }
            else {
                false
            }
        }
        else {
            true
        }
    }

    pub fn set_ride(&mut self, r: Ride) {
        //println!("Vehicle {} taking ride {}", self.index, r.index);
        self.pos.t = self.get_start_distance(&r) + Intersection::get_distance(&r.start, &r.end);
        self.runs.push(r.index);
        self.cur_ride = Some(r);
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
