use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Register {
    X,
    Y,
    Z,
    W
}

#[derive(Clone, Copy, Debug)]
enum Value {
    Register(Register),
    Constant(i64),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Inp(Register),
    Add(Register, Value),
    Mul(Register, Value),
    Div(Register, Value),
    Mod(Register, Value),
    Eql(Register, Value),
}

#[derive(Default, Clone, Hash, PartialEq, Eq)]
struct State {
    rx: i64,
    ry: i64,
    rz: i64,
    rw: i64
}

impl State {

    fn get_register(&self, r: &Register) -> i64 {
        match r {
            Register::X => self.rx,
            Register::Y => self.ry,
            Register::Z => self.rz,
            Register::W => self.rw
        }
    }

    fn set_register(&mut self, r: &Register, v: i64) {
        match r {
            Register::X => self.rx = v,
            Register::Y => self.ry = v,
            Register::Z => self.rz = v,
            Register::W => self.rw = v
        }
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input.lines().filter(|line| !line.is_empty())
        .map(|line| {
            let a = match line.chars().nth(4).unwrap() {
                'x' => Register::X,
                'y' => Register::Y,
                'z' => Register::Z,
                'w' => Register::W,
                _ => panic!("invalid register in pos 1")
            };
            match &line[..3] {
                "inp" => {
                    Op::Inp(a)
                },
                s => {
                    let b = match &line[6..] {
                        "x" => Value::Register(Register::X),
                        "y" => Value::Register(Register::Y),
                        "z" => Value::Register(Register::Z),
                        "w" => Value::Register(Register::W),
                        v => Value::Constant(v.parse().unwrap())
                    };
                    match s {
                        "add" => Op::Add(a, b),
                        "mul" => Op::Mul(a, b),
                        "div" => Op::Div(a, b),
                        "mod" => Op::Mod(a, b),
                        "eql" => Op::Eql(a, b),
                        _ => panic!("invalid instruction: {}", s)
                    }
                }
            }
        })
        .collect()
}

fn solve(ops: &[Op]) {
    let mut alus: Vec<(State, (i64, i64))> = vec![(State::default(), (0, 0))];
    for instruction in ops {
        match instruction {
            Op::Inp(r) => {
                let mut new_alus: Vec<(State, (i64, i64))> = Vec::new();
                let mut indexes: HashMap<State, usize> = HashMap::new();
                for (state, (a, b)) in &alus {
                    for v in 1..=9 {
                        let mut new_state = state.clone();
                        new_state.set_register(r, v);
                        let new_a = a * 10 + v;
                        let new_b = b * 10 + v;
                        if let Some(idx) = indexes.get(&new_state) {
                            new_alus[*idx].1.0 = new_alus[*idx].1.0.min(new_a);
                            new_alus[*idx].1.1 = new_alus[*idx].1.1.max(new_b);
                        } else {
                            indexes.insert(new_state.clone(), new_alus.len());
                            new_alus.push((new_state, (new_a, new_b)));
                        }
                    }
                }
                alus = new_alus;
                println!("Processing {} alu states.", alus.len());
            },
            _ => {
                for (state, _) in &mut alus {
                    match instruction {
                        Op::Inp(_) => {},
                        Op::Add(r1, v) => {
                            let res = state.get_register(r1) + match v {
                                Value::Register(r2) => state.get_register(r2),
                                Value::Constant(c) => *c
                            };
                            state.set_register(r1, res);
                        },
                        Op::Mul(r1, v) => {
                            let res = state.get_register(r1) * match v {
                                Value::Register(r2) => state.get_register(r2),
                                Value::Constant(c) => *c
                            };
                            state.set_register(r1, res);
                        },
                        Op::Div(r1, v) => {
                            let v = match v {
                                Value::Register(r2) => state.get_register(r2),
                                Value::Constant(c) => *c
                            };
                            if v == 0 {
                                //return Err(RuntimeError::DivByZero);
                                panic!("div by zero");
                            }
                            let res = state.get_register(r1) / v;
                            state.set_register(r1, res);
                        },
                        Op::Mod(r1, v) => {
                            let b = match v {
                                Value::Register(r2) => state.get_register(r2),
                                Value::Constant(c) => *c
                            };
                            let a = state.get_register(r1);
                            if a < 0 || b <= 0 {
                                //return Err(RuntimeError::ModNegative);
                                panic!("mod negative");
                            }
                            state.set_register(r1, a % b);
                        },
                        Op::Eql(r1, v) => {
                            let a = state.get_register(r1);
                            let b =  match v {
                                Value::Register(r2) => state.get_register(r2),
                                Value::Constant(c) => *c
                            };
                            state.set_register(r1, if a == b { 1 } else { 0 });
                        }
                    }
                }
            }

        }
    }
    let mut lowest = i64::MAX;
    let mut highest = i64::MIN;
    for (state, (a, b)) in alus {
        if state.rz == 0 {
            lowest = lowest.min(a);
            highest = highest.max(b);
        }
    }
    println!("Highest input: {}", highest);
    println!("Lowest input: {}", lowest);
}

fn main() {
    let compile_time = adventofcode2021::time!({
        parse_input(include_str!("../day_24_input.txt"))
    });
    println!("compiled in {}s", compile_time.time);
    let ops = compile_time.result;
    println!("contains {} instructions", ops.len());
    adventofcode2021::print_time!({
        solve(&ops);
    });
}
