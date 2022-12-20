use std::collections::BinaryHeap;
use std::env;
use std::fs;



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
    
    // Find the three top-most elves:
    let podium = 
        // Split input string over double new-lines
        file_content.split("\n\n")
        // For each group of lines...
        .map(|item| 
            // Split again each group over single newlines
            item.split("\n")
            // For each line in a group, parse as a u32 and unwrap (disregard parsing errors, the input is clean)
            .map( |s|
                // Parses and returns a result
                s.parse::<u32>()
                // Disregard errors
                .unwrap())
            // Sum all values in a group
            .sum::<u32>())
        // Collect all total inventories into a binary heap (sorted low to high)
        .collect::<BinaryHeap<_>>()
        // Turn the binary heap into a sorted vector (into_iter_sorted is still not available)
        .into_sorted_vec()
        // Get iterator from vec
        .into_iter()
        // Reverse its traversing order (greatest to lowest value)
        .rev()
        // Take only the first three sums
        .take(3)
        // Collect into a vec
        .collect::<Vec<_>>();

    // First question: what's the highest amount of calories an elf is carrying?
    println!("The highest amount of calories carried by an elf is {}", &podium[0] );
        
    // Second question: what's the total amount of calories carried by the top three elves?
    let podium_sum: u32 = podium.iter().sum();
    println!("The sum of the top 3 elves with the most calories is {podium_sum}");

    Ok(())
}
