use nalgebra::Vector3;

#[allow(dead_code)]
pub struct Ray {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>
} 

#[allow(dead_code)]
impl Ray {
    pub fn new(a: Vector3<f64>, b: Vector3<f64>) -> Self {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vector3<f64> { 
        self.a 
    }

    pub fn direction(&self) -> Vector3<f64> { 
        self.b 
    }

    pub fn at(&self, t: f64) -> Vector3<f64> { 
        self.a + t * self.b 
    }
}