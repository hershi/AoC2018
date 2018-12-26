pub fn addr(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize] + input[b as usize];
    output
}

pub fn addi(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize] + b;
    output
}

pub fn mulr(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize] * input[b as usize];
    output
}

pub fn muli(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize] * b;
    output
}

pub fn banr(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize] & input[b as usize];
    output
}

pub fn bani(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize] & b;
    output
}

pub fn borr(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize] | input[b as usize];
    output
}

pub fn bori(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize] | b;
    output
}

pub fn setr(a: i32, _b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = input[a as usize];
    output
}

pub fn seti(a: i32, _b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = a;
    output
}

pub fn gtir(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = if a > input[b as usize] { 1 } else { 0 };
    output
}

pub fn gtri(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = if input[a as usize] > b { 1 } else { 0 };
    output
}

pub fn gtrr(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = if input[a as usize] > input[b as usize] { 1 } else { 0 };
    output
}

pub fn eqir(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = if a == input[b as usize] { 1 } else { 0 };
    output
}

pub fn eqri(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = if input[a as usize] == b { 1 } else { 0 };
    output
}

pub fn eqrr(a: i32, b: i32, c:i32, input: &::Registers) -> ::Registers {
    let mut output = input.clone();
    output[c as usize] = if input[a as usize] == input[b as usize] { 1 } else { 0 };
    output
}

