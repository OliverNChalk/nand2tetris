use crate::tokenizer::Tokenizer;

pub(crate) fn parse(mut tokenizer: Tokenizer) -> Class {
    Class { name: "TODO".to_string(), variables: vec![], subroutines: vec![] }
}

#[derive(Debug)]
pub(crate) struct Class {
    pub(crate) name: String,
    pub(crate) variables: Vec<VariableDeclaration>,
    pub(crate) subroutines: Vec<SubroutineDeclaration>,
}

#[derive(Debug)]
pub(crate) struct VariableDeclaration {}

#[derive(Debug)]
pub(crate) struct SubroutineDeclaration {}
