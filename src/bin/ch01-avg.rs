use rand::Rng;

fn main() {
    let mut rewards: Vec<i32> = vec![];
    let mut rng = rand::thread_rng();
    for n in 1..101 {
        let random_int = rng.gen_range(0..10);
        rewards.push(random_int);

        let q: i32 = rewards.iter().sum();

        println!("{}", q / n);
    }    
}