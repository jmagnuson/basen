

use std::ops::Add;
use std::ops::Sub;
//use std::ops::Mul;
use std::cmp;

#[derive(Debug, Eq)]
pub struct BaseN {
    pub base: usize,
    pub vec: Vec<u8>
}

impl BaseN {

    /// Creates new BaseN 
    pub fn new(base: usize) -> BaseN/*<T>*/ {
        BaseN { base: base, vec: Vec::new() }
    }

    /// Creates new BaseN with initial vector capacity
    pub fn with_capacity(base: usize, capacity: usize) -> BaseN {
        BaseN { base: base, vec: Vec::with_capacity(capacity) }
    }

    /// Creates new BaseN with existing base and vec
    pub fn with_existing(base: usize, vec: Vec<u8>) -> BaseN {
        BaseN { base: base, vec: vec }
    }

    /// Creates a new BaseN from usize value
    pub fn from_usize(base: usize, val: usize) -> Result<BaseN, &'static str> {

        let mut new_vec:Vec<u8> = Vec::new();
        let mut val10: usize = val;
        loop {
            let val = val10 / base;
            match val {
                0 => {
                    new_vec.push(val10 as u8);
                    break;
                }
                _ => {
                    let remainder = val10 % base;
                    //println!("{}", remainder);
                    new_vec.push(remainder as u8);
                    val10 = val;
                }
            }
        }
        let new_basen: BaseN = BaseN { base: base, vec: new_vec };

        Ok(new_basen)
    }

    /// Converts existing mutable BaseN to new base
    pub fn to_base_mut(&mut self, new_base: usize) -> Result<bool, &'static str> {

        if self.base == new_base {
            return Ok(true);
        }

        let mut val10: usize = self.to_usize().unwrap();

        loop {
            let val = val10 / new_base;
            match val {
                0 => {
                    self.vec.push(val10 as u8);
                    break;
                }
                _ => {
                    let remainder = val10 % new_base;
                    self.vec.push(remainder as u8);
                    val10 = val;
                }
            }
        }

        Ok(true)
    }


    /// Converts to a new BaseN copy
    pub fn to_base(&self, new_base: usize) -> Result<BaseN, &'static str> {

        //TODO: If base is same, just return existing??
        if self.base == new_base {
            return Ok(self.clone());
        }

        // TODO: Create copy, convert mutable, or both?
        //let new_basen = BaseN::new(new_base);

        //println!("converting base {} to base {}", self.base, new_base);
        let val10: usize = self.to_usize().unwrap();
        //println!("{}", val10);

        BaseN::from_usize(new_base, val10)
        
    }

    /// Converts existing BaseN to a usize
    pub fn to_usize(&self) -> Result<usize, &'static str> {
        let mut val10: usize = 0;
        for (i, x) in self.vec.iter().enumerate() {
            val10 = val10 + (*x as usize) * self.base.pow(i as u32);
        }
        Ok(val10)
    }

}

//impl Copy for BaseN { }

impl PartialEq for BaseN {
    fn eq(&self, other: &BaseN) -> bool {
        if self.base == other.base {
            (self.vec == other.vec)
        } else {
            self.vec == other.to_base(self.base).unwrap().vec

        }
    }
}

impl Clone for BaseN {
    fn clone(&self) -> Self {
        BaseN { base: self.base, vec: self.vec.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vec.clone_from(&source.vec);
        self.base = source.base;
    }
}

impl Add for BaseN {
    type Output = Result<BaseN, &'static str>;

    // Option<> because might overflow, bases incorrect, etc.
    // I suppose you could add a base16 to base10 with an internal convert
    fn add(self, rhs: BaseN) -> Result<BaseN, &'static str> {

        let add_basen = if rhs.base==self.base {
            rhs
        } else { 
            rhs.to_base(self.base).unwrap()
            // TODO: Throw error?  Don't actually know 
            // what base user wanted to end with
        };

        // TODO: Use zip() w/ carry array instead?
        let max = cmp::max(self.vec.len(), add_basen.vec.len());
        let mut new_vec: Vec<u8> = Vec::with_capacity(max);

        let mut carry: u8 = 0;

        for i in 0..max {
            match (self.vec.get(i), add_basen.vec.get(i)) {
                (Some(x), Some(y)) => {
                    let sum: u8 = *x + *y + carry;
                    carry = sum / (add_basen.base as u8);
                    new_vec.push(sum % (add_basen.base as u8));
                },
                (Some(x), None) => {
                    // no carry, but need to resize
                    let sum: u8 = *x + carry;
                    carry = sum / (add_basen.base as u8);
                    new_vec.push(sum % (add_basen.base as u8));
                },
                (_, _) => break // no ops needed

            }
        }

        let new_basen: BaseN = BaseN { base: add_basen.base, vec: new_vec };

        Ok(new_basen)
    }
}

impl Sub for BaseN {
    type Output = Result<BaseN, &'static str>;

    // Option<> because might overflow, bases incorrect, etc.
    // I suppose you could add a base16 to base10 with an internal convert
    fn sub(self, rhs: BaseN) -> Result<BaseN, &'static str> {

        let sub_basen = if rhs.base==self.base {
            rhs
        } else { 
            rhs.to_base(self.base).unwrap()
            // TODO: Throw error?  Don't actually know 
            // what base user wanted to end with
        };

        let max = cmp::max(self.vec.len(), sub_basen.vec.len());
        let mut new_vec: Vec<u8> = Vec::with_capacity(max);

        let mut borrow: u8 = 0;

        for i in 0..max {
            match (self.vec.get(i), sub_basen.vec.get(i)) {
                (Some(x), Some(y)) => {
                    let mut diff: i8 = (*x as i8) - (*y as i8) - (borrow as i8);
                    borrow = if diff < 0 { 
                        diff += self.base as i8; 1 } else { 0 };
                    new_vec.push(diff as u8);
                },
                (Some(x), None) => {
                    let mut diff: i8 = (*x as i8) - (borrow as i8);
                    borrow = if diff < 0 { 
                        diff += self.base as i8; 1 } else { 0 };
                    new_vec.push(diff as u8);
                },
                (_, _) => break // no ops needed

            }
        }

        let new_basen: BaseN = BaseN { base: sub_basen.base, vec: new_vec };

        Ok(new_basen)
    }
}

