use num_traits::Num;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
};

fn cut_index<T>(matrix_inner: &Vec<Vec<T>>, [input_x, input_y]: [usize; 2]) -> Vec<Vec<T>>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub + Display + Debug,
{
    let len = matrix_inner.len();
    assert!(len > 2);
    let input_x = input_x;
    let input_y = input_y;
    let mut index_y = 0;
    let mut output: Vec<Vec<T>> = vec![vec![num_traits::zero(); len - 1]; len - 1];
    for (y, line) in matrix_inner.iter().enumerate() {
        if y == input_y {
            continue;
        }
        index_y += 1;
        let mut index_x = 0;
        for (x, element) in line.iter().enumerate() {
            if x == input_x {
                continue;
            }
            index_x += 1;
            output[index_x - 1][index_y - 1] = *element;
        }
    }
    output
}

pub fn get_determinant<T>(matrix_inner: &Vec<Vec<T>>) -> T
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub + Display + Debug,
{
    let len = matrix_inner.len();
    if len == 0 {
        return num_traits::zero();
    }
    if len == 1 {
        return matrix_inner[0][0];
    }
    if len == 2 {
        return matrix_inner[0][0] * matrix_inner[1][1] - matrix_inner[0][1] * matrix_inner[1][0];
    }
    let mut output = num_traits::zero();
    for x in 0..len {
        let current = cut_index(matrix_inner, [x, 0]);
        if x % 2 == 0 {
            output += get_determinant(&current) * matrix_inner[0][x];
        } else {
            output -= get_determinant(&current) * matrix_inner[0][x];
        }
    }
    output
}
