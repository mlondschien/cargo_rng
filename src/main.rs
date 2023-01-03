use rayon::ThreadPoolBuilder;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

fn main() {
    let seed = 0;

    let mut thread_pool_builder = ThreadPoolBuilder::new();
    thread_pool_builder = thread_pool_builder.num_threads(1);
    let pool = thread_pool_builder.build().unwrap();

    let mut rng = StdRng::seed_from_u64(seed);

    let seeds = (0..10).into_iter().map(|_| rng.gen()).collect::<Vec<u64>>();

    pool.install(|| {
        seeds.into_par_iter().for_each(|seed| {
            println!("\n seed: {}", seed);
            let mut rng = StdRng::seed_from_u64(seed);
            
            for _ in 0..2 {
                println!("{}", rng.gen_range(0 .. 100));
            }
        });
    });
}
