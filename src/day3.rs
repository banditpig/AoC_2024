use std::fs;
use std::os::unix::raw::{dev_t, ino_t};
use regex::Regex;
use crate::utils::load_input;
type X = i32;
type Y = i32;
type
Ix = i32;

#[derive(Debug,PartialEq, Copy, Clone)]
enum Op{
    Do(Ix),
    DoNot(Ix),
    Mul(Ix, X, Y),
}
fn extract_ix( op: &Op) -> Ix{
    match op {
        Op::Do(ix) => *ix,
        Op::DoNot(ix) => *ix,
        Op::Mul(ix, _, _) => *ix,
    }
}
#[derive(Debug,PartialEq)]
struct Data{
    instructions: Vec<Instruction>
}
// impl Data {
//     pub fn new(input: Vec<&str>) -> Data{
//         let instructions = input.iter().map(|x| Instruction::new(x.to_string())).collect();
//         Data{
//             instructions
//         }
//     }
//     pub fn evaluate(&mut self){
//         self.instructions.iter_mut().for_each(|x| x.collect_mul());
//
//         //println!("{:?}", &self.instructions);
//         self.instructions.iter_mut().for_each(|x| x.collect_do());
//         //println!("{:?}", &self.instructions);
//         self.instructions.iter_mut().for_each(|x| x.collect_do_not());
//
//         self.instructions.iter_mut().for_each(|x| x.evaluate());
//         println!("{:?}", &self.instructions);
//     }
//
// }
#[derive(Debug,PartialEq)]
struct Instruction{
    memory: String,
    ops: Vec<Op>
}
impl Instruction{
    pub fn new(memory: String) -> Instruction{
        Instruction{
            ops: Vec::new(),
            memory
        }

    }
    fn evaluate(&mut self){
        self.ops.iter_mut().for_each(|x| self.collect_mul());

        //println!("{:?}", &self.instructions);
        self.ops.iter_mut().for_each(|x| self.collect_do());
        //println!("{:?}", &self.instructions);
        self.ops.iter_mut().for_each(|x| self.collect_do_not());
        self.ops.sort_by(|a, b| extract_ix(a).cmp(&extract_ix(b)))

        // self.ops.sort_by(|a, b| match (a, b) {
        //     (Op::Do(a), Op::Do(b)) => a.cmp(b),
        //     (Op::DoNot(a), Op::DoNot(b)) => a.cmp(b),
        //     (Op::Mul(_, a, _), Op::Mul(_, b, _)) => a.cmp(b),
        //     _ => panic!("Invalid Op variant"),
        // });
    }

    fn collect_mul(&mut self){
        let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

        for capture in re.captures_iter(self.memory.as_str()) {
            let whole_match = capture.get(0).unwrap();
            let ix = whole_match.start() as Ix;
            let x = &capture[1].parse::<X>().unwrap();
            let y = &capture[2].parse::<Y>().unwrap();
            self.ops.push(Op::Mul(ix, *x, *y));
        }
    }
    fn collect_do(&mut self){
        let re = Regex::new(r"do\(\)").unwrap();
       // let re = Regex::new(r"stop").unwrap();
        let mut total = 0;
        for capture in re.captures_iter(self.memory.as_str()) {
            let whole_match = capture.get(0).unwrap();
            let ix = whole_match.start() as Ix;
            self.ops.push(Op::Do(ix));
        }
    }
    fn collect_do_not(&mut self){
        let re = Regex::new(r"don't").unwrap();
        for capture in re.captures_iter(self.memory.as_str()) {
            let whole_match = capture.get(0).unwrap();
            let ix = whole_match.start() as Ix;
            self.ops.push(Op::DoNot(ix));
        }
    }
}
pub fn part1(){
    let text = fs::read_to_string("/Users/mikehoughton/RustroverProjects/AoC_2024/data/day3.txt").expect("Something went wrong reading the file");
    //split text on '+
    let all = text.split('+').collect::<Vec<&str>>();

    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let mut total = 0;
    for capture in re.captures_iter(text.as_str()) {
        let whole_match = capture.get(0).unwrap();
        println!("Matched: {}", whole_match.start());
        let x = &capture[1]; // First number
        let y = &capture[2]; // Second number
        total += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();

    }
    println!("Total {}", total);
}
pub fn part2(){
    let text = fs::read_to_string("/Users/mikehoughton/RustroverProjects/AoC_2024/data/day3.txt").expect("Something went wrong reading the file");
    //split text on '+
    let all = text.split('+').collect::<Vec<&str>>();
    let mut data = Instruction::new(text);
    data.evaluate();


}