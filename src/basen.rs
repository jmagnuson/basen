

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp;
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
pub struct BaseN {
    pub base: usize,
    pub vec: Vec<u8>
}

impl BaseN {

    /// Creates new number given a base.
    pub fn new(base: usize) -> BaseN {
        BaseN { base: base, vec: Vec::new() }
    }

    /// Creates new number with initial vector capacity.
    pub fn with_capacity(base: usize, capacity: usize) -> BaseN {
        BaseN { base: base, vec: Vec::with_capacity(capacity) }
    }

    /// Creates new number with existing base and vec.
    pub fn with_existing(base: usize, vec: Vec<u8>) -> BaseN {
        BaseN { base: base, vec: vec }
    }

    /// Creates a new number from base and usize value.
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
                    new_vec.push(remainder as u8);
                    val10 = val;
                }
            }
        }
        let new_basen: BaseN = BaseN { base: base, vec: new_vec };

        Ok(new_basen)
    }

    /// Converts existing number to new base.
    pub fn set_base(&mut self, new_base: usize) -> Result<bool, &'static str> {

        if self.base == new_base {
            return Ok(true);
        }

        let mut val10: usize = self.to_usize().unwrap();

        self.base = new_base;
        self.vec.clear();

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


    /// Converts to a new base copy.
    pub fn to_base(&self, new_base: usize) -> Result<BaseN, &'static str> {

        //TODO: If base is same, just return existing??
        if self.base == new_base {
            return Ok(self.clone());
        }

        // TODO: Create copy, convert mutable, or both?

        let val10: usize = self.to_usize().unwrap();

        BaseN::from_usize(new_base, val10)
        
    }

    /// Converts existing BaseN to a usize.
    pub fn to_usize(&self) -> Result<usize, &'static str> {
        let mut val10: usize = 0;
        for (i, x) in self.vec.iter().enumerate() {
            val10 = val10 + (*x as usize) * self.base.pow(i as u32);
        }
        Ok(val10)
    }

}

impl PartialEq for BaseN {

    /// Compares partial equivalence between the values of arbitrary bases.
    fn eq(&self, other: &BaseN) -> bool {
        if self.base == other.base {
            (self.vec == other.vec)
        } else {
            self.vec == other.to_base(self.base).unwrap().vec
        }
    }
}

impl PartialOrd for BaseN {

    /// Compares partial order between the values of arbitrary bases.
    fn partial_cmp(&self, other: &BaseN) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BaseN {

    /// Compares total order between the values of arbitrary bases.
    fn cmp(&self, other: &BaseN) -> Ordering {
        self.to_usize().unwrap().cmp(&other.to_usize().unwrap())
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

impl Mul for BaseN {
    type Output = Result<BaseN, &'static str>;

    fn mul(self, rhs: BaseN) -> Result<BaseN, &'static str> {
        // XXX: convert to usize for now
        BaseN::from_usize(self.base, self.to_usize().unwrap() * rhs.to_usize().unwrap())
    }
}

impl Div for BaseN {
    type Output = Result<BaseN, &'static str>;

    fn div(self, rhs: BaseN) -> Result<BaseN, &'static str> {
        // XXX: convert to usize for now
        BaseN::from_usize(self.base, self.to_usize().unwrap() / rhs.to_usize().unwrap())
    }
}
