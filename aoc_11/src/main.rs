use std::iter;
use std::collections::HashMap;

fn get_coordinates(from:i32, to:i32) -> Vec<(i32,i32)> {
    (from..to+1)
        .flat_map(|x|iter::repeat(x).take((to - from) as usize + 1))
        .zip((from..to+1).cycle())
        .collect::<Vec<(i32,i32)>>()
}

type Grid = HashMap<(i32, i32), i32>;

fn create_grid(from:i32, to:i32, serial: i32) -> Grid {
    get_coordinates(from, to)
        .iter()
        .fold(HashMap::new(), |mut acc, (x,y)| {
            let rack_id = x + 10;
            acc.insert((*x,*y), (((rack_id * y)+serial) * rack_id)/100%10 - 5);
            acc
        })
}

fn create_pre_comp(from:i32, to:i32, grid: &Grid) -> Grid {
    let mut pre_comp = Grid::new();

    get_coordinates(from, to)
        .iter()
        .for_each(|(x,y)|{
            let current = pre_comp.get(&(x-1,*y)).unwrap_or(&0)
                + pre_comp.get(&(*x,y-1)).unwrap_or(&0)
                - pre_comp.get(&(x-1,y-1)).unwrap_or(&0)
                + grid.get(&(*x,*y)).unwrap();
            pre_comp.insert((*x,*y),current);
        });

    pre_comp
}


fn find_max(from: i32, to: i32, size: i32, pre_comp: &Grid) -> ((i32,i32), i32, i32) {
    get_coordinates(from, to - size + 1)
        .iter()
        .fold(((-1,-1),std::i32::MIN,size), |max, (x,y)| {
            let val =
                 pre_comp.get(&(x+size-1,y+size-1)).unwrap()
                 - pre_comp.get(&(x-1,y+size-1)).unwrap_or(&0)
                 - pre_comp.get(&(x+size-1,y-1)).unwrap_or(&0)
                 + pre_comp.get(&(x-1,y-1)).unwrap_or(&0);

            if val <= max.1 { max } else { ((*x,*y), val, size) }
        })
}

fn main() {
    let serial = 7803;
    let from :i32 = 1;
    let to :i32 = 300;
    let grid = create_grid(from, to, serial);
    let pre_comp = create_pre_comp(from, to, &grid);

    //let res = (1..to+1)
    let res = (from..to+1)
        .inspect(|x| println!("Size: {}", x))
        .map(|x| find_max(from, to, x, &pre_comp))
        .max_by_key(|x|x.1);

    println!("{:?}", res);
}
