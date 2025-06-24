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

    #[allow(dead_code)]
    struct TreeNode {
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    }
    fn build_a_linked_list(height: u32) -> Option<Box<TreeNode>> {
        if height == 0 {
            return None;
        }
        Some(Box::new(TreeNode {
            value: height as i32,
            left: build_a_linked_list(height - 1),
            right: build_a_linked_list(height - 1),
        }))
    }

    #[test]
    fn benchmark() {
        let tree_height = 10;
        //test without background drop
        let mut vecs = Vec::new();
        for _ in 0..1000 {
            vecs.push(build_a_linked_list(tree_height));
        }
        let start = Instant::now();
        for vec in vecs {
            drop(vec);
        }
        let duration_normal = start.elapsed();

        //test with background drop
        let bgdrop = Bgdrop::new();
        let mut vecs_bg = Vec::new();
        for _ in 0..1000 {
            vecs_bg.push(build_a_linked_list(tree_height));
        }
        let start_bg = Instant::now();
        for vec in vecs_bg {
            bgdrop.drop(vec);
        }
        let duration_bg = start_bg.elapsed();

        // test multiple threads
        let bgdrop_threads = Bgdrop::with_threads(4);
        let mut vecs_bg_threads = Vec::new();
        for _ in 0..1000 {
            vecs_bg_threads.push(build_a_linked_list(tree_height));
        }
        let start_bg_threads = Instant::now();
        for vec in vecs_bg_threads {
            bgdrop_threads.drop(vec);
        }
        let duration_bg_threads = start_bg_threads.elapsed();

        // Print the durations
        println!("Duration without background drop: {:?}", duration_normal);
        println!("Duration with background drop: {:?}", duration_bg);
        println!(
            "Duration with background drop and threads: {:?}",
            duration_bg_threads
        );
        println!(
            "Speedup with background drop: {:.2}x",
            duration_normal.as_secs_f64() / duration_bg.as_secs_f64()
        );
        println!(
            "Speedup with background drop and threads: {:.2}x",
            duration_normal.as_secs_f64() / duration_bg_threads.as_secs_f64()
        );
    }
}
