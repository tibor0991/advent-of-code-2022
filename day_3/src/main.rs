// Provides an interface to compute a priority given a char
trait CharScore {
    fn get_priority(&self) -> u8;
}

impl CharScore for char {
    // The key insight here is that the priority can also be used as a bit index in a u64 number...
    fn get_priority(&self) -> u8 {
        match *self {
            lowercase @ 'a'..='z' => (lowercase as u8) - 97 + 1,
            uppercase @ 'A'..='Z' => (uppercase as u8) - 65 + 27,
            _ => unreachable!()
        }
    }
}

// Provides an interface to turn a type into a u64 bitmask
trait BitMask {
    fn to_bitmask(&self) -> u64;
}

impl BitMask for &[char] {
    // ...which we can use to compute a kind of hash set of a given string.
    // We don't care about duplicates, we assign a bit to each available char.
    // (Inspired by a video from Matt Parker @ Stand-up Math)
    fn to_bitmask(&self) -> u64 {
        self
        .into_iter()
        .fold(0u64, |acc, item| 
            {acc | (1 << item.get_priority())}
        )
    }
}

fn main() {
    // The rucksacks, as presented in the input (no file parsing, it's the same anyway)
    let rucksacks = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw"
    ];

    // Computes the total priority
    let total_priority: u32 = rucksacks
        // Turn vector into iterable
        .into_iter() 
        // For each iterator:   
        .map(|r| 
            // Convert string iterator to list of chars
            r.chars()
            // Collect into vector of chars (lets us use chunks_exact later)
            .collect::<Vec<char>>()
            // Obtain at most 2 chunks
            .chunks_exact(r.len() / 2)
            // Turn each chunk into its u64 representation
            .map(|chunk| chunk.to_bitmask())
            // Fold the chunks on each other to get the common bitmask
            .fold(u64::MAX, |acc, item| acc & item))
        // For each bitmask, obtain the index (equal to the priority of the common item)
        .map( |m| (f64::log2(m as f64) as u32))
        // Sum all priorities (and actually runs the iterators)
        .sum();

    // Prints the total priority
    println!("The sum of priorities is {total_priority}!");

}
