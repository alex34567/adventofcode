use std::io::prelude::*;
use std::fs::File;
use std::mem;
use std::ops::{Index, IndexMut};

struct DoubleVec<T> {
    forward: Vec<T>,
    backward: Vec<T>,
}

impl<T> DoubleVec<T> {
    fn new() -> Self {
        DoubleVec {
            forward: Vec::new(),
            backward: Vec::new(),
        }
    }
    fn forward_push(&mut self, value: T) {
        self.forward.push(value);
    }
    fn backward_push(&mut self, value: T) {
        self.backward.push(value);
    }
    fn forward_len(&self) -> usize {
        self.forward.len()
    }
    fn backward_len(&self) -> usize {
        self.backward.len()
    }
}

impl<T> Index<isize> for DoubleVec<T> {
    type Output = T;
    fn index(&self, index: isize) -> &T {
        if index < 0 {
            let back_index = index * -1 - 1;
            &self.backward[back_index as usize]
        } else {
            &self.forward[index as usize]
        }
    }
}

impl<T> IndexMut<isize> for DoubleVec<T> {
    fn index_mut(&mut self, index: isize) -> &mut T {
        if index < 0 {
            let back_index = index * -1 - 1;
            &mut self.backward[back_index as usize]
        } else {
            &mut self.forward[index as usize]
        }
    }
}

fn push_0_all_right(vec: &mut DoubleVec<DoubleVec<u8>>) {
    for x in 0..vec.backward_len() {
        vec[x as isize * -1 - 1].forward_push(0)
    }
    for x in 0..vec.forward_len() {
        vec[x as isize].forward_push(0)
    }
}

fn push_0_all_left(vec: &mut DoubleVec<DoubleVec<u8>>) {
    for x in 0..vec.backward_len() {
        vec[x as isize * -1 - 1].backward_push(0)
    }
    for x in 0..vec.forward_len() {
        vec[x as isize].backward_push(0)
    }
}

fn push_0_all_up(vec: &mut DoubleVec<DoubleVec<u8>>) {
    vec.forward_push(DoubleVec::new());
    for x in 0..vec[0].backward_len() {
        let len = vec.forward_len();
        vec[len as isize - 1].backward_push(0)
    }
    for x in 0..vec[0].forward_len() {
        let len = vec.forward_len();
        vec[len as isize - 1].forward_push(0)
    }
}

fn push_0_all_down(vec: &mut DoubleVec<DoubleVec<u8>>) {
    vec.backward_push(DoubleVec::new());
    for x in 0..vec[0].backward_len() {
        let len = vec.backward_len();
        vec[len as isize * -1].backward_push(0)
    }
    for x in 0..vec[0].forward_len() {
        let len = vec.backward_len();
        vec[len as isize * -1].forward_push(0)
    }
}

fn main() {
    let mut grid: DoubleVec<DoubleVec<u8>> = DoubleVec::new();
    grid.forward_push(DoubleVec::new());
    grid[0].forward_push(1);
    let mut current_x = 0;
    let mut current_y = 0;
    let mut back_x = 0;
    let mut back_y = 0;
    let mut houses = 1;
    let input_file = File::open("input.txt").unwrap();
    for wraped in input_file.bytes() {
        let x = wraped.unwrap();
        if x == b'>' {
            current_x += 1;
            if current_x == grid[0].forward_len() as isize {
                push_0_all_right(&mut grid);
                grid[current_y][current_x] = 1;
                houses += 1;
            } else if grid[current_y][current_x] == 0 {
                grid[current_y][current_x] = 1;
                houses += 1;
            }
        } else if x == b'<' {
            current_x -= 1;
            if current_x == grid[0].backward_len() as isize * -1 - 1 {
                push_0_all_left(&mut grid);
                grid[current_y][current_x] = 1;
                houses += 1;
            } else if grid[current_y][current_x] == 0 {
                grid[current_y][current_x] = 1;
                houses += 1;
            }
        } else if x == b'^' {
            current_y += 1;
            if current_y == grid.forward_len() as isize {
                push_0_all_up(&mut grid);
                grid[current_y][current_x] = 1;
                houses += 1;
            } else if grid[current_y][current_x] == 0 {
                grid[current_y][current_x] = 1;
                houses += 1;
            }
        } else if x == b'v' {
            current_y -= 1;
            if current_y == grid.backward_len() as isize * -1 - 1 {
                push_0_all_down(&mut grid);
                grid[current_y][current_x] = 1;
                houses += 1;
            } else if grid[current_y][current_x] == 0 {
                grid[current_y][current_x] = 1;
                houses += 1;
            }
        }
        mem::swap(&mut current_x, &mut back_x);
        mem::swap(&mut current_y, &mut back_y);
    }
    println!("{}", houses);
}
