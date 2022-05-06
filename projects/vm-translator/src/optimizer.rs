// strategy:
// - iterate over all machine instructions
// - if the instruction in question assigns A to the same thing, delete the 2nd assign
// - god, I wish I had an intermediate format instead of text

// perf: this function can def be optimized (no pun intended)
pub fn optimize(source: Vec<String>) -> Vec<String> {
    let mut redundant_lines = Vec::new();

    let mut current_a = String::from("0");
    for (index, line) in source.iter().enumerate() {
        // skip over whitespace
        if line.contains("//") || line.is_empty() { continue; }

        // check for assingment of A via A-instruction
        let a_index = line.find("@");
        if let Some(a_index) = a_index {
            // todo: is there a reduce() that can return an arbitrary type?
            let mut new_a = String::with_capacity(4);
            line.chars().skip(a_index).for_each(|char| {
                if char.is_numeric() { new_a.push(char); }
            });

            if new_a == current_a {
                // as the line is a no-op, let's mark for deletion
                redundant_lines.push(index);
            }

            // regardless of no-op not, we can safely update current_a to latest
            current_a = new_a;
        }

        // check for assingment of A via C-instruction
        let assingment_index = line.find("A=");
        if let Some(assignment_index) = assingment_index {
            current_a = String::from(line.chars().nth(assignment_index + 2).unwrap());
        }

        // todo: remove A=? assignments if they are no ops, requires tracking mutation of M & D
    }

    // perf: depending on amount of source, we should not be searching an array
    let mut optimized_source = Vec::new();
    for (index, line) in source.iter().enumerate() {
        if redundant_lines.contains(&index) { continue; }

        optimized_source.push(line.clone());
    }

    optimized_source
}
