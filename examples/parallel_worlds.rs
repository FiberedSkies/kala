use std::time::Instant;

use aika::universes::Universe;
use aika::worlds::*;

struct AdderAgent {
    id: usize,
    sum: u64,
}

impl AdderAgent {
    pub fn new(id: usize) -> Self {
        AdderAgent { id, sum: 0 }
    }
}

impl Agent for AdderAgent {
    fn step(&mut self, _: &mut Option<Vec<u8>>, time: &f64, _: &mut Mailbox) -> Event {
        self.sum += 1;

        Event::new(*time, self.id, Action::Wait)
    }

    fn get_state(&self) -> Option<&[u8]> {
        None
    }
}

fn main() {
    let duration = 20_000_000f64;
    let timestep = 1.0;
    let config = Config::new(timestep, Some(duration), 10, 10, false);

    let mut universe = Universe::<8, 1>::new();

    for _ in 0..10 {
        let mut world = World::create(config.clone());

        world.spawn(Box::new(AdderAgent::new(0)));

        world.schedule(0.0, 0).unwrap();

        universe.add_world(world);
    }

    let start = Instant::now();
    let _ = universe.run_parallel();
    let elapsed = start.elapsed();

    let total_steps = universe
        .worlds
        .iter()
        .map(|world| world.step_counter())
        .map(|steps| steps as f64)
        .sum::<f64>();

    println!("Benchmark Results:");
    println!("Total time: {:.2?}", elapsed);
    println!("Total events processed: {}", total_steps);
    println!(
        "Events per second: {:.2}",
        total_steps / elapsed.as_secs_f64()
    );
    println!(
        "Average event processing time: {:.3?} per event",
        elapsed / total_steps as u32
    );
}
