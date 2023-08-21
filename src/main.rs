use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
struct Matrix<T, const X: usize, const Y: usize>
where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SubAssign
        + AddAssign
        + Default
        + Copy,
{
    inner: [[T; X]; Y],
}

impl<T, const X: usize, const Y: usize> Matrix<T, X, Y>
where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SubAssign
        + AddAssign
        + Default
        + Copy,
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
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SubAssign
        + AddAssign
        + Default
        + Copy,
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
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SubAssign
        + AddAssign
        + Default
        + Copy,
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

impl<T, const X1: usize, const Y1: usize, const X2: usize> Mul<Matrix<T, X2, X1>>
    for Matrix<T, X1, Y1>
where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SubAssign
        + AddAssign
        + Default
        + Copy,
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
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SubAssign
        + AddAssign
        + Default
        + Copy,
{
    type Output = T;
    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        assert!(x < X);
        assert!(y < Y);
        &self.inner[x][y]
    }
}

impl<T, const X: usize, const Y: usize> IndexMut<[usize; 2]> for Matrix<T, X, Y>
where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + SubAssign
        + AddAssign
        + Default
        + Copy,
{
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        assert!(x < X);
        assert!(y < Y);
        &mut self.inner[x][y]
    }
}

fn main() {
    let a: Matrix<i32, 3, 2> = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    let mut b: Matrix<i32, 2, 3> = Matrix::new([[10, 5], [20, 6], [30, 7]]);
    let b2: Matrix<i32, 2, 3> = Matrix::new([[10, 5], [20, 6], [30, 7]]);

    println!("{:?}", b + b2);
    println!("{}", b[[0, 1]]);
    b[[0, 0]] = 100;
    println!("{}", b[[0, 0]]);
    println!("{:?}", b * 20);
    println!("{:?}", b);

    println!("{:?}", a * b);
}
