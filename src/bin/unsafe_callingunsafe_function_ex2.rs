use std::ops::{Add, Mul, Sub};

struct VarManipulator<T>(*mut T)
where 
    T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T>;
impl <T> VarManipulator<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    fn new(ptr: * mut T) -> Self {
        Self(ptr)
    }
    unsafe fn add(&self, operand2: T) {
        *self.0 = *self.0 + operand2;
    }
    unsafe fn mul(&self, operand2: T) {
        *self.0 = *self.0 * operand2;
    }
    unsafe fn sub(&self, operand2: T) {
        *self.0 = *self.0 - operand2;
    }
    unsafe fn get_val(&self) ->T {
        *self.0
    }
    
}
fn main() {
    let mut x =20;
    let manupulator = VarManipulator::new(&mut x);
    unsafe {
        manupulator.sub(10);
        manupulator.mul(9);
        manupulator.add(10);
        assert_eq!(manupulator.get_val(), 100);
        assert_eq!(x, manupulator.get_val());
        
    }
}