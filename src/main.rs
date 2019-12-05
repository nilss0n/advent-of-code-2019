use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;
use std::collections::HashMap;

fn calculate_fuel_required(mass: i32) -> i32 {
    cmp::max(mass / 3 - 2, 0)
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_fuel_required_for_fuel(mass: i32) -> i32 {
    let mut fuel_mass = mass;
    let mut extra_fuel = 0;
    while {
        fuel_mass = calculate_fuel_required(fuel_mass);
        extra_fuel += fuel_mass;
        fuel_mass > 0
    } {}
    extra_fuel
}

fn aoc_01() {
 let mut sum : i32 = 0;
    let mut sum2: i32 = 0;
    for line in lines_from_file("inputs/input_01a.txt") {
        let mass = line.parse::<i32>().unwrap();
        let module_fuel = calculate_fuel_required(mass);
        sum += module_fuel;
        let extra_fuel = get_fuel_required_for_fuel(module_fuel);
        sum2 += module_fuel + extra_fuel;
        //println!("{}", line);
    }

    println!("Fuel required for modules {}", sum);

    println!("Total fueld required {}", sum2);
}

fn load_param(p: &Vec<i32>, addr: usize, pmode: i32) -> i32 {
    if pmode == 0 {
        let val_addr = p[addr] as usize;
        p[val_addr]
    } else {
        p[addr]
    }
}

fn store_param(p: &mut Vec<i32>, param_addr: usize, value: i32) {
    let addr = p[param_addr] as usize;
    p[addr] = value;
}

fn get_input() -> i32 {
    println!(">: ");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i32>().unwrap()
}

fn get_param_mode(op: i32, param: i32) -> i32 {
    let ten : i32 = 10;
    (op / ten.pow((param + 1) as u32)) % 10
}

fn get_opcode(instruction: i32) -> i32 {
    instruction % 100
}

fn evaluate_program(p: &mut Vec<i32>) {
    let mut pc = 0;
    let mut instruction = p[pc];
    let mut op = get_opcode(instruction);
    while op != 99 {
        match op {
            1 => {
                let a = load_param(p, pc + 1, get_param_mode(instruction, 1));
                let b = load_param(p, pc + 2, get_param_mode(instruction, 2));
                store_param(p, pc + 3, a + b);
                pc += 4;
            },
            2 => {
                let a = load_param(p, pc + 1, get_param_mode(instruction, 1));
                let b = load_param(p, pc + 2, get_param_mode(instruction, 2));
                store_param(p, pc + 3, a * b);
                pc += 4;
            },
            3 => {
                let input = get_input();
                println!("Received input: {}", input);
                store_param(p, pc + 1, input);
                pc += 2;
            },
            4 => {
                println!(">>>>: {}", load_param(p, pc + 1, get_param_mode(instruction, 1)));
                pc += 2;
            },
            5 => {
                let val = load_param(p, pc + 1, get_param_mode(instruction, 1));
                let addr = load_param(p, pc + 2, get_param_mode(instruction, 2));
                if val != 0 {
                    pc = addr as usize;
                } else {
                    pc += 3;
                }
            },
            6 => {
                let val = load_param(p, pc + 1, get_param_mode(instruction, 1));
                let addr = load_param(p, pc + 2, get_param_mode(instruction, 2));
                if val == 0 {
                    pc = addr as usize;
                } else {
                    pc += 3;
                }
            },
            7 => {
                let a = load_param(p, pc + 1, get_param_mode(instruction, 1));
                let b = load_param(p, pc + 2, get_param_mode(instruction, 2));
                store_param(p, pc + 3, (a < b) as i32);
                pc += 4;
            },
            8 => {
                let a = load_param(p, pc + 1, get_param_mode(instruction, 1));
                let b = load_param(p, pc + 2, get_param_mode(instruction, 2));
                store_param(p, pc + 3, (a == b) as i32);
                pc += 4;
            },
            _ => {
                println!("Reached a wacky opcode {} at PC={}", op, pc);
            }
        }
        instruction = p[pc];
        op = get_opcode(instruction);
    }
}


fn aoc_02() {
    let line = lines_from_file("inputs/input_02.txt")[0].clone();

    let p: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    let mut p1 = p.clone();
    p1[1] = 12;
    p1[2] = 2;

    evaluate_program(&mut p1);
    println!("{}", p1[0]);


    // Part two:
    // 19690720

    for noun in 0..100 {
        for verb in 0..100 {

            let mut p1 = p.clone();
            p1[1] = noun;
            p1[2] = verb;

            evaluate_program(&mut p1);
            let result = p1[0];
            if result == 19690720 {
                println!("part two: {}", noun * 100 + verb);
                return;
            }
        }
    }
}



struct Line {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32
}


fn parse_line_segments_from_input(input: &str) -> Vec<Line> {
    let mut lines : Vec<Line> = Vec::new();

    let steps: Vec<(char, i32)> = input.split(",").map(|s| {
        let dir: char = s.chars().nth(0).unwrap();
        let len = s[1..].parse::<i32>().unwrap();
        (dir, len)
    }).collect();

    let mut x = 0;
    let mut y = 0;
    for (d, l) in steps {
        match d {
            'R' => {
                lines.push(Line {x0: x, y0: y, x1 : x + l, y1: y});
                x += l;
            },
            'L' => {
                lines.push(Line {x0: x, y0: y, x1 : x - l, y1: y});
                x -= l;
            },
            'U' => {
                lines.push(Line {x0: x, y0: y, x1 : x, y1: y + l});
                y += l;
            },
            'D' => {
                lines.push(Line {x0: x, y0: y, x1 : x, y1: y - l});
                y -= l;
            },
            _ => {}
        }
    }

    lines
}

fn aoc_03() {
    //let test1 = vec!["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"];
    let test1 = lines_from_file("inputs/input_03.txt");

    let mut grid : HashMap<(i32, i32), i32> = HashMap::new();
    let a_lines = parse_line_segments_from_input(&test1[0]);
    let b_lines = parse_line_segments_from_input(&test1[1]);

    let mut a_dist = 0;
    for l in a_lines {
        //println!("({}, {}, {}, {})", l.x0, l.y0, l.x1, l.y1);
        if l.x0 == l.x1 {
            // Vertical case
            // Have to loop in increasing order because rust is dumb.
            for y in cmp::min(l.y0, l.y1)..=cmp::max(l.y0, l.y1) {
                grid.insert((l.x0, y), a_dist + (y-l.y0).abs());
            }

            a_dist += (l.y1 - l.y0).abs();
        } else {
            // Horisontal case
            for x in cmp::min(l.x0, l.x1)..=cmp::max(l.x0, l.x1) {
                grid.insert((x, l.y0), a_dist + (x-l.x0).abs());
            }

            a_dist += (l.x1 - l.x0).abs();
        }
    }
    let mut intersections = Vec::new();

    let mut check = |coord, dist| {
        match grid.get(&coord) {
            Some(distance) => {
                intersections.push((coord, distance + dist));
            },
            None => {}
        }
    };

    let mut b_dist = 0;
    for l in b_lines {
        if l.x0 == l.x1 {
            // Vertical case
            for y in cmp::min(l.y0, l.y1)..=cmp::max(l.y0, l.y1) {
                check((l.x0, y), b_dist + (y-l.y0).abs());
            }
            b_dist += (l.y1 - l.y0).abs();

        } else {
            // Horisontal case
            for x in cmp::min(l.x0, l.x1)..=cmp::max(l.x0, l.x1) {
                check((x, l.y0), b_dist + (x-l.x0).abs());
            }
            b_dist += (l.x1 - l.x0).abs();
        }

    }
    let closest_a = intersections.iter().map(|((x, y), _)| x.abs() + y.abs()).filter(|d| *d != 0).min().unwrap();

    let closest_b = intersections.iter().filter_map(|(_, d)| if *d != 0 { Some(d) } else { None }).min().unwrap();
    println!(" Part 1: {}", closest_a);
    println!(" Part 2: {}", closest_b);

}

fn digits(n: i32) -> [i32; 6] {
    let ten: i32 = 10;
    let mut res = [0; 6];
    for (i, p) in (0..6).rev().enumerate() {
        res[i] = (n / ten.pow(p)) % 10
    };
    res
}

fn aoc_04() {

    let mut count = 0;
    let mut count2 = 0;

    for n in 134564 .. 585159 {

        let digits = digits(n);
        let increasing = (0..5).map(|i| digits[i] <= digits[i+1]).all(|x| x);

        let has_pair = (0..5).map(|i| digits[i] == digits[i+1]).any(|x| x);
        let has_exclusive_pair = (0..5).map(|i|
            (i == 0 || digits[i-1] != digits[i]) && digits[i] == digits[i+1] && (i == 4 || digits[i+2] != digits[i])
        ).any(|x| x);


        //println!("{} => {:?} , {} {}", n, digits, decreasing, has_pair);
        if increasing && has_pair {
            count += 1;
        }

        if increasing && has_exclusive_pair {
            count2 += 1;
        }
    }

    println!("{}", count);
    println!("{}", count2);
}


fn aoc_05() {
    // println!("{}", get_param_mode(1102, 1));
    // println!("{}", get_param_mode(1102, 2));
    // println!("{}", get_param_mode(2, 1));
    // println!("{}", get_param_mode(102, 1));
    // println!("{}", get_param_mode(102, 2));

    let src = lines_from_file("inputs/input_05.txt")[0].clone();
    let mut p: Vec<i32> = src.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    evaluate_program(&mut p);
}

fn main() {
    //aoc_01();
    //aoc_02();
    //aoc_03();
    //aoc_04();
    aoc_05();
}
