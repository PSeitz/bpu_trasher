// Imports `trash_bpu`
mod generated_code;
use generated_code::*;

/// Trashes the branch predictor with 256 iterations
/// See [trash_bpu_with_iterations] for more information
#[inline(never)]
pub fn trash_bpu() {
    trash_bpu_with_iterations(256);
}

/// Trashes the the branch predictor
///
/// It evaluates 8k branches * num_iterations * 4
///
/// Some BPU have 2-level adaptive predictors, so we need to apply different patterns to trash it.
///
/// ## TODO
/// The current patterns are not sufficent on some architectures. We need to have local and global patterns.
/// Local patterns are patterns that are applied to a single branch, while global patterns are
/// applied to all branches.
///
/// Currently there are only global patterns applied.
#[inline(never)]
pub fn trash_bpu_with_iterations(num_iterations: u32) {
    for _ in 0..num_iterations {
        eval_branches(&mut || rand::random::<u32>());
    }
    let mut alternate = || {
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            COUNTER
        }
    };
    for _ in 0..num_iterations {
        eval_branches(&mut alternate);
    }
    // Allways false
    for _ in 0..num_iterations {
        eval_branches(&mut || 0);
    }
    // Allways true
    for _ in 0..num_iterations {
        eval_branches(&mut || 1);
    }
}
