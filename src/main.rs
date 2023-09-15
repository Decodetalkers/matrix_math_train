#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use num_traits::Num;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

#[derive(Debug, Clone, Copy)]
struct Matrix<T, const X: usize, const Y: usize>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    inner: [[T; X]; Y],
}

impl<T, const X: usize> Matrix<T, X, X>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    fn cut_index(&self, [x, y]: [usize; 2]) -> Matrix<T, { X - 1 }, { X - 1 }>
    where
        [(); X - 1]:,
    {
        todo!()
    }
}

trait Determinant<T> {
    fn determinant(&self) -> T;
}

impl<const X: usize, T> Determinant<T> for Matrix<T, X, X>
where
    Assert<{ X > 2 }>: IsTrue,
    Matrix<T, { X - 1 }, { X - 1 }>: Determinant<T>,
    [(); X - 1]:,
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    fn determinant(&self) -> T {
        let mut output = num_traits::zero();
        let matrix = self.matrix();

        for x in 0..X {
            let current: Matrix<T, { X - 1 }, { X - 1 }> = Matrix::empty();
            if x % 2 == 0 {
                output += (current * self[[0, x]]).determinant();
            } else {
                output -= (current * self[[0, x]]).determinant();
            }
        }
        output
    }
}

impl<T> Determinant<T> for Matrix<T, 0, 0>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    fn determinant(&self) -> T {
        num_traits::zero()
    }
}

impl<T> Determinant<T> for Matrix<T, 1, 1>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    fn determinant(&self) -> T {
        self[[0, 0]]
    }
}

impl<T> Determinant<T> for Matrix<T, 2, 2>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    fn determinant(&self) -> T {
        self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]]
    }
}

impl<T, const X: usize, const Y: usize> Matrix<T, X, Y>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    fn empty() -> Self {
        Matrix {
            inner: [[Default::default(); X]; Y],
        }
    }

    fn new(vector: [[T; X]; Y]) -> Self {
        Matrix { inner: vector }
    }

    #[allow(unused)]
    fn matrix(&self) -> &[[T; X]; Y] {
        &self.inner
    }

    fn get_matrix(&mut self) -> &mut [[T; X]; Y] {
        &mut self.inner
    }
}

impl<T, const X: usize, const Y: usize> Mul<T> for Matrix<T, X, Y>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    type Output = Matrix<T, X, Y>;
    fn mul(self, rhs: T) -> Self::Output {
        let mut output = self.clone();
        let matrix = output.get_matrix();
        for x in 0..X {
            for y in 0..Y {
                matrix[y][x] = self.inner[y][x] * rhs;
            }
        }
        output
    }
}

impl<T, const X: usize, const Y: usize> Add<Matrix<T, X, Y>> for Matrix<T, X, Y>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    type Output = Matrix<T, X, Y>;
    fn add(self, rhs: Matrix<T, X, Y>) -> Self::Output {
        let mut output = self.clone();
        let matrix = output.get_matrix();
        for x in 0..X {
            for y in 0..Y {
                matrix[y][x] = self.inner[y][x] + rhs.inner[y][x];
            }
        }
        output
    }
}

impl<T, const X: usize, const Y: usize> Sub<Matrix<T, X, Y>> for Matrix<T, X, Y>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    type Output = Matrix<T, X, Y>;
    fn sub(self, rhs: Matrix<T, X, Y>) -> Self::Output {
        let mut output = self.clone();
        let matrix = output.get_matrix();
        for x in 0..X {
            for y in 0..Y {
                matrix[y][x] = self.inner[y][x] - rhs.inner[y][x];
            }
        }
        output
    }
}

impl<T, const X1: usize, const Y1: usize, const X2: usize> Mul<Matrix<T, X2, X1>>
    for Matrix<T, X1, Y1>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    type Output = Matrix<T, X2, Y1>;
    fn mul(self, rhs: Matrix<T, X2, X1>) -> Self::Output {
        let mut output = Matrix::empty();
        let matrix_inner = output.get_matrix();
        for y in 0..Y1 {
            for x in 0..X2 {
                let mut element: T = Default::default();
                for x0 in 0..X1 {
                    element += self.inner[y][x0] * rhs.inner[x0][x];
                }
                matrix_inner[y][x] = element;
            }
        }
        output
    }
}

impl<T, const X: usize, const Y: usize> Index<[usize; 2]> for Matrix<T, X, Y>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    type Output = T;
    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        assert!(x != 0 && y != 0);
        assert!(y < y + 1);
        assert!(x < x + 1);
        &self.inner[x - 1][y - 1]
    }
}

impl<T, const X: usize, const Y: usize> IndexMut<[usize; 2]> for Matrix<T, X, Y>
where
    T: Num + Default + Copy + AddAssign + SubAssign + Add + Sub,
{
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        assert!(x != 0 && y != 0);
        assert!(y < y + 1);
        assert!(x < x + 1);
        &mut self.inner[x - 1][y - 1]
    }
}

fn main() {
    let a: Matrix<i32, 3, 2> = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    let mut b: Matrix<i32, 2, 3> = Matrix::new([[10, 5], [20, 6], [30, 7]]);
    let b2: Matrix<i32, 2, 3> = Matrix::new([[10, 5], [20, 6], [30, 7]]);
    println!("{}", a[[1, 2]]);
    println!("{:?}", b + b2);
    println!("{:?}", b - b2);
    println!("{}", b[[1, 2]]);
    b[[1, 1]] = 100;
    println!("{}", b[[1, 1]]);
    println!("{:?}", b * 20);
    println!("{:?}", b);

    println!("{:?}", a * b);
}
