//extern crate neuronet;

use matrix::{Matrix};

#[test]
fn matrix_t(){
    let mut m = Matrix::new(2,3);
    m.set(0,0,1);
    m.set(0,1,2);
    m.set(0,2,3);
    let mt = m.t();
    assert_eq!(mt.get(0,0),1);
    assert_eq!(mt.get(1,0),2);
    assert_eq!(mt.get(2,0),3);
}

#[test]
fn matrix_mul(){
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
    assert_eq!(m3.const_mult,Some(6));
    assert_eq!(m3.get(0,0),19);
    assert_eq!(m3.get(0,1),22);
    assert_eq!(m3.get(1,0),43);
    assert_eq!(m3.get(1,1),50);
}

#[test]
fn matrix_kroneker_product(){
    let mut m1 = Matrix::new(2,2);
    m1.set(0,0,1);
    m1.set(0,1,2);
    m1.set(1,0,3);
    m1.set(1,1,4);
    let mut m2 = Matrix::new(2,2);
    m2.set(0,0,0);
    m2.set(0,1,5);
    m2.set(1,0,6);
    m2.set(1,1,7);
    let m3 = Matrix::kroneker_product(&m1,&m2);
    assert_eq!(m3.nrow,4);
    assert_eq!(m3.ncol,4);
    assert_eq!(m3.get(0,0),0);
    assert_eq!(m3.get(0,1),5);
    assert_eq!(m3.get(0,2),0);
    assert_eq!(m3.get(0,3),10);
    assert_eq!(m3.get(1,0),6);
    assert_eq!(m3.get(1,1),7);
    assert_eq!(m3.get(1,2),12);
    assert_eq!(m3.get(1,3),14);
    assert_eq!(m3.get(2,0),0);
    assert_eq!(m3.get(2,1),15);
    assert_eq!(m3.get(2,2),0);
    assert_eq!(m3.get(2,3),20);
    assert_eq!(m3.get(3,0),18);
    assert_eq!(m3.get(3,1),21);
    assert_eq!(m3.get(3,2),24);
    assert_eq!(m3.get(3,3),28);
}

#[test]
fn matrix_copy(){

    let mut m1 = Matrix::new(2,2);
    m1.set(0,0,1);
    m1.set(0,1,2);
    m1.set(1,0,3);
    m1.set(1,1,4);

    let m2 = m1.copy();
    assert_eq!(m2.nrow, m1.nrow);
    assert_eq!(m2.ncol ,m1.ncol);
    assert_eq!(m2.get(0,0), 1);
    assert_eq!(m2.get(0,1), 2);
    assert_eq!(m2.get(1,0), 3);
    assert_eq!(m2.get(1,1), 4);
}

