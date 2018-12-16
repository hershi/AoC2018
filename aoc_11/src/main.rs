use std::iter;

fn get_coordinates(from:i32, to:i32) -> Vec<(i32,i32)> {
    (from..to+1)
        .flat_map(|x|iter::repeat(x).take((to - from) as usize + 1))
        .zip((from..to+1).cycle())
        .collect::<Vec<(i32,i32)>>()
}

#[derive(Debug)]
struct VecGrid {
    width: i32,
    height: i32,

    // First value is the coordinate ordinal to which it belongs
    // Second value is the 'generation' in which it was set
    grid: Vec<i32>,
}

impl VecGrid {
    fn new(width: i32, height: i32) -> VecGrid {
        VecGrid {
            width,
            height,
            grid: vec![std::i32::MIN; (width * height) as usize] }
    }

    fn set(&mut self, x: i32, y: i32, val: i32) {
        let index = self.calculate_index(x,y);
        self.grid[index] = val;
    }

    fn get(&self, x: i32, y: i32) -> Option<&i32> {
        if !self.in_bounds(x,y) { None } else { self.grid.get(self.calculate_index(x,y)) }
    }

    fn in_bounds(&self, x:i32, y:i32) -> bool {
        let x = x - 1;
        let y = y - 1;
        y >= 0 && y < self.height && x >= 0 && x < self.width
    }

    fn calculate_index(&self, x: i32, y: i32) -> usize {
        let x = x - 1;
        let y = y - 1;
        (self.width * y + x) as usize
    }
}

fn create_grid(from:i32, to:i32, serial: i32) -> VecGrid {
    get_coordinates(from, to)
        .iter()
        .fold(VecGrid::new(to-from+1, to-from+1), |mut acc, (x,y)| {
            let rack_id = x + 10;
            acc.set(*x,*y, (((rack_id * y)+serial) * rack_id)/100%10 - 5);
            acc
        })
}

fn create_pre_comp(from:i32, to:i32, grid: &VecGrid) -> VecGrid {
    get_coordinates(from, to)
        .iter()
        .fold(VecGrid::new(to-from+1, to-from+1), |mut acc, (x,y)| {
            let current = acc.get(x-1,*y).unwrap_or(&0)
                + acc.get(*x,y-1).unwrap_or(&0)
                - acc.get(x-1,y-1).unwrap_or(&0)
                + grid.get(*x,*y).unwrap();
            acc.set(*x, *y, current);
            acc
        })
}


fn find_max(from: i32, to: i32, size: i32, pre_comp: &VecGrid) -> ((i32,i32), i32, i32) {
    get_coordinates(from, to - size + 1)
        .iter()
        .fold(((-1,-1),std::i32::MIN,size), |max, (x,y)| {
            let val =
                 pre_comp.get(x+size-1,y+size-1).unwrap_or(&0)
                 - pre_comp.get(x-1,y+size-1).unwrap_or(&0)
                 - pre_comp.get(x+size-1,y-1).unwrap_or(&0)
                 + pre_comp.get(x-1,y-1).unwrap_or(&0);

            if val <= max.1 { max } else { ((*x,*y), val, size) }
        })
}

fn main() {
    let serial = 7803;
    let from :i32 = 1;
    let to :i32 = 300;

    let grid = create_grid(from, to, serial);
    let pre_comp = create_pre_comp(from, to, &grid);

    let res = (from..to+1)
        //.inspect(|x| println!("Size: {}", x))
        .map(|x| find_max(from, to, x, &pre_comp))
        .max_by_key(|x|x.1);

    println!("{:?}", res);
}
