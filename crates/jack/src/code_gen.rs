use crate::parser::structure::Class;

pub(crate) fn compile(class: &Class) -> Vec<String> {
    let mut code = Vec::default();

    for variable in &class.variables {
        todo!();
    }

    for subroutine in &class.subroutines {
        // Function boilerplate.
        code.push(format!(
            "function {}.{} {}",
            class.name,
            subroutine.name,
            subroutine.body.variables.len()
        ));
        code.push("push argument 0".to_string());
        code.push("pop pointer 0".to_string());

        // Function body.
        code.extend(subroutine.body.compile());
    }

    code
}
