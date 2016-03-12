
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::cmp;

#[derive(Debug, Eq)]
pub struct BaseN/*<T>*/{
    pub base: usize, // TODO: Why isn't base u8?  Want to limit to 255 right?
    pub vec: Vec</*T*/u8>
    // TODO: Make proper accessor functions for struct fields
}

impl BaseN {

    pub fn new(base: usize) -> BaseN/*<T>*/ {
        BaseN { base: base, vec: Vec::new() }
    }
    pub fn with_capacity(base: usize, capacity: usize) -> BaseN {
        BaseN { base: base, vec: Vec::with_capacity(capacity) }
    }
    pub fn with_existing(base: usize, vec: Vec<u8>) -> BaseN {
        BaseN { base: base, vec: vec }
    }
    pub fn from_usize(base: usize, val: usize) -> Result<BaseN, &'static str> {

        let mut new_vec:Vec<u8> = Vec::new();
        let mut val10: usize = val;
        loop {
            let _val = val10 / base;
            match _val {
                0 => {
                    new_vec.push(val10 as u8);
                    break;
                }
                _ => {
                    let remainder = val10 % base;
                    //println!("{}", remainder);
                    new_vec.push(remainder as u8);
                    val10 = _val;
                }
            }
        }
        let new_basen: BaseN = BaseN { base: base, vec: new_vec };

        Ok(new_basen)
    }

    pub fn to_base(self, new_base: usize) -> Result<BaseN, &'static str> {

        //TODO: If base is same, just return existing??
        if self.base == new_base {
            return Ok(self);
        }

        // TODO: Create copy, convert mutable, or both?
        //let new_basen = BaseN::new(new_base);

        //println!("converting base {} to base {}", self.base, new_base);
        let mut val10: usize = self.to_usize().unwrap();
        //println!("{}", val10);

        BaseN::from_usize(new_base, val10)
        
    }
    pub fn to_usize(&self) -> Result<usize, &'static str> {
        let mut _val10: usize = 0;
        for (i, x) in self.vec.iter().enumerate() {
            _val10 = _val10 + (*x as usize) * self.base.pow(i as u32);
        }
        Ok(_val10)
    }

}

//impl Copy for BaseN { }

impl PartialEq for BaseN {
    fn eq(&self, other: &BaseN) -> bool {
        (self.vec == other.vec)
            && (self.base == other.base)
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
    fn add(self, _rhs: BaseN) -> Result<BaseN, &'static str> {

        let add_basen = match _rhs.base==self.base {
            true => _rhs,//.clone(),
            false => _rhs.to_base(self.base).unwrap()
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

