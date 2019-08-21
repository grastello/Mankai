use crate::interpreter::*;
use crate::parser::Sexp;
use crate::token::TokenKind;

/// The `if!` special form.
pub fn if_special_form(
    interpreter: &mut Interpreter,
    arguments: Vec<&Sexp>,
) -> Result<MankaiObject, RuntimeError> {
    // Check that we have exactly three arguments.
    if arguments.len() != 3 {
        return Err(RuntimeError::new("'if!' requires exactly three arguments!"));
    }

    // Evaluate the condition.
    let condition = interpreter.evaluate(arguments.get(0).unwrap())?;

    // Evaluate the "then" or the "else" branch accordingly.
    match condition {
        MankaiObject::Bool(true) => interpreter.evaluate(arguments.get(1).unwrap()),
        MankaiObject::Bool(false) => interpreter.evaluate(arguments.get(2).unwrap()),
        _ => Err(RuntimeError::new(
            "1st argument to 'if!' must evaluate to a boolean!",
        )),
    }
}

/// The `set!` special form.
pub fn set(
    interpreter: &mut Interpreter,
    arguments: Vec<&Sexp>,
) -> Result<MankaiObject, RuntimeError> {
    // Check that we have exactly two arguments.
    if arguments.len() != 2 {
        return Err(RuntimeError::new("expected exacly two arguments to 'set!'"));
    }

    // Get token that identifies the name of the variable. Return an error if
    // trying to set! a special form of a native function or if the token is not
    // an identifier.
    let name = match arguments.get(0).unwrap() {
        Sexp::Atom(token) => token,
        Sexp::List(_) => {
            return Err(RuntimeError::new(
                "expected identifier as first argument to 'set!'",
            ))
        }
    };

    if let TokenKind::Identifier = name.kind {

    } else {
        return Err(RuntimeError::new(&format!(
            "'{}' is not an identifier!",
            name.lexeme
        )));
    }

    if interpreter.is_special_form(name) {
        return Err(RuntimeError::new(&format!(
            "can't assign to '{}' because the name is reserved for a special form!",
            name.lexeme
        )));
    }

    if interpreter.is_native_fucntion(name) {
        return Err(RuntimeError::new(&format!(
            "can't assign to '{}' because the name is reserved for a native function!",
            name.lexeme
        )));
    }

    if interpreter.is_constant(name) {
        return Err(RuntimeError::new(&format!(
            "can't assign to '{}' because the name is reserved for a constant!",
            name.lexeme
        )));
    }

    // Get the value to assign.
    let value = interpreter.evaluate(arguments.get(1).unwrap())?;
    let value_clone = value.clone();

    // Perform the binding.
    interpreter.environment.define(name, value);
    Ok(value_clone)
}
