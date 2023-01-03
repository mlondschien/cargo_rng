use rayon::ThreadPoolBuilder;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

fn main() {
    let seed = 0;

    let mut thread_pool_builder = ThreadPoolBuilder::new();
    thread_pool_builder = thread_pool_builder.num_threads(4);
    let pool = thread_pool_builder.build().unwrap();

    let mut rng = StdRng::seed_from_u64(seed);

    let seeds = (0..10).into_iter().map(|_| rng.gen()).collect::<Vec<u64>>();

    pool.install(|| {
        let result: Vec<Vec<u64>> = seeds.into_par_iter().map(move |seed| {
            println!("\n seed: {}", seed);
            let mut rng = StdRng::seed_from_u64(seed);
            
            let mut result = Vec::new();
            for _ in 0..10 {
                result.push(rng.gen_range(0 .. 100));
            }
            println!("result: {:?}", result);
        
            result
        }).collect();
    });
}
