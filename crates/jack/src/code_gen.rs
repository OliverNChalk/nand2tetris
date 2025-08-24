use std::sync::atomic::{AtomicU64, Ordering};

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use thiserror::Error;

use crate::parser::structure::{Class, FieldModifier, Type};

pub(crate) fn compile<'a>(
    vm_symbol_counter: &'static AtomicU64,
    class: &Class<'a>,
) -> Result<Vec<String>, CompileError<'a>> {
    let mut code = Vec::default();

    // Setup the class context before compiling methods.
    let mut indexes = Indices::default();
    let mut symbols = HashMap::default();
    for variable in &class.variables {
        match symbols.entry(variable.name) {
            Entry::Occupied(_) => return Err(CompileError::DuplicateSymbol(variable.name)),
            Entry::Vacant(entry) => {
                let (category, index) = match variable.modifier {
                    FieldModifier::Field => (SymbolLocation::This, indexes.next_field()),
                    FieldModifier::Static => (SymbolLocation::Static, indexes.next_static()),
                };

                entry.insert(SymbolEntry { symbol_type: variable.var_type, category, index })
            }
        };
    }
    let context = ClassContext { name: class.name, symbols, vm_symbols: vm_symbol_counter };

    // Generate the code for each subroutine in the class.
    for subroutine in &class.subroutines {
        code.extend(subroutine.compile(&context)?);
    }

    Ok(code)
}

#[derive(Debug, Error)]
pub(crate) enum CompileError<'a> {
    #[error("Duplicate symbol; symbol={0}")]
    DuplicateSymbol(&'a str),
    #[error("Invalid callee; callee={0}")]
    InvalidCallee(&'a str),
    #[error("Unknown symbol; symbol={0}")]
    UnknownSymbol(&'a str),
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
    pub(crate) vm_symbols: &'static AtomicU64,
}

impl<'a> ClassContext<'a> {
    pub(crate) fn next_label(&self) -> u64 {
        self.vm_symbols.fetch_add(1, Ordering::Relaxed)
    }
}

pub(crate) struct SymbolEntry<'a> {
    pub(crate) symbol_type: Type<'a>,
    pub(crate) category: SymbolLocation,
    pub(crate) index: u16,
}

impl<'a> SymbolEntry<'a> {
    pub(crate) fn compile_push(&self) -> String {
        format!("push {} {}", self.category, self.index)
    }

    pub(crate) fn compile_pop(&self) -> String {
        format!("pop {} {}", self.category, self.index)
    }
}

#[derive(Debug, PartialEq, Eq, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum SymbolLocation {
    This,
    Static,
    Local,
    Argument,
}
