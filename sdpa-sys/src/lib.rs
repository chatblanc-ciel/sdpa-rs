

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Sdpa {
    innner :SDPA
}
impl Sdpa {
    fn new() -> Self
    {
        Self {
            innner: unsafe{ bindings::SDPA::new() },
        }
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        let _ = Sdpa::new();
    }
}
