// use std::ops::{Sub,Add,Mul,Div};
// pub trait VectorElement : Sized+Clone+Copy {}

use std::ops::{Add, Mul, Sub};

//~~~~~~~~~~~~~~~~~ .Math
/// A vector with two dimensions.
/// 
/// ```
/// let vec = [1,2].into()
/// ```
#[derive(Clone)] // #[derive(Clone, Copy)]
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

    pub fn magnitude(&self) -> T {
        self.x*self.x + self.y*self.y
    }

    pub fn distance(&self, v2: &Vec2D<T>) -> T {
        self.sub_vec(v2).magnitude()
    }
}
impl Vec2D<f32> {  // Bonus functions for real numbers
    pub fn normalize(&self) -> Vec2D<f32> {
        let len = (self.x*self.x + self.y*self.y).sqrt();
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

    pub fn to_usize_vec(&self) -> Vec2D<usize> {
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