// Manually translated the input program into source code
//
// ## Input data
// > Register A: 59397658
// > Register B: 0
// > Register C: 0
// >
// > Program: 2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0

fn main() {
    let mut reg_a = 59397658;
    let mut reg_b;
    let mut reg_c;
    let mut output = String::new();

    while reg_a != 0 {
        // 2,4, bst
        reg_b = reg_a & 7;

        // 1,1, bxl
        reg_b ^= 1;

        // 7,5, cdv
        reg_c = reg_a / 2usize.pow(reg_b as u32);

        // 4,6, bxc
        reg_b ^= reg_c;

        // 1,4, bxl
        reg_b ^= 4;

        // 0,3, adv
        reg_a /= 2usize.pow(3);

        // 5,5, out
        if !output.is_empty() {
            output.push(',');
        }
        output.push_str(&(reg_b & 7).to_string());

        // 3,0, jnz
    }

    println!("{output}");
}
