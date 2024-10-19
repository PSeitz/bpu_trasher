use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let n = 8096; // Number of branches
    let group_size = 1024; // Number of branches per function

    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_code.rs");
    let mut file = File::create(&dest_path).unwrap();

    // Add necessary imports at the top of the generated file
    writeln!(file, "use std::arch::asm;").unwrap();
    writeln!(file, "use std::hint::black_box;").unwrap();

    writeln!(
        file,
        "/// This function should not be inlined by the compiler to prevent the generated code from being optimized out"
    )
    .unwrap();
    writeln!(file, "#[inline(never)]").unwrap();
    writeln!(file, "pub fn do_something(val: u32) {{").unwrap();
    writeln!(file, "    black_box(val);").unwrap();
    writeln!(file, "}}").unwrap();

    // Generate multiple smaller functions to handle branches
    let mut function_count = 0;
    for chunk in (0..n).collect::<Vec<_>>().chunks(group_size) {
        // Start a new function
        writeln!(file, "#[inline(never)]").unwrap();
        writeln!(
            file,
            "pub fn eval_branches_{}(random: &mut dyn FnMut() -> u32) {{",
            function_count
        )
        .unwrap();

        for i in chunk {
            writeln!(file, "    let val = random();").unwrap();
            writeln!(file, "    if val % 2 == 1 {{").unwrap();
            writeln!(file, "        do_something(val);").unwrap();
            writeln!(file, "    }}").unwrap();

            // Generate a random number of nops (0-3) to randomize the addresses
            let nop_count = i % 4;
            if nop_count != 0 {
                writeln!(file, "    unsafe {{").unwrap();
                for _ in 0..nop_count {
                    writeln!(file, "        asm!(\"nop\");").unwrap();
                }
                writeln!(file, "    }}").unwrap();
            }
        }

        writeln!(file, "}}").unwrap();
        function_count += 1;
    }

    // Main function to dispatch across the smaller functions
    writeln!(
        file,
        "pub fn eval_branches(random: &mut dyn FnMut() -> u32) {{"
    )
    .unwrap();
    for i in 0..function_count {
        writeln!(file, "    eval_branches_{}(random);", i).unwrap();
    }
    writeln!(file, "}}").unwrap();
}
