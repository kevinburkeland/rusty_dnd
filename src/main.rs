use crate::dice::Dice;


pub mod dice;

fn main() {
    //instatiate results vector
    let mut results: Vec<u32> = Vec::new();
    for _ in 0..6 {
        //roll 4d6, drop the lowest
        let mut rolls: Vec<u32> = Dice::d6(4);
        rolls.sort_unstable();
        rolls.remove(0);
        //sum the remaining rolls
        let mut sum: u32 = 0;
        for roll in rolls {
            sum += roll;
        }
        //add the sum to the vector of results
        results.push(sum);
    }
    //print results
    for result in results {
        println!("{}", result);
}
}