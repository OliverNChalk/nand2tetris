// strategy:
// - iterate over all machine instructions
// - if the instruction in question assigns A to the same thing, delete the 2nd assign


pub fn optimize(source: Vec<String>) -> Vec<String> {
    let cleaned = source.iter().filter(|line| !line.contains("//") && !line.is_empty());

    for line in cleaned {
        println!("{}", line);
        // do your magic
    }

    source
}
