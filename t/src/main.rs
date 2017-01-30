extern crate blas_sys;
extern crate libc;
use blas_sys::c;
use libc::c_int;
use libc::c_float;
fn main() {
    let x:[c_float;3] = [3.2,5.4,7.1];
    let xp: *const c_float = &x[0];
    let incx: c_int = 1;
    let y:[c_float;3] = [9.8,6.4,3.5];
    let yp: *const c_float = &y[0];
    let incy: c_int = 1;
    let n: c_int = x.len() as i32;
    unsafe {
        println!("{:?}",c::cblas_sdot(n,xp,incx,yp,incy));
    }
}
