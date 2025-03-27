use std::fmt;
use std::sync::LazyLock;

pub type Arity = u8;

#[derive(Debug, Clone, PartialEq)]
pub enum Pat {
    X,
    At(Box<Pat>, Box<Pat>),
}

impl fmt::Display for Pat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Pat::*;
        match self {
            At(l, r) if matches!(**r, At(_, _)) => {
                write!(f, "{}({})", l, r)
            }
            X => write!(f, "X"),
            At(p1, p2) => write!(f, "{}{}", p1, p2),
        }
    }
}

pub type Idx = u8;

fn comprehension(ps1: Vec<Pat>, ps2: Vec<Pat>) -> Vec<Pat> {
    ps1.into_iter()
        .flat_map(|p1| {
            ps2.iter()
                .cloned()
                .map(|p2| Pat::At(Box::new(p1.clone()), Box::new(p2)))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub static ALL_PATTERNS: LazyLock<Vec<Pat>> = LazyLock::new(|| {
    use Pat::*;
    let x: Vec<Pat> = vec![X];
    let xx: Vec<Pat> = comprehension(x.clone(), x.clone());
    let xxx: Vec<Pat> = {
        let mut result = comprehension(xx.clone(), x.clone());
        result.extend(comprehension(x.clone(), xx.clone()));
        result
    };
    let xxxx: Vec<Pat> = {
        let mut result = comprehension(xxx.clone(), x.clone());
        result.extend(comprehension(xx.clone(), xx.clone()));
        result.extend(comprehension(x.clone(), xxx.clone()));
        result
    };
    let xxxxx: Vec<Pat> = {
        let mut result = comprehension(xxxx.clone(), x.clone());
        result.extend(comprehension(xxx.clone(), xx.clone()));
        result.extend(comprehension(xx.clone(), xxx.clone()));
        result.extend(comprehension(x.clone(), xxxx.clone()));
        result
    };
    let xxxxxx: Vec<Pat> = {
        let mut result = comprehension(xxxxx.clone(), x.clone());
        result.extend(comprehension(xxxx.clone(), xx.clone()));
        result.extend(comprehension(xxx.clone(), xxx.clone()));
        result.extend(comprehension(xx.clone(), xxxx.clone()));
        result.extend(comprehension(x.clone(), xxxxx.clone()));
        result
    };

    let mut all: Vec<Pat> = Vec::new();
    all.extend(x);
    all.extend(xx);
    all.extend(xxx);
    all.extend(xxxx);
    all.extend(xxxxx);
    all.extend(xxxxxx.iter().take(xxxxxx.len() - 1).cloned());

    all
});

pub fn holes_of(code: u8) -> usize {
    match code {
        0 => 1,
        1 => 2,
        2..=3 => 3,
        4..=8 => 4,
        9..=22 => 5,
        23..=63 => 6,
        _ => {
            panic!("Unknown code!");
        }
    }
}

#[derive(Debug, Clone)]
pub enum Hole {
    // Empty,
    Arg(u8),
    Ptr(u8),
}

enum Mode {
    Spine,
    App1,
    App2,
    App3,
}

#[derive(Default, Debug, Clone)]
pub struct ParseRes {
    pub spine: Vec<Hole>,
    pub app1: Vec<Hole>,
    pub app2: Vec<Hole>,
    pub app3: Vec<Hole>,
}

pub fn parse_pat(p: &Pat) -> ParseRes {
    let mut result = ParseRes::default();
    parse(p, Mode::Spine, &mut 0, &mut 0, &mut result);
    result
}

pub static DECODE_TABLE: LazyLock<[ParseRes; 64]> = LazyLock::new(|| {
    let parsed: Vec<ParseRes> = ALL_PATTERNS.iter().map(|p| parse_pat(p)).collect();
    let res: [ParseRes; 64] = parsed
        .try_into()
        .expect("pattern decode table size should match");
    res
});

#[test]
fn parse_pat_spec() {
    for (pat, res) in ALL_PATTERNS.iter().zip(DECODE_TABLE.iter()).into_iter() {
        println!("{},{:?}", pat, res);
    }
}

fn parse(p: &Pat, mode: Mode, arg_count: &mut u8, ptr_count: &mut u8, acc: &mut ParseRes) {
    let mut stack: Vec<Pat> = Vec::new();
    let mut p_it = p;
    let mut res: Vec<Hole> = Vec::new();

    // push all the branch
    while *p_it != Pat::X {
        match p_it {
            Pat::X => {
                panic!("parse: strange!");
            }
            Pat::At(l, r) => {
                stack.push(*r.clone());
                p_it = l;
            }
        }
    }
    res.push(Hole::Arg(*arg_count));
    *arg_count = *arg_count + 1;

    // pop the stack
    loop {
        match stack.pop() {
            None => {
                break;
            }
            Some(v) => match v {
                Pat::X => {
                    res.push(Hole::Arg(*arg_count));
                    *arg_count = *arg_count + 1;
                }
                Pat::At(_, _) => {
                    // handle nested application
                    res.push(Hole::Ptr(*ptr_count));
                    *ptr_count = *ptr_count + 1;
                    let next_mode: Mode = if *ptr_count == 1 {
                        Mode::App1
                    } else if *ptr_count == 2 {
                        Mode::App2
                    } else {
                        Mode::App3
                    };
                    parse(&v, next_mode, arg_count, ptr_count, acc);
                }
            },
        }
    }

    // accumulate the result
    match mode {
        Mode::Spine => {
            acc.spine = res;
        }
        Mode::App1 => {
            acc.app1 = res;
        }
        Mode::App2 => {
            acc.app2 = res;
        }
        Mode::App3 => {
            acc.app3 = res;
        }
    }
}
