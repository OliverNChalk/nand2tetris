pub struct JackFile<'a> {
    source: Vec<JackLine<'a>>,
}

type JackLine<'a> = (usize, &'a str);

impl<'a> JackFile<'a> {
    pub fn new(file: &'a String) -> JackFile<'a> {
        let source = file
            .lines()
            .map(|l| l.trim())
            .filter(|l| !(l.starts_with("//") || l.len() == 0))
            .enumerate()
            .map(|(number, line)| (number + 1, line.trim()))
            .collect();

        JackFile { source }
    }

    pub fn commands(&self) -> &Vec<JackLine<'a>> {
        &self.source
    }
}
