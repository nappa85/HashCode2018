
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;

struct Pizza {
    rows: usize,
    cols: usize,
    minIngr: usize,
    maxCells: usize,
    matrix: Vec<Vec<String>>
}

impl Pizza {
    fn addRow(&mut self, r: usize, s: String) {
        self.matrix.push(Vec::with_capacity(self.cols));
        for (c, v) in s.split(' ').enumerate() {
            self.matrix[r].push(v.to_owned());
        }
    }
}

impl FromStr for Pizza {
    type Err = String;

    fn from_str(d: &str) -> Result<Self, Self::Err> {
        let s = d.to_owned();
        let p: Vec<&str> = s.split(' ').collect();
        if p.len() != 4 {
            Err(format!("Wrong input {}", d))
        }
        else {
            let rows:usize = p[0].parse().unwrap();
            let cols:usize = p[1].parse().unwrap();
            Ok(Pizza {
                rows: rows,
                cols: cols,
                minIngr: p[2].parse().unwrap(),
                maxCells: p[3].parse().unwrap(),
                matrix: Vec::with_capacity(rows),
            })
        }
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Usage: <file.in>");

    let f = File::open(args.get(1).unwrap()).unwrap();
    let file = BufReader::new(&f);
    let mut iter = file.lines();

    let mut p:Pizza = iter.next().unwrap().unwrap().parse().unwrap();//orribile
    for i in 0..p.rows {
        p.addRow(i, iter.next().unwrap().unwrap());
    }
}
