use std::ops::Add;

use opis::{Bit, Int, pow, mod_inv, modulo};

// Curve 25519
// A
const A: &str = "0x0076d06";

// PRIME
const P: &str = "0x007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed";

// X25519
// Alice(256bits)
// Bob(256bits)

// ed25519 

#[derive(Clone, Debug)]
pub struct Point {pub x: Int, pub y: Int}

impl PartialEq for Point {
    fn eq(&self, b: &Self) -> bool {
        self.x == b.x && self.y == b.y
    }
}

impl Eq for Point {}

impl Point {
    pub fn double(self) -> Self {

        let two = Int::from("2");
        
        let three = Int::from("3");

        let a = Int::from(A);
        
        let p = Int::from(P);      
        
        let three_x_squared_plus_a = &(&three * &pow(&self.x, &two)) + &a;
        
        let two_y_mod_inv = mod_inv(&(&two * &self.y), &p);

        let mut slope = three_x_squared_plus_a * two_y_mod_inv;

        slope = modulo(&slope, &p);

        let mut xr = pow(&slope, &two) - (&two * &self.x);

        xr = modulo(&xr, &p);

        let mut yr = &(&slope * &(&self.x - &xr)) - &self.y;

        yr = modulo(&yr, &p);

        Point {x: xr, y: yr.inverse()}
    
    }
}

impl Add for Point {

    type Output = Self;
    
    fn add(self, q: Self) -> Self {

        if q.y == self.clone().y.inverse() {
            Point {x: Int::zero(), y: Int::zero()}
        }

        else if self == q {
            self.double()
        }

        else {

            let two = Int::from("2");

            let p = Int::from(P);

            let yq_sub_yp = &q.y - &self.y;

            let xq_sub_xp = &q.x - &self.x;

            let xq_sub_xp_mod_inv = mod_inv(&xq_sub_xp, &p);

            let mut slope = yq_sub_yp * xq_sub_xp_mod_inv;

            slope = modulo(&slope, &p);

            let mut xr = pow(&slope, &two) - (&self.x - &q.x);

            xr = modulo(&xr, &p);

            let mut yr = &(&slope * &(&self.x - &xr)) - &self.y;

            yr = modulo(&yr, &p);

            Point {x: xr, y: yr.inverse()}

        }

    }

}

impl Add for &Point {

    type Output = Point;
    
    fn add(self, b: Self) -> Point {
        self.clone() + b.clone()
    }

}

pub fn multiply(a: &str, u: &str) -> Point {

    let a_int: Int = Int::from(a);

    let point: Point = Point {
        x: Int::from(u),
        y: Int::one()
    };
    
    let mut res: Point = point.clone();

    a_int.bits
        .iter()
        .skip(2)
        .for_each(|x| {

            res = res.clone().double();
        
            if x == &Bit::One { 
                res = &res + &point
            }
    
        });

    res

}

