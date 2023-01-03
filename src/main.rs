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
        let _result: Vec<Vec<u64>> = seeds.into_par_iter().map(move |seed| {
            println!("\n seed: {}", seed);
            let mut rng = StdRng::seed_from_u64(seed);
            
            let used = rng.gen::<u64>();
            println!("used: {}", used);

            let mut result = Vec::new();
            for _idx in 0..10 {
                let s: u64 = rng.gen_range(0 .. 100);
                result.push(s);
            }
            println!("result: {:?}", result);
        
            result
        }).collect();
    });
}
