use jandom::Random;
use std::env;

fn main() {
    let world_seed = env::args()
        .nth(1)
        .map(|seed| seed.parse::<i64>().ok())
        .flatten()
        .expect("no seed provided");
    let x = env::args()
        .nth(2)
        .map(|x| x.parse::<i32>().ok())
        .flatten()
        .expect("no x coordinate provided");
    let z = env::args()
        .nth(3)
        .map(|z| z.parse::<i32>().ok())
        .flatten()
        .expect("no z coordinate provided");

    let x1 = x.wrapping_mul(x).wrapping_mul(0x4c1906i32) as i64;
    let x2 = x.wrapping_mul(0x5ac0dbi32) as i64;
    let z1 = (z.wrapping_mul(z) as i64).wrapping_mul(0x4307a7i64);
    let z2 = z.wrapping_mul(0x5f24fi32) as i64;

    let seed = world_seed
        .wrapping_add(x1)
        .wrapping_add(x2)
        .wrapping_add(z1)
        .wrapping_add(z2)
        ^ 0x3ad8025f;

    let mut rng = Random::new(seed);

    if rng.next_i32_bounded(10) == 0 {
        println!("\x1b[0;32mChunk is slimechunk\x1b[0;32m");
    } else {
        println!("Chunk is not slimechunk");
    }
}
