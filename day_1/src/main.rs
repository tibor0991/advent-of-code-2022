

use std::env;
use std::fs;
use std::process::exit;


fn main() -> Result<(), String>{
    // Collect the arguments from command line (just the input file path)
    let args: Vec<String> = env::args()
        .collect();
    
    // Attempts to retrieve the cmd line argument, propagates error upwards.
    let input_path = args.get(1)
        .ok_or("No input file provided!")?;

    // Attempts to open the file, propagates error upwards.
    let file_content = fs::read_to_string(input_path)
        .map_err( |err| format!("Unable to open file {input_path}: {err}"))?;
    
    let (elf_index, highest_calorie) = file_content
        .split("\n\n")
        .map(|full_inv|
            full_inv.split("\n")
            .filter_map(|s|->Option<_> {
                match s.parse::<u32>() {
                    Ok(t) => Some(t),
                    Err(e) => { println!("Unable to parse [{s}]: {e}", ); exit(2) }
                }
            })
            .sum::<u32>()
        )
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1)).ok_or("An error occured!")?;
    
    println!("Elf {elf_index} has the most calories in its backpack! ({highest_calorie} calories)");
    Ok(())
}
