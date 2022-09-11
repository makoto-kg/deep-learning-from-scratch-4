use rand::Rng;
use std::cmp::Ordering;
use plotters::prelude::*;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let steps = 1_000;
    let epsilon = 0.1;

    let bandit = Bandit::new(20);
    let mut agent = Agent::new(epsilon, Some(20));

    let mut total_reward = 0;
    let mut total_rewards: Vec<f64> = vec![];
    let mut rates: Vec<f64> = vec![];

    for step in 0..steps {
        // 1. choose an action
        let action = agent.get_action();

        // 2. get a reward
        let reward = bandit.play(action);

        // 3. learn from an action and a reward
        agent.update(action, reward);
        total_reward += reward;

        total_rewards.push(total_reward as f64);
        rates.push(total_reward as f64 / (step as f64 + 1f64));
    }

    // println!("Total rewards: {:?}", total_reward);
    // println!("Total reward: {:?}", total_rewards);

    let (_, rewards_max) = total_rewards
        .iter()
        .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v | (v.min(m), v.max(n)));
    let (_, rates_max) = rates
        .iter()
        .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v| (v.min(m), v.max(n)));
    
    // prepare for drawing a graphs
    let mut points_total_rewards = vec![];
    let mut points_rates = vec![];
    for (i, val) in total_rewards.iter().enumerate() {
        points_total_rewards.push(((i + 1) as f64, *val));
    }
    for (i, val) in rates.iter().enumerate() {
        points_rates.push(((i + 1) as f64, *val));
    }

    // draw graph
    let root =
        BitMapBackend::new("output/bandit/total_reward.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bandit Total Reward", ("sans-serif", 20).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..1_000f64, 0f64..rewards_max)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(points_total_rewards, &RED))?;

    // draw a graph2
    let root =
        BitMapBackend::new("output/bandit/rates.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption("Bandit Rates", ("sans-serif", 20).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..1_000f64, 0f64..rates_max)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(points_rates, &RED))?;

    Ok(())
}