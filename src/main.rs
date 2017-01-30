extern crate blas;
use blas::c::*;
fn main() {
    let x = [3.2,5.4,7.1];
    let incx = 1;
    let y = [9.8,6.4,3.5];
    let incy = 1;
    let n = x.len() as i32;
    println!("{:?}", sdot(n,&x,incx,&y,incy));
}
