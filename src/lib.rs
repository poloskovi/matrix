use std::ops::{Add, Mul, Sub};
use std::fmt;
use std::fmt::Display;

pub struct Matrix<T>{
    pub m: Vec<T>,
    pub nrow: usize,
    pub ncol: usize,
}

impl<T> Matrix<T>{

    /// Матрица [nrow] x [ncol], заполненная значениями по умолчанию (нулями)
    pub fn new(nrow: usize, ncol: usize) -> Matrix<T>
    where T: Default + Copy + Clone{
        Matrix {
            m: vec![T::default(); ncol*nrow],
            nrow,
            ncol,
        }
    }

    /// Возвращает значение в ячейке (row,col)
    pub fn get(&self, row:usize, col:usize) -> T
    where T: Copy{
        let index = row * self.ncol + col;
        self.m[index]
    }

    /// Устанавливает значение x в ячейку (row,col)
    pub fn set(&mut self, row:usize, col:usize, x: T) {
        let index = row * self.ncol + col;
        self.m[index] = x;
    }

    /// Вычитание
    pub fn sub(m1: &Matrix<T>, m2: &Matrix<T>) -> Matrix<T>
    where T: Default + Copy + Clone + Sub<Output=T>{
        assert_eq!(m1.nrow, m2.nrow);
        assert_eq!(m1.ncol, m2.ncol);
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for index in 0..result.m.len(){
            result.m[index] = m1.m[index] - m2.m[index];
        }
        result
    }

    /// Сложение
    pub fn add(m1: &Matrix<T>, m2: &Matrix<T>) -> Matrix<T>
    where T: Default + Copy + Clone + Add<Output=T>{
        assert_eq!(m1.nrow, m2.nrow);
        assert_eq!(m1.ncol, m2.ncol);
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for index in 0..result.m.len(){
            result.m[index] = m1.m[index] + m2.m[index];
        }
        result
    }

    /// Возвращает транспонированную матрицу
    pub fn t(&self) -> Matrix<T>
    where T: Default + Copy + Clone{
        let mut result = Matrix::new(self.ncol, self.nrow);
        for row in 0..self.nrow {
            for col in 0..self.ncol {
                result.set(col, row, self.get(row,col));
            }
        }
        result
    }

    /// Умножение
    pub fn mul(m1: &Matrix<T>, m2: &Matrix<T>) -> Matrix<T>
    where T: Add<Output=T> + Mul<Output=T> + Default + Copy + Clone{
        assert_eq!(m1.ncol, m2.nrow);
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for i in 0..m1.nrow {
            for j in 0..m2.ncol {
                let mut cij = T::default();
                for r in 0..m1.ncol {
                    cij = cij + (m1.get(i,r) * m2.get(r,j));
                }
                result.set(i,j,
                    cij);
            }
        }
        result
    }

    /// Произведение Кронекера (тензорное умножение)
    pub fn kroneker_product(m1: &Matrix<T>, m2: &Matrix<T>) -> Matrix<T>
    where T: Add<Output=T> + Mul<Output=T> + Default + Copy + Clone{
        let mut result = Matrix::new(m1.nrow*m2.nrow, m1.ncol*m2.ncol);
        for i1 in 0..m1.nrow{
            for j1 in 0..m1.ncol{
                for i2 in 0..m2.nrow{
                    for j2 in 0..m2.ncol{
                        let row = i1 * m2.nrow + i2;
                        let col = j1 * m2.ncol + j2;
                        result.set(row, col,
                            m1.get(i1, j1) * m2.get(i2, j2));
                    }
                }
            }
        }
        result
    }

    /// Количество ячеек в матрице
    pub fn count_of_cells(&self) -> usize {
        self.nrow * self.ncol
    }

    /// Преобразует вектор в матрицу [1] x [len]
    pub fn vec_to_matrix(vector: Vec<T>) -> Matrix<T>
    where T: Default + Copy + Clone{
        let mut result = Matrix::new(1, vector.len());
        for (i, value) in vector.iter().enumerate() {
            result.set(0, i, *value)
        }
        result
    }

    /// копия матрицы
    pub fn copy(&self) -> Matrix<T>
    where T: Default + Copy + Clone{
        let mut result = Matrix::new(self.nrow, self.ncol);
        for row in 0..self.nrow{
            for col in 0..self.ncol{
                result.set(row, col, self.get(row, col));
            }
        }
        result
    }

}

impl<T> fmt::Display for Matrix<T>
    where T: Display + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Default + Copy + Clone{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // для больших матриц вместо части строк и столбцов выводим "..."
        let skip_rows_after: usize = 14;
        let skip_columns_after: usize = 9;

        let skip_rows = self.nrow > skip_rows_after + 1;
        let skip_columns = self.ncol > skip_columns_after + 1;

        for row in 0..self.nrow{

            if skip_rows {
                if row > skip_rows_after + 1 && row != self.nrow-1 {
                    continue;
                }
            }

            if self.nrow == 1{
                write!(f, "⟮ ")?; //U+27EE
            } else if row == 0 {
                write!(f, "⎛ ")?; //U+239B
            } else if row == self.nrow-1 {
                write!(f, "⎝ ")?;
            } else {
                write!(f, "⎜ ")?;
            }

            for col in 0..self.ncol{
                if skip_columns {
                    if col > skip_columns_after && col != self.ncol-1 {
                        if col == skip_columns_after + 1 {
                            write!(f, "{:4} ", " ...")?;
                        }
                        continue;
                    }
                }
                if skip_rows && row == skip_rows_after + 1 {
                    write!(f, "{:4} ", " ...")?;
                }else{
                    write!(f, "{:4} ", self.get(row,col))?;
                }
            }
            if self.nrow == 1{
                writeln!(f, " ⟯")?;
            } else if row == 0 {
                writeln!(f, " ⎞")?;
            } else if row == self.nrow-1 {
                writeln!(f, " ⎠")?;
            } else {
                writeln!(f, " ⎟")?;
            }
        }
        write!(f, "")
    }
}


//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}

