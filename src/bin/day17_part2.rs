// Manually translated the input program into source code
//
// ## Input data
// > Register A: 59397658
// > Register B: 0
// > Register C: 0
// >
// > Program: 2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0

fn main() {
    let program = [2, 4, 1, 1, 7, 5, 4, 6, 1, 4, 0, 3, 5, 5, 3, 0];
    let a = backtrack(0, program.as_slice()).unwrap();
    println!("{a}");
    assert_eq!(run(a), program);
}

fn backtrack(a: usize, program: &[usize]) -> Option<usize> {
    for x in 0..8 {
        if step(a * 8 + x) == *program.last().unwrap() {
            if program.len() == 1 {
                return Some(a * 8 + x);
            }
            let res = backtrack(a * 8 + x, &program[..(program.len() - 1)]);
            if res.is_some() {
                return res;
            }
        }
    }
    None
}

fn run(initial_a: usize) -> Vec<usize> {
    let mut reg_a = initial_a;
    let mut output = Vec::new();

    while reg_a != 0 {
        output.push(step(reg_a));
        reg_a /= 8;
    }

    output
}

fn step(reg_a: usize) -> usize {
    let mut reg_b = reg_a & 7;

    reg_b ^= 1;

    let reg_c = reg_a / 2usize.pow(reg_b as u32);

    reg_b ^= reg_c ^ 4;

    reg_b & 7
}

#[cfg(test)]
mod test {
    use super::{backtrack, run};
    const PROGRAM: [usize; 16] = [2, 4, 1, 1, 7, 5, 4, 6, 1, 4, 0, 3, 5, 5, 3, 0];

    #[test]
    fn test_part_1() {
        assert_eq!(run(59397658), vec![4, 6, 1, 4, 2, 1, 3, 1, 6]);
    }

    #[test]
    fn test_backtrack() {
        assert_eq!(backtrack(0, PROGRAM.as_slice()).unwrap(), 202366627359274);
    }

    #[test]
    fn test_answer_is_valid() {
        assert_eq!(run(202366627359274), PROGRAM);
    }
}
