use crate::utils::load_input;

struct Report{
    levels: Vec<usize>,
    initial_direction: Direction
}
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
}
impl Report {
    fn new(levels: Vec<usize> ) -> Report {
        let initial_direction = direction(levels[0], levels[1]);
        Report {
            levels,
            initial_direction,
        }
    }
    pub fn safe(&self) -> bool {
        if self.initial_direction == Direction::Flat { return false }
        for lr in self.levels.windows(2){
            let l =  lr[0];
            let r = lr[1];
            let diff = l.abs_diff(r);
            if diff == 0 { return false }
            if diff > 3 { return false }
            let d = direction(l, r);
            if d != self.initial_direction { return false }

        }
        true
    }
    fn causes_error(&self, l:usize, r:usize) -> bool {
        let diff = l.abs_diff(r);
        if diff == 0 { return false }
        if diff > 3 { return false }
        let d = direction(l, r);
        if d == self.initial_direction { return true; }
        false
    }
    pub fn safe_lax(&self) -> bool {
        if self.initial_direction == Direction::Flat { return false }
        for lr in self.levels.windows(2){
            let l =  lr[0];
            let r = lr[1];

            return self.causes_error(l, r);

        }
        true
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