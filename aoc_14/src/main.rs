fn break_down(mut num: usize) -> Vec<usize> {
    if num == 0 {return vec![0];}

    let mut result = vec![];
    while num > 0 {
        result.push(num % 10);
        num = num / 10;
    }
    result.into_iter().rev().collect()
}

fn part_1() {
    let start = 793031;
    let mut recipes: Vec<usize> = vec![3,7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while recipes.len() < start + 10 {
        let x = recipes[elf_1] + recipes[elf_2];
        recipes.append(&mut break_down(x));
        elf_1 = (elf_1 + recipes[elf_1] + 1) % recipes.len();
        elf_2 = (elf_2 + recipes[elf_2] + 1) % recipes.len();
    }

    let result = recipes.into_iter().skip(start).take(10).collect::<Vec<usize>>();
    for x in result {
        print!("{}", x);
    }
    println!("");
}

fn part_2() {
    let seq = 793031;
    let seq = break_down(seq);

    let mut recipes: Vec<usize> = vec![3,7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    loop {
        let x = recipes[elf_1] + recipes[elf_2];
        let mut x = break_down(x);
        let xlen = x.len();
        recipes.append(&mut x);
        elf_1 = (elf_1 + recipes[elf_1] + 1) % recipes.len();
        elf_2 = (elf_2 + recipes[elf_2] + 1) % recipes.len();

        for i in 0..xlen {
            if recipes.len() < seq.len() + i { continue; }

            let start = recipes.len() - seq.len() - i;
            let end = start + seq.len();
            let slice = &recipes[start..end];
            if slice.iter().zip(seq.iter()).all(|(a,b)|a==b) {
                //println!("{} found match: {:?}", start, recipes);
                println!("{} found match", start);
                return;
            }
        }
    }

    println!("");
}

fn main() {
    part_1();
    part_2();
}
