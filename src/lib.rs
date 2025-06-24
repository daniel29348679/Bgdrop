pub mod bgdrop;
pub mod prelude;

pub use bgdrop::Bgdrop;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn it_works() {
        let bgdrop = Bgdrop::new();
        bgdrop.drop(42);
    }

    #[test]
    fn benchmark_with_threads() {
        let bgdrop = Bgdrop::with_threads(1);
        let mut vecs = Vec::new();
        for i in 0..100_000 {
            let vec = vec![i; 1000];
            vecs.push(vec); // Keep the vec alive until the end of the test
        }
        let start = Instant::now();
        for vec in vecs {
            bgdrop.drop(vec); // Drop each vector in the background
        }
        let duration = start.elapsed();
        println!("Dropped 100,000 vectors in background in {:?}", duration);

        let bgdrop = Bgdrop::with_threads(1);
        let mut vecs = Vec::new();
        for i in 0..100_000 {
            let vec = vec![i; 1000];
            vecs.push(vec); // Keep the vec alive until the end of the test
        }
        let start = Instant::now();
        for vec in vecs {
            bgdrop.drop(vec); // Drop each vector in the background     
        }
        let duration = start.elapsed();
        println!(
            "Dropped 100,000 vectors in background with 4 threads in {:?}",
            duration
        );

        // use normal drop to ensure the background thread has time to process
        let mut vecs = Vec::new();
        for i in 0..100_000 {
            let vec = vec![i; 1000];
            vecs.push(vec); // Keep the vec alive until the end of the tests    
        }

        let start = Instant::now();
        for vec in vecs {
            drop(vec); // Drop each vector normally
        }
        let duration = start.elapsed();
        println!("Dropped 100,000 vectors normally in {:?}", duration);
    }
}
