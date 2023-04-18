use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Binary, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::{fs, str};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // input file containing Hack assembly
    #[arg(short, long)]
    input: String,

    // output binary location
    #[arg(short, long)]
    output: String,
}

#[derive(Debug)]
enum Instruction {
    Addr(usize),
    Comp(CompInstr),
}

#[derive(Debug)]
struct CompInstr {
    dest: Dest,
    comp: Comp,
    jump: Jump,
}

#[derive(Debug)]
enum Comp {
    Zero = 0b0101010,
    One = 0b0111111,
    NegOne = 0b0111010,
    D = 0b0001100,
    A = 0b0110000,
    M = 0b1110000,
    NotD = 0b0001101,
    NotA = 0b0110001,
    NotM = 0b1110001,
    NegD = 0b0001111,
    NegA = 0b0110011,
    NegM = 0b1110011,
    IncD = 0b0011111,
    IncA = 0b0110111,
    IncM = 0b1110111,
    DecD = 0b0001110,
    DecA = 0b0110010,
    DecM = 0b1110010,
    DPlusA = 0b0000010,
    DPlusM = 0b1000010,
    DMinusA = 0b0010011,
    DMinusM = 0b1010011,
    AMinusD = 0b0000111,
    MMinusD = 0b1000111,
    DAndA = 0b0000000,
    DAndM = 0b1000000,
    DOrA = 0b0010101,
    DOrM = 0b1010101,
}

impl FromStr for Comp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Comp::Zero),
            "1" => Ok(Comp::One),
            "-1" => Ok(Comp::NegOne),
            "D" => Ok(Comp::D),
            "A" => Ok(Comp::A),
            "M" => Ok(Comp::M),
            "!D" => Ok(Comp::NotD),
            "!A" => Ok(Comp::NotA),
            "!M" => Ok(Comp::NotM),
            "-D" => Ok(Comp::NegD),
            "-A" => Ok(Comp::NegA),
            "-M" => Ok(Comp::NegM),
            "D+1" => Ok(Comp::IncD),
            "A+1" => Ok(Comp::IncA),
            "M+1" => Ok(Comp::IncM),
            "D-1" => Ok(Comp::DecD),
            "A-1" => Ok(Comp::DecA),
            "M-1" => Ok(Comp::DecM),
            "D+A" => Ok(Comp::DPlusA),
            "D+M" => Ok(Comp::DPlusM),
            "D-A" => Ok(Comp::DMinusA),
            "D-M" => Ok(Comp::DMinusM),
            "A-D" => Ok(Comp::AMinusD),
            "M-D" => Ok(Comp::MMinusD),
            "D&A" => Ok(Comp::DAndA),
            "D&M" => Ok(Comp::DAndM),
            "D|A" => Ok(Comp::DOrA),
            "D|M" => Ok(Comp::DOrM),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Dest(bool, bool, bool); // (M, D, A)

impl FromStr for Dest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Dest(s.contains("A"), s.contains("D"), s.contains("M")))
    }
}

impl Dest {
    fn binary_str(&self) -> String {
        format!(
            "{:b}{:b}{:b}",
            self.0 as isize, self.1 as isize, self.2 as isize
        )
    }
}

#[derive(Debug)]
enum Jump {
    None = 0b000,
    JGT = 0b001,
    JEQ = 0b0010,
    JGE = 0b011,
    JLT = 0b100,
    JNE = 0b101,
    JLE = 0b110,
    JMP = 0b111,
}

impl FromStr for Jump {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JGT" => Ok(Jump::JGT),
            "JEQ" => Ok(Jump::JEQ),
            "JGE" => Ok(Jump::JGE),
            "JLT" => Ok(Jump::JLT),
            "JNE" => Ok(Jump::JNE),
            "JLE" => Ok(Jump::JLE),
            "JMP" => Ok(Jump::JMP),
            _ => Ok(Jump::None),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Compiling {} into {}...", args.input, args.output);

    println!(
        "Starting first stage: Removing comments and whitespace, and converting labels to pointers..."
    );

    let input = fs::read_to_string(args.input)?;

    let mut addrs: HashMap<&str, usize> = HashMap::new();
    let mut var_addr = 16;
    let mut ops: Vec<&str> = Vec::new();

    let mut i = 0;
    for op in input.lines() {
        if op.is_empty() || op.starts_with("//") {
            continue;
        }

        let op = match op.split_once("//") {
            Some(i) => i.0,
            None => op,
        }
        .trim();

        if op.starts_with("(") {
            println!("Label found: {}", op);
            addrs.insert(&op[1..op.len() - 1], i);
        } else {
            ops.push(op);
            i += 1
        }
    }

    println!("{:?}", addrs);

    println!("Starting second stage: Conversion to internal intermediate representation...");

    let mut stack: Vec<Instruction> = Vec::new();

    for op in ops {
        let op = match op.split_once("//") {
            Some(i) => i.0,
            None => op,
        }
        .trim();

        if op.is_empty() || op.starts_with("//") {
        } else if op.starts_with("@") {
            let inst = Instruction::Addr(match op[1..].parse() {
                Ok(i) => i,
                Err(_) => match &op[1..] {
                    "SP" => 0,
                    "LCL" => 1,
                    "ARG" => 2,
                    "THIS" => 3,
                    "THAT" => 4,
                    "SCREEN" => 16384,
                    "KBD" => 24576,
                    _ => match addrs.get(&op[1..]) {
                        // check if address is in map
                        Some(&v) => v,
                        None => match op[2..].parse() {
                            // parse addresses in format @Rx
                            Ok(i) => i,
                            Err(_) => {
                                // add variable to map
                                addrs.insert(&op[1..], var_addr);
                                var_addr += 1;
                                var_addr - 1 // has to be a better way to do this
                            }
                        },
                    },
                },
            });
            println!("addr: {} {:?}", op, inst);
            stack.push(inst);
        } else {
            let mut inst: CompInstr = CompInstr {
                dest: Dest(false, false, false),
                comp: Comp::Zero,
                jump: Jump::None,
            };

            let op: &str = match op.split_once("=") {
                Some(s) => {
                    inst.dest = match Dest::from_str(&s.0) {
                        Ok(i) => i,
                        Err(e) => panic!("error parsing dest: {s:?}\n{e:?}"),
                    };
                    s.1
                }
                None => op,
            };

            let op: &str = match op.split_once(";") {
                Some(s) => {
                    inst.jump = match Jump::from_str(&s.1) {
                        Ok(i) => i,
                        Err(e) => panic!("error parsing jump: {s:?}\n{e:?}"),
                    };
                    s.0
                }
                None => op,
            };

            inst.comp = match Comp::from_str(&op) {
                Ok(i) => i,
                Err(e) => panic!("error parsing comp: {op:?}\n{e:?}"),
            };

            println!("comp: {} {:?}", op, inst);

            stack.push(Instruction::Comp(inst));
        }
    }

    let mut out: String = String::new();

    for op in stack {
        match op {
            Instruction::Addr(a) => out.push_str(&format!("0{:015b}\n", a)[..]),
            Instruction::Comp(c) => out.push_str(
                &format!(
                    "111{:07b}{}{:03b}\n",
                    c.comp as isize,
                    c.dest.binary_str(),
                    c.jump as isize
                )[..],
            ),
        }
    }

    println!("{}", out);

    fs::write(args.output, out).expect("Failed to write file.");

    Ok(())
}
