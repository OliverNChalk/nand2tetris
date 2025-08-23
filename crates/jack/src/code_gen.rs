use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::parser::structure::{Class, FieldModifier};

pub(crate) fn compile<'a>(class: &Class<'a>) -> Result<Vec<String>, CompileError<'a>> {
    let mut code = Vec::default();

    // Setup the class context before compiling methods.
    let mut indexes = Indices::default();
    let mut symbols = HashMap::default();
    for variable in &class.variables {
        match symbols.entry(variable.name) {
            Entry::Occupied(_) => return Err(CompileError::DuplicateSymbol(variable.name)),
            Entry::Vacant(entry) => {
                let (category, index) = match variable.modifier {
                    FieldModifier::Field => (SymbolCategory::Field, indexes.next_field()),
                    FieldModifier::Static => (SymbolCategory::Static, indexes.next_static()),
                };

                entry.insert(SymbolEntry { name: variable.name, category, index })
            }
        };
    }
    let context = ClassContext { name: class.name, symbols };

    // Generate the code for each subroutine in the class.
    for subroutine in &class.subroutines {
        code.extend(subroutine.compile(&context)?);
    }

    Ok(code)
}

pub(crate) enum CompileError<'a> {
    DuplicateSymbol(&'a str),
}

#[derive(Debug, Default)]
struct Indices {
    field: u16,
    class_static: u16,
}

impl Indices {
    fn next_field(&mut self) -> u16 {
        let next = self.field;
        self.field += 1;

        next
    }

    fn next_static(&mut self) -> u16 {
        let next = self.class_static;
        self.class_static += 1;

        next
    }
}

pub(crate) struct ClassContext<'a> {
    pub(crate) name: &'a str,
    pub(crate) symbols: HashMap<&'a str, SymbolEntry<'a>>,
}

pub(crate) struct SymbolEntry<'a> {
    pub(crate) name: &'a str,
    pub(crate) category: SymbolCategory,
    pub(crate) index: u16,
}

pub(crate) enum SymbolCategory {
    Field,
    Static,
    Local,
    Arg,
}
