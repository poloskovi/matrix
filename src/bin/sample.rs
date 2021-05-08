extern crate matrix;
use matrix::Matrix;

fn test_print() {

    println!("Пример: вывод на экран");

    let mut m1 = Matrix::new(2,2);
    m1.set(0,0,1.2);
    m1.set(0,1,2.3);
    m1.set(1,0,3.4);
    m1.set(1,1,4.5);

    println!("{}", m1);

    m1.const_mult = Some(1.234);

    println!("{}", m1);

}

fn test_mul() {

    println!("Пример: умножение");
    let mut m1 = Matrix::new(2,2);
    m1.const_mult = Some(2);
    m1.set(0,0,1);
    m1.set(0,1,2);
    m1.set(1,0,3);
    m1.set(1,1,4);
    let mut m2 = Matrix::new(2,2);
    m2.const_mult = Some(3);
    m2.set(0,0,5);
    m2.set(0,1,6);
    m2.set(1,0,7);
    m2.set(1,1,8);
    let m3 = Matrix::mul(&m1,&m2);
    println!("{}", m1);
    println!("X");
    println!("{}", m2);
    println!("=");
    println!("{}", m3);

}

fn main() {
    
    test_print();
    test_mul();
    
}
