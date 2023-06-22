use rand::Rng;

pub struct Dice {
    pub rolls: u32,
}

impl Dice {
    pub fn d4(x: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut results: Vec<u32> = Vec::new();
        for _ in 0..x {
            results.push(rng.gen_range(1..5));
        }
        results
    }
    pub fn d6(x: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut results: Vec<u32> = Vec::new();
        for _ in 0..x {
            results.push(rng.gen_range(1..7));
        }
        results
    }
    pub fn d8(x: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut results: Vec<u32> = Vec::new();
        for _ in 0..x {
            results.push(rng.gen_range(1..9));
        }
        results
    }
    pub fn d10(x: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut results: Vec<u32> = Vec::new();
        for _ in 0..x {
            results.push(rng.gen_range(1..11));
        }
        results
    }
    pub fn d12(x: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut results: Vec<u32> = Vec::new();
        for _ in 0..x {
            results.push(rng.gen_range(1..13));
        }
        results
    }
    pub fn d20(x: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut results: Vec<u32> = Vec::new();
        for _ in 0..x {
            results.push(rng.gen_range(1..21));
        }
        results
    }
    pub fn d100(x: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut results: Vec<u32> = Vec::new();
        for _ in 0..x {
            results.push(rng.gen_range(1..101));
        }
        results
    }
}