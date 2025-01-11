use std::fs::read_to_string;
use std::net::UdpSocket;
use std::ptr::read;
use itertools::all;
use tracing::{info, debug};
use crate::utils::load_input;
#[derive(Debug,PartialEq)]
struct Report{
    levels: Vec<usize>,
    initial_direction: Direction,
    bad_level_count: usize
}
#[derive(Debug,PartialEq)]
struct Data{
    reports: Vec<Report>
}
#[derive(Debug,PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    Flat,
}
fn direction(l: usize, r: usize) -> Direction {
    if l < r {
       return  Direction::Increasing
    }
    if l > r {
       return  Direction::Decreasing
    }
   Direction::Flat
}
impl Data {
    pub fn new(reports: Vec<Report>) -> Data {
        Data {
            reports
        }
    }
    // pub fn  safe(&self) -> bool {
    //
    //     self.reports.iter().all(|r| r.safe())
    // }
    pub fn count_safe(&self) -> usize {
        self.reports.iter().filter(|r| r.safe()).count()
    }
    pub fn count_safe_lax(& mut self) -> usize {
       //elf.reports.iter_mut().filter(|mut r| r.safe_lax()).count()
        //count number of safe_lax
        let clean =self.count_safe();
        //
        self.reports.iter().filter(|mut r| r.safe_lax()).count()
    }
}
impl Report {
    fn new(levels: Vec<usize>) -> Report {
        let initial_direction = direction(levels[0], levels[1]);
        Report {
            levels,
            initial_direction,
            bad_level_count: 0
        }
    }


    #[tracing::instrument(level = "debug")]
    pub fn safe(&self) -> bool {
        // if self.initial_direction == Direction::Flat { return false }
        for lr in self.levels.windows(2) {
            let l = lr[0];
            let r = lr[1];

            if self.causes_error(l, r) == true { return false };
        }
        true
    }
    fn causes_error(&self, l: usize, r: usize) -> bool {
        let diff = l.abs_diff(r);
        if diff == 0 { return true }
        if diff > 3 { return true }
        if direction(l, r) != self.initial_direction { return true; }

        false
    }

    fn can_fix(&self, mut ix: i32, r: usize) -> bool {
        if (ix < 0) {
            ix = 0;
        }
        let can_fix = self.causes_error(self.levels[ix as usize], r) == false;
        //println!("can fix {} {} {}", self.levels[ix as usize], r, can_fix);
        can_fix
    }

    pub fn safe_lax(&self) -> bool {
        let mut ix = 0;
        let mut errors = 0;
        let mut allowed = 1;
        println!("{:?}", self.levels);
        for lr in self.levels.windows(2) {
            let l = lr[0];
            let r = lr[1];

            if self.causes_error(l, r) {
                println!("err. can fix {}", self.can_fix(ix - 1, r));
               // print!("{:?}",  );
                // if self.can_fix(ix - 1, r) && allowed == 1{
                //     //errors -= 1;
                //     allowed = 0
                // }else{
                //     errors += 1;
                // }
            }
            ix += 1;
        }

        errors == 0
    }



}

pub fn part1(){

    let data = load_input("../data/day2.txt");
    let reports = data.iter()
        .map(|x| Report::new(x.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect()))
        .collect();
    let d = Data::new(reports);
    println!("{}", d.count_safe());
}
pub fn part2(){
    let data = load_input("../data/day2.txt");
    let reports = data.iter()
        .map(|x| Report::new(x.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect()))
        .collect();
    let mut  d = Data::new(reports);
    println!("{}", d.count_safe_lax());
}