pub enum OpCode {
    // memory access
    Push,
    Pop,
    // arithmetic
    Add,
    Sub,
    Neg,
    // comparison
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    // bitwise logical
    And,
    Or,
    Not,
}

#[derive(Debug)]
pub enum RegionType {
    Constant,
    Fixed(u32),
    Dynamic(u32),
}

#[derive(Debug)]
pub enum Region {
    Constant,
    Pointer,
    Temp,
    Static,
    Local,
    Argument,
    This,
    That,
}

impl Region {
    pub fn offset(self) -> RegionType {
        match self {
            Region::Constant => RegionType::Constant,
            Region::Pointer => RegionType::Fixed(3),
            Region::Temp => RegionType::Fixed(5),
            Region::Static => RegionType::Fixed(16),
            Region::Local => RegionType::Dynamic(1),
            Region::Argument => RegionType::Dynamic(2),
            Region::This => RegionType::Dynamic(3),
            Region::That => RegionType::Dynamic(4),
        }
    }
}

pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter{ count: 0 }
    }

    pub fn inc(&mut self) -> u32 {
        self.count += 1;

        self.count
    }
}

// hack helpers
fn increment_stack() -> Vec<String> {
    vec![
        format!("@0"),
        format!("M=M+1"),
    ]
}

fn decrement_stack() -> Vec<String> {
    vec![
        format!("@0"),
        format!("M=M-1"),
    ]
}

fn write_head() -> Vec<String> {
    vec![
        format!("@0"),
        format!("A=M"),
        format!("M=D"),
    ]
}

fn read_head() -> Vec<String> {
    vec![
        format!("A=M"),
        format!("D=M"),
    ]
}

fn read_negated_head() -> Vec<String> {
    vec![
        format!("A=M"),
        format!("D=-M"),
    ]
}

enum VmComparison {
    JEQ,
    JNE,
    JLT,
    JLE,
    JGT,
    JGE,
}

impl ToString for VmComparison {
    fn to_string(&self) -> String {
        match *self {
            VmComparison::JEQ => "JEQ".to_owned(),
            VmComparison::JNE => "JNE".to_owned(),
            VmComparison::JLT => "JLT".to_owned(),
            VmComparison::JLE => "JLE".to_owned(),
            VmComparison::JGT => "JGT".to_owned(),
            VmComparison::JGE => "JGE".to_owned(),
        }
    }
}

fn compare(operator: VmComparison, label_count: &mut Counter) -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    let true_branch = format!("LOW_LEVEL_LABEL{}", label_count.inc());
    let continue_branch = format!("LOW_LEVEL_LABEL{}", label_count.inc());

    // point at the first populated element
    result.append(&mut decrement_stack());
    // set D to -HEAD
    result.append(&mut read_negated_head());
    // point at the next element
    result.append(&mut decrement_stack());
    // get diff of 1st and 2nd element
    result.push(format!("A=M"));
    result.push(format!("D=D+M"));
    // jump false if
    result.push(format!("@{}", true_branch));
    result.push(format!("D;{}", operator.to_string()));
    // DEFAULT: IS FALSE
    result.push(format!("D=0"));
    result.push(format!("@{}", continue_branch));
    result.push(format!("0;JMP"));
    // JUMP: IS TRUE
    result.push(format!("({})", true_branch));
    result.push(format!("D=-1"));
    // JUMP: CONTINUE
    result.push(format!("({})", continue_branch));
    // set HEAD to D
    result.append(&mut write_head());
    // point at next free slot
    result.append(&mut increment_stack());

    result
}

// todo: only implements constant right now
pub fn push(region: Region, index: u32) -> Vec<String> {
    match region.offset() {
        RegionType::Constant => push_constant(index),
        RegionType::Dynamic(offset) => push_dynamic(offset, index),
        RegionType::Fixed(offset) => push_fixed(offset, index),
    }
}

fn push_constant(value: u32) -> Vec<String> {
    let mut result = Vec::with_capacity(1);
    // inject constant via A-instruction
    result.push(format!("@{}", value));
    result.push(format!("D=A"));
    // push onto stack
    result.append(&mut write_head());
    // point stack at next free slot
    result.append(&mut increment_stack());

    result
}

fn push_fixed(offset: u32, index: u32) -> Vec<String> {
    let address = offset + index;

    let mut result = Vec::with_capacity(1);

    // read from memory
    result.push(format!("@{}", address));
    result.push(format!("D=M"));
    // push onto stack
    result.append(&mut write_head());
    // point stack at next free slot
    result.append(&mut increment_stack());

    result
}

fn push_dynamic(offset: u32, index: u32) -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    // load offset
    result.push(format!("@{}", offset));
    result.push(format!("D=M"));
    // add index to offset
    result.push(format!("@{}", index));
    result.push(format!("D=D+A"));
    // read from memory
    result.push(format!("A=D"));
    result.push(format!("D=M"));
    // push onto stack
    result.append(&mut write_head());
    // point stack at next free slot
    result.append(&mut increment_stack());

    result
}

pub fn pop(region: Region, index: u32) -> Vec<String> {
    match region.offset() {
        RegionType::Constant => panic!("can't pop to constant"),
        RegionType::Dynamic(offset) => pop_dynamic(offset, index),
        RegionType::Fixed(offset) => pop_fixed(offset, index),
    }
}

fn pop_dynamic(offset: u32, index: u32) -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    // point at first data
    result.append(&mut decrement_stack());
    // read HEAD
    result.append(&mut read_head());
    // store HEAD in R13
    result.push(format!("@R13"));
    result.push(format!("M=D"));

    // load offset
    result.push(format!("@{}", offset));
    result.push(format!("D=M"));
    // add index to offset
    result.push(format!("@{}", index));
    result.push(format!("D=D+A"));
    // store address in R14
    result.push(format!("@R14"));
    result.push(format!("M=D"));
    // load HEAD into D
    result.push(format!("@R13"));
    result.push(format!("D=M"));
    // load address into A
    result.push(format!("@R14"));
    result.push(format!("A=M"));
    // write D to address
    result.push(format!("M=D"));

    result
}

fn pop_fixed(offset: u32, index: u32) -> Vec<String> {
    let address = offset + index;

    let mut result = Vec::with_capacity(1);

    // point at first variable
    result.append(&mut decrement_stack());
    // read from HEAD
    result.append(&mut read_head());
    // write to memory
    result.push(format!("@{}", address));
    result.push(format!("M=D"));

    result
}

pub fn add() -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    // point at the first populated element
    result.append(&mut decrement_stack());
    // load the first value from the stack
    result.append(&mut read_head());
    // decrement the stack pointer to read next value
    result.append(&mut decrement_stack());
    // add the next value to D register
    result.push(format!("A=M"));
    result.push(format!("D=D+M"));
    // overwrite the previous value with D
    result.append(&mut write_head());
    // point stack at next free slot
    result.append(&mut increment_stack());

    result
}

pub fn sub() -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    // point at the first populated element
    result.append(&mut decrement_stack());
    // set D to HEAD
    result.append(&mut read_negated_head());
    // point at the next populated element
    result.append(&mut decrement_stack());
    // add HEAD to D
    result.push(format!("A=M"));
    result.push(format!("D=D+M"));
    // set HEAD to D
    result.append(&mut write_head());
    // point at next free slot
    result.append(&mut increment_stack());

    result
}

pub fn neg() -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    // point at the first populated element
    result.append(&mut decrement_stack());
    // set D to HEAD
    result.push(format!("A=M"));
    result.push(format!("D=-M"));
    // set HEAD to D
    result.append(&mut write_head());
    // point at next free slot
    result.append(&mut increment_stack());

    result
}

pub fn eq(label_count: &mut Counter) -> Vec<String> {
    compare(VmComparison::JEQ, label_count)
}

pub fn lt(label_count: &mut Counter) -> Vec<String> {
    compare(VmComparison::JLT, label_count)
}

pub fn le(label_count: &mut Counter) -> Vec<String> {
    compare(VmComparison::JLE, label_count)
}

pub fn gt(label_count: &mut Counter) -> Vec<String> {
    compare(VmComparison::JGT, label_count)
}

pub fn ge(label_count: &mut Counter) -> Vec<String> {
    compare(VmComparison::JGE, label_count)
}

pub fn and() -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    // point at the first arg
    result.append(&mut decrement_stack());
    // set D to HEAD
    result.append(&mut read_head());
    // point at the second arg
    result.append(&mut decrement_stack());
    // set D to D&M
    result.push(format!("A=M"));
    result.push(format!("D=D&M"));
    // set HEAD to D
    result.append(&mut write_head());
    // point at next free slot
    result.append(&mut increment_stack());

    result
}

pub fn or() -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    // point at the first populated element
    result.append(&mut decrement_stack());
    // set D to HEAD
    result.append(&mut read_head());
    // point at the next element
    result.append(&mut decrement_stack());
    // set D to D&M
    result.push(format!("A=M"));
    result.push(format!("D=D|M"));
    // set HEAD to D
    result.append(&mut write_head());
    // point at next free slot
    result.append(&mut increment_stack());

    result
}

pub fn not() -> Vec<String> {
    let mut result = Vec::with_capacity(1);

    // point at the first populated element
    result.append(&mut decrement_stack());
    // set D to NOT HEAD
    result.push(format!("A=M"));
    result.push(format!("D=!M"));
    // set HEAD to D
    result.append(&mut write_head());
    // point at next free slot
    result.append(&mut increment_stack());

    result
}
