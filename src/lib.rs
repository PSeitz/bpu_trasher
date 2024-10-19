// Imports `trash_bpu`
include!(concat!(env!("OUT_DIR"), "/generated_code.rs"));

/// Trashes the BPU
pub fn trash_bpu() {
    for _ in 0..256 {
        eval_branches(&mut || rand::random::<u32>());
    }
    let mut alternate = || {
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            COUNTER
        }
    };
    for _ in 0..256 {
        eval_branches(&mut alternate);
    }
    // Allways false
    for _ in 0..256 {
        eval_branches(&mut || 0);
    }
    // Allways true
    for _ in 0..256 {
        eval_branches(&mut || 1);
    }
}
