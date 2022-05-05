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
pub fn push(args: Vec<&str>) -> Result<Vec<String>, String> {
    if args.len() != 2 {
        return Err(String::from("invalid push arguments"));
    }

    let region = args.get(0).ok_or(String::from("push: missing region"))?;
    assert_eq!(*region, "constant");

    let value = args.get(1).ok_or(String::from("push: missing value"))?;

    let mut result = Vec::with_capacity(1);
    // store value in D
    result.push(format!("@{}", value));
    result.push(format!("D=A"));
    // push D to the top of the stack
    result.append(&mut write_head());
    // point stack at next free slot
    result.append(&mut increment_stack());

    Ok(result)
}

pub fn add() -> Result<Vec<String>, String> {
    // 1. pops the top 2 stack elements
    // 2. adds the popped elements
    // 3. pushes the result onto the stack
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

    Ok(result)
}

pub fn sub() -> Result<Vec<String>, String> {
    // 1. pops the top 2 stack elements
    // 2. subs the popped elements
    // 3. pushes the result onto the stack
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

    Ok(result)
}

pub fn neg() -> Result<Vec<String>, String> {
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

    Ok(result)
}

pub fn eq(label_count: &mut Counter) -> Result<Vec<String>, String> {
    Ok(compare(VmComparison::JEQ, label_count))
}

pub fn lt(label_count: &mut Counter) -> Result<Vec<String>, String> {
    Ok(compare(VmComparison::JLT, label_count))
}

pub fn le(label_count: &mut Counter) -> Result<Vec<String>, String> {
    Ok(compare(VmComparison::JLE, label_count))
}

pub fn gt(label_count: &mut Counter) -> Result<Vec<String>, String> {
    Ok(compare(VmComparison::JGT, label_count))
}

pub fn ge(label_count: &mut Counter) -> Result<Vec<String>, String> {
    Ok(compare(VmComparison::JGE, label_count))
}

pub fn and() -> Result<Vec<String>, String> {
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

    Ok(result)
}

pub fn or() -> Result<Vec<String>, String> {
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

    Ok(result)
}

pub fn not() -> Result<Vec<String>, String> {
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

    Ok(result)
}
