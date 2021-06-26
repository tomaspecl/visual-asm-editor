const JUMPS: &[&str] = &["jmp"];
const CONDITIONAL_JUMPS: &[&str] = &["ja", "jae", "jb", "jb", "jbe", "jc", "jcxz", "jecxz", "jrcxz", "je", "jg", "jge", "jl", "jle", "jna", "jnae", "jnb", "jnbe", "jnc", "jne", "jng", "jnge", "jnl", "jnle", "jno", "jnp", "jns", "jnz", "jo", "jp", "jpe", "jpo", "js", "jz"];

// Optional(white-space) label: Optional(white-space) instruction operands Optional(white-space) Optional(;comment)
pub fn contains_jump(str: &str) -> JumpType {
    let instruction_operands = str.trim_start().split(';').next().unwrap().trim_end().rsplit(':').next().unwrap().trim_start();

    let mut i = instruction_operands.split_whitespace();
    if let Some(jmp) = i.next() {
        if let Some(label) = i.next() {
            if JUMPS.contains(&&jmp.to_lowercase()[..]) {
                JumpType::Jmp(label.to_string())
            }else if CONDITIONAL_JUMPS.contains(&&jmp.to_lowercase()[..]) {
                JumpType::CondJmp(label.to_string())
            }else{
                JumpType::None
            }
        }else{
            JumpType::None
        }
    }else{
        JumpType::None
    }
}

pub enum JumpType {
    None,
    Jmp(String),
    CondJmp(String),
}