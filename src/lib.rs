const MULTIPLIER: i64 = 0x5deece66d;
const INCREMENT: i64 = 0xb;
const MASK: i64 = (1 << 48) - 1;

pub struct Random {
    state: i64,
}


impl Random {
    fn next_i32(&self) -> i32 {
        unimplemented!()
    }
    fn next_i32(&self, bound: i32) -> i32 {
        unimplemented!()
    }
    fn next_i64(&self) -> i64 {
        unimplemented!()
    }
    fn next_bool(&self) -> bool {
        unimplemented!()
    }
    fn next_f32(&self) -> f32 {
        unimplemented!()
    }
    fn next_f64(&self) -> f64 {
        unimplemented!()
    }
    fn next_bytes(&self, bytes: [u8]) -> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}