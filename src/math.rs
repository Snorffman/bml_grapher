// use std::ops::{Sub,Add,Mul,Div};
// pub trait VectorElement : Sized+Clone+Copy {}

use std::ops::{Add, Mul, Sub};

//~~~~~~~~~~~~~~~~~ .Math
/// A vector with two dimensions.
#[derive(Clone, Debug)] // #[derive(Clone, Copy)]
pub struct Vec2D<T>  {
    pub x: T,
    pub y: T,
}
impl<T> Vec2D<T> {  // u can have non-f32 types, but only for accessing values.
    pub fn new(_x: T, _y: T) -> Self {
        Vec2D { x: _x, y: _y }
    }
}
impl<T: Copy+ Sub<Output=T> + Add<Output=T> + Mul<Output=T> > Vec2D<T> {
    pub fn sub_vec(&self, v2: &Vec2D<T>) -> Vec2D<T> {
        Vec2D::new(self.x-v2.x, self.y-v2.y)
    }
    pub fn add_vec(&self, v2: &Vec2D<T>) -> Vec2D<T> {
        Vec2D::new(self.x+v2.x, self.y+v2.y)
    }
    pub fn mult_scalar(&self, s: T) -> Vec2D<T> {
        Vec2D::new(s*self.x, s*self.y)
    }

    pub fn magnitude(&self) -> T {
        self.x*self.x + self.y*self.y
    }

    pub fn distance(&self, v2: &Vec2D<T>) -> T {
        self.sub_vec(v2).magnitude()
    }
    // Dot product
    pub fn dot(&self, v2: &Vec2D<T>) -> T {
        self.x * v2.x + self.y*v2.y
    }
}
impl Vec2D<f32> {  // Bonus functions for real numbers
    pub fn normalize(&self) -> Vec2D<f32> {
        let len = (self.x*self.x + self.y*self.y).sqrt();
        let len = if len == 0.0 {1.0} else {len}; // Prevent division by 0
        Vec2D::new(self.x/len, self.y/len)
    }
    pub fn with_magnitude(&self, magnitude: f32) -> Vec2D<f32> {
        let len = (self.x*self.x + self.y*self.y).sqrt();
        Vec2D::new(self.x*magnitude / len, self.y*magnitude / len)
    }
    
    pub fn rotate(&self, t: f32) -> Vec2D<f32>{
        Vec2D::new( 
            f32::cos(t)*self.x - f32::sin(t)*self.y, 
            f32::sin(t)*self.x + f32::cos(t)*self.y
        )
    } 

    pub fn usize(&self) -> Vec2D<usize> {
        Vec2D::new(self.x as usize, self.y as usize)
    }
    
}









impl<T: Copy> From<[T; 2]> for Vec2D<T> {
    fn from(item: [T;2]) -> Self {
        Vec2D { x: item[0], y: item[1] }
    }
}
impl<T: Copy> From<Vec<T>> for Vec2D<T> {
    fn from(item: Vec<T>) -> Self {
        Vec2D { x: item[0], y: item[1] }
    }
}
// impl Add for Vec2D<f32> {
//     type Output = Self;

//     fn add(self, rhs: Vec2D<f32>) -> Self::Output {
//         Vec2D::add_vec(&self, &rhs)
//     }
// }
// impl Sub for Vec2D<f32> {
//     type Output = Self;

//     fn sub(self, rhs: Vec2D<f32>) -> Self::Output {
//         Vec2D::sub_vec(&self, &rhs)
//     }
// }


pub fn lerp(a: f32,b:f32 ,t: f32) -> f32  {
    a + (b-a)*t
}

// c= a + t(b-a)
pub fn invLerp(a: f32, b: f32, c: f32) -> f32 {
    (c-a)/(b-a)
}












#[derive(Debug, Clone)]
pub struct Segment {
    pub start: Vec2D<f32>,
    pub end: Vec2D<f32>,
}
impl Segment {
    pub fn new(start: &Vec2D<f32>, end: &Vec2D<f32>) -> Self {
        Segment { start: Vec2D::new(start.x, start.y), end: Vec2D::new(end.x, end.y)}
    }

    // bounds_check
    // Checks whether a point is within the boundaries of the segment
    pub fn bounds_check(&self, point: &Vec2D<f32>) -> bool {
        let x_t = invLerp(self.start.x, self.end.x, point.x);
        let y_t = invLerp(self.start.y, self.end.y, point.y);
        
        (x_t >= 0.0 && x_t <= 1.0) && (y_t >= 0.0 && y_t <= 1.0)
    }

    // If there is an intersection it returns the t values, 
    // Otherwise (if lines are parallel) it returns None
    pub fn get_t(&self, other: &Segment) -> Option<(f32,f32)> {
        let l1 = Segment::to_line(&self);
        let l2 = Segment::to_line(other);

        
        // If intersection exists, make sure it is within bounds
        if let Some(point) = l1.intersect(&l2) {
            let x_t = invLerp(self.start.x, self.end.x, point.x);
            let y_t = invLerp(self.start.y, self.end.y, point.y);

            if self.bounds_check(&point) && other.bounds_check(&point) {
                Some((x_t, y_t))
            }else {
                None
            }
        }else {
            return None;
        }
    }



    pub fn to_line(&self) -> Line {
        // r= a + tb
        let a = &self.start;
        let b = self.end.sub_vec(&self.start);

        Line::new(-b.y, b.x, a.y*b.x - a.x*b.y)
    }


}



pub struct Line {  
    a : f32, b:f32, c: f32,
}
impl Line {
    pub fn new(a:f32, b:f32, c:f32) -> Self {
        Line{a,b,c}
    }

    // a1 x + b1 y = c1
    // a2 x + b2 y = c2
    pub fn intersect(&self, other: &Line) -> Option<Vec2D<f32>> {
        let m = Matrix::new(vec![
            vec![self.a, self.b],
            vec![other.a, other.b],
        ]);
        let c = vec![self.c, other.c];

        if let Some(solution) = m.cramers_solve(&c) {
            return Some(Vec2D::new(solution[0], solution[1]))
        }else {
            None
        }
    }
}


// A matrix struct. Atm it isn't very efficient cos it doesn't use parallelism, avoids mutability, etc.
// But the idea is there for coolness.
pub struct Matrix {
    arr: Vec<Vec<f32>>
}
impl Matrix {
    pub fn new(arr: Vec<Vec<f32>>) -> Self {
        Matrix { arr }
    }  

    // i : column number to remove
    pub fn remove_col(&self, i: usize)  -> Matrix {
        let m = self.arr[0].len();
        assert!( i < m );
        
        let arr: Vec<Vec<f32>> = self.arr.iter().map(
            |row| { 
                let mut new_row: Vec<f32> = Vec::with_capacity(m-1);
                for col_idx in 0..m {
                    if col_idx != i {
                        new_row.push(row[col_idx]);
                    }
                }
                new_row
            }
        ).collect();
        Matrix::new(arr)
    }
    // i : row number to remove
    pub fn remove_row(&self, i: usize) -> Matrix {
        let m = self.arr.len();
        assert!(i < m );

        let mut new_row: Vec<Vec<f32>> = Vec::with_capacity(m-1);
        for row_idx in 0..m {
            if row_idx != i {
                new_row.push(self.arr[row_idx].clone() );
            }
        }
        Matrix::new(new_row)
    }
    // Replaces collumn to the matrix, in the location i
    pub fn replace_col(&self, i: usize, vec: &Vec<f32>) -> Matrix {
        assert!(vec.len() == self.arr.len());
        let n:usize = self.arr.len(); let m: usize = self.arr[0].len();
        let mut new_arr: Vec<Vec<f32>> = vec![vec![0. ; m] ; n];

        for row in 0..n{
            for col in 0..m {
                if col == i {
                    new_arr[row][col] = vec[row];
                }else {
                    new_arr[row][col] = self.arr[row][col];
                }
            }
        }

        Matrix::new(new_arr)
    }

    // Gets the determinant of the matrix (recursive approach)
    // Assumes: Matrix is square
    pub fn det(&self) -> f32 {
        let mut determinant = 0.0;
        let n:usize = self.arr.len(); let m:usize = self.arr[0].len();
        let row_num = 0;  // we'll do it from the first row
        //? Base case: 1x1 matrix
        assert!(n > 0 && m > 0);
        if n == 1 { return self.arr[0][0]  }

        for i in 0..self.arr.len() {
            // 1 if even, -1 otherwise.
            let sign:f32 = if i % 2 == 0 {1.0} else {-1.0};
            let e: f32 = self.arr[row_num][i];

            let mat_ij: Matrix = self.remove_col(i).remove_row(row_num);

            determinant += sign* e * mat_ij.det();
        }
        determinant
    }


    // Multiplies the matrix by a scalar
    pub fn mult_scalar(&self, v: f32) -> Matrix  {
        let arr: Vec<Vec<f32>>= self.arr.iter()
            .map(|row| row.iter().map(|val|  val*v).collect())
            .collect();
        Matrix::new(arr)
    }
    pub fn mut_mult_scalar(&mut self, v: f32) {
        for row in &mut self.arr {
            for element in row {
                *element = *element * v;
            }
        }
    }

    // Uses cramers rule to solve for x in:  Mx = C
    // Assumes:
    // 1) this is a square matrix, and x,c are the correct size.
    // 2) This matrix is inversible : |M| /= 0 
    pub fn cramers_solve(&self, c: &Vec<f32>) -> Option<Vec<f32>> {
        let n:usize = self.arr.len(); let m:usize = self.arr[0].len();
        let correct_dimentions:bool = n== m && n == c.len();
        if !correct_dimentions {return None;}

        let det_m = self.det();
        if det_m == 0.0 { return None; } // Fail case: |M| == 0 

        let mut solution:Vec<f32> = vec![0.; n];

        for j in 0..m {
            let det_c = self.replace_col(j, c, ).det();

            solution[j] = det_c / det_m;
        }

        Some(solution)
    }
}