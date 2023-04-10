use clap::Parser;
use std::error::Error;
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
    Addr(i32),
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
    Zero,
    One,
    NegOne,
    D,
    A,
    NotD,
    NotA,
    NegD,
    NegA,
    IncD,
    IncA,
    DecD,
    DecA,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
}

impl FromStr for Comp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Comp::Zero),
            "1" => Ok(Comp::One),
            "-1" => Ok(Comp::NegOne),
            "D" => Ok(Comp::D),
            "A" | "M" => Ok(Comp::A),
            "!D" => Ok(Comp::NotD),
            "!A" | "!M" => Ok(Comp::NotA),
            "-D" => Ok(Comp::NegD),
            "-A" | "-M" => Ok(Comp::NegA),
            "D+1" => Ok(Comp::IncD),
            "A+1" | "M+1" => Ok(Comp::IncA),
            "D-1" => Ok(Comp::DecD),
            "A-1" | "M-1" => Ok(Comp::DecA),
            "D+A" | "D+M" => Ok(Comp::DPlusA),
            "D-A" | "D-M" => Ok(Comp::DMinusA),
            "A-D" | "M-D" => Ok(Comp::AMinusD),
            "D&A" | "D&M" => Ok(Comp::DAndA),
            "D|A" | "D|M" => Ok(Comp::DOrA),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Dest {
    Null,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

impl FromStr for Dest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" => Ok(Dest::M),
            "D" => Ok(Dest::D),
            "DM" => Ok(Dest::DM),
            "A" => Ok(Dest::A),
            "AM" => Ok(Dest::AM),
            "AD" => Ok(Dest::AD),
            "ADM" => Ok(Dest::ADM),
            _ => Ok(Dest::Null),
        }
    }
}

#[derive(Debug)]
enum Jump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
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
            _ => Ok(Jump::Null),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Compiling {} into {}...", args.input, args.output);

    let input = fs::read_to_string(args.input)?;

    let mut stack: Vec<Instruction> = Vec::new();

    for op in input.lines() {
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
                    _ => match op[2..].parse() {
                        Ok(i) => i,
                        Err(e) => panic!("{} {}", op, e),
                    },
                },
            });
            println!("addr: {} {:?}", op, inst);
            stack.push(inst);
        } else {
            let mut inst: CompInstr = CompInstr {
                dest: Dest::Null,
                comp: Comp::Zero,
                jump: Jump::Null,
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

    Ok(())
}
