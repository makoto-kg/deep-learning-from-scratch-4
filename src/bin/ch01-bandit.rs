use rand::Rng;
use std::cmp::Ordering;

struct Bandit {
    rates: Vec<i32>,
}

impl Bandit {
    fn new(arms: i32) -> Bandit {
        let mut rng = rand::thread_rng();
        let mut rates: Vec<i32> = vec![];
        for _ in 0..arms {
            rates.push(rng.gen());
        }

        Bandit {
            rates: rates,
        }
    }

    fn play(&self, arm: usize) -> u32 {
        let mut rng = rand::thread_rng();
        if self.rates[arm] > rng.gen() {
            1
        } else {
            0
        }
    }
}

pub struct Agent {
    epsilon: f32,
    qs: Box<[f32]>,
    ns: Box<[f32]>,
}

impl Agent {
    fn new(epsilon: f32, action_size: Option<usize>) -> Agent {
        Agent {
            epsilon: epsilon,
            qs: vec![0.0; action_size.unwrap_or(10)].into_boxed_slice(),
            ns: vec![0.0; action_size.unwrap_or(10)].into_boxed_slice(),
        }
    }

    pub fn update(&mut self, action: usize, reward: u32) {
        self.ns[action] += 1.0;
        self.qs[action] += (reward as f32 - self.qs[action]) / self.ns[action];
    }

    pub fn get_action(&self) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < self.epsilon {
            rng.gen_range(0..self.qs.len()) 
        } else {
            self.qs
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(index, _)| index)
                .unwrap() as usize
        }
    }
}

fn main() {
    let steps = 1000;
    let epsilon = 0.1f32;

    let bandit = Bandit::new(20);
    let mut agent = Agent::new(epsilon, Some(20));

    let mut total_reward = 0;
    let mut total_rewards: Vec<u32> = vec![];
    let mut rates: Vec<f32> = vec![];

    for step in 0..steps {
        // 1. choose an action
        let action = agent.get_action();

        // 2. get a reward
        let reward = bandit.play(action);

        // 3. learn from an action and a reward
        agent.update(action, reward);
        total_reward += reward;

        total_rewards.push(total_reward);
        rates.push(total_reward as f32 / (step + 1) as f32);
    }

    println!("{:?}", total_rewards);
}