use crate::interpreter::{MankaiObject, RuntimeError};

// Functions with symbolic names (such as '+', '-', ...).

/// Sum all the arguments. Return an error if a non numeric argument is found
/// or no arguments are found at all.
pub fn sum(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'+' requires at least one argument!"));
    }

    // Perform the sum.
    let mut sum = 0.0;
    for (i, value) in arguments.iter().enumerate() {
        match value {
            MankaiObject::Number(n) => sum += n,
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument of '+' must be a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(sum))
}

/// Perform subtraction of an arbitrary number of elements.
/// If only one element is given then substract act just inverts it and return,
/// if multiple arguments are given multiple substractions are performed
/// starting from the first argument e.g. substract([a, b, c]) = a - b - c.
pub fn substract(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'-' requires at least one argument!"));
    }

    // If there's only one argument negate it and return.
    if arguments.len() == 1 {
        return match arguments.get(0).unwrap() {
            MankaiObject::Number(n) => Ok(MankaiObject::Number(-n)),
            _ => Err(RuntimeError::new("1st arguments to '-' must be a number!")),
        };
    }

    // If there are more arguments perform the right number of substractions.
    let mut result = match arguments.get(0).unwrap() {
        MankaiObject::Number(n) => *n,
        _ => return Err(RuntimeError::new("1st arguments to '-' must be a number!")),
    };

    for (i, value) in arguments.iter().enumerate().skip(1) {
        match value {
            MankaiObject::Number(n) => result -= n,
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument to '-' must be a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(result))
}

/// Multiply all the arguments. Return an error if a non numeric argument is
/// found or no arguments are found at all.
pub fn multiplication(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'*' requires at least one argument!"));
    }

    // Perform the multiplication of all arguments.
    let mut result = 1.0;
    for (i, value) in arguments.iter().enumerate() {
        match value {
            MankaiObject::Number(n) => result *= n,
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument to '*' must be a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(result))
}

/// Divide all the arguments togheter. Return an error if a non numeric
/// argument is gound or no arguments are found at all.
/// We impose that division([a]) = 1/a.
pub fn division(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'/' requires at least one argument!"));
    }

    // Handle the one argument case.
    if arguments.len() == 1 {
        return match arguments.get(0).unwrap() {
            MankaiObject::Number(n) => Ok(MankaiObject::Number(1.0 / n)),
            _ => Err(RuntimeError::new("1st argument to '/' must be a number!")),
        };
    }

    // Handle the multiple arguments case.
    let mut result = match arguments.get(0).unwrap() {
        MankaiObject::Number(n) => *n,
        _ => return Err(RuntimeError::new("1st argument to '/' must be a number!")),
    };

    for (i, value) in arguments.iter().enumerate().skip(1) {
        match value {
            MankaiObject::Number(n) => {
                if *n != 0.0 {
                    result /= n
                } else {
                    return Err(RuntimeError::new(&format!(
                        "can't divide by zero ({}-th argument to '/' is zero)!",
                        i + 1
                    )));
                }
            }
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument to '/' must be a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(result))
}

/// == implementation.
pub fn equals(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.len() != 2 {
        return Err(RuntimeError::new("'==' requires exactly two arguments!"));
    }

    let left = arguments.get(0).unwrap();
    let right = arguments.get(1).unwrap();

    Ok(MankaiObject::Bool(left == right))
}

/// > implementation.
pub fn greater_than(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.len() != 2 {
        return Err(RuntimeError::new("'>' reuires exactly two arguments!"));
    }

    let left = match arguments.get(0).unwrap() {
        MankaiObject::Number(n) => *n,
        _ => return Err(RuntimeError::new("1st argument to '>' must be a number!")),
    };

    let right = match arguments.get(1).unwrap() {
        MankaiObject::Number(n) => *n,
        _ => return Err(RuntimeError::new("2nd argument to '>' must be a number!")),
    };

    Ok(MankaiObject::Bool(left > right))
}

/// < implementation.
pub fn less_than(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.len() != 2 {
        return Err(RuntimeError::new("'<' reuires exactly two arguments!"));
    }

    let left = match arguments.get(0).unwrap() {
        MankaiObject::Number(n) => *n,
        _ => return Err(RuntimeError::new("1st argument to '<' must be a number!")),
    };

    let right = match arguments.get(1).unwrap() {
        MankaiObject::Number(n) => *n,
        _ => return Err(RuntimeError::new("2nd argument to '<' must be a number!")),
    };

    Ok(MankaiObject::Bool(left < right))
}

// Functions with alfanumeric names.

/// Logic AND with unfixed arity.
pub fn and(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'and' requires at least one argument!"));
    }

    // Perform and.
    for (i, value) in arguments.iter().enumerate() {
        match value {
            MankaiObject::Bool(false) => return Ok(MankaiObject::Bool(false)),
            MankaiObject::Bool(true) => (),
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument to 'and' is not a boolean!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Bool(true))
}

/// Analogue of lisp's iconic `car`: get the head of a list.
pub fn car(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.len() != 1 {
        return Err(RuntimeError::new("'car' requires exectly one argument!"));
    }

    match arguments.get(0).unwrap() {
        MankaiObject::List(list) => {
            if list.is_empty() {
                Err(RuntimeError::new("can't apply 'car' to the empty list!"))
            } else {
                Ok(list.get(0).unwrap().clone())
            }
        }
        _ => Err(RuntimeError::new("1st argument to 'car' must be a list!")),
    }
}

/// Analogue of lisp's `cdr`: get the tail of a list.
pub fn cdr(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.len() != 1 {
        return Err(RuntimeError::new("'cdr' requires exectly one argument!"));
    }

    match arguments.get(0).unwrap() {
        MankaiObject::List(list) => {
            if list.is_empty() {
                Err(RuntimeError::new("can't apply 'cdr' to the empty list!"))
            } else {
                let mut cdr = Vec::new();
                for value in list.iter().skip(1) {
                    cdr.push(value.clone());
                }

                Ok(MankaiObject::List(cdr))
            }
        }
        _ => Err(RuntimeError::new("1st argument to 'car' must be a list!")),
    }
}

/// Cons function. Append to the first argument all the others in the given
/// order. The first argument must be a list.
pub fn cons(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.len() < 2 {
        return Err(RuntimeError::new("'cons' requires at least two arguments!"));
    }

    // Do the appending.
    let first = arguments.get(0).unwrap().clone();
    match first {
        MankaiObject::List(mut list) => {
            for value in arguments.iter().skip(1) {
                list.push(value.clone());
            }

            Ok(MankaiObject::List(list))
        }
        _ => Err(RuntimeError::new("1st argument to 'cons' must be a list")),
    }
}

/// Check if the given argument is a Mankai boolean.
pub fn is_boolean(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity. {
    if arguments.len() != 1 {
        return Err(RuntimeError::new("'list?' requires exactly one argument!"));
    }

    match arguments.get(0).unwrap() {
        MankaiObject::Bool(_) => Ok(MankaiObject::Bool(true)),
        _ => Ok(MankaiObject::Bool(false)),
    }
}

/// Check if the given argument is a Mankai list.
pub fn is_list(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity. {
    if arguments.len() != 1 {
        return Err(RuntimeError::new("'list?' requires exactly one argument!"));
    }

    match arguments.get(0).unwrap() {
        MankaiObject::List(_) => Ok(MankaiObject::Bool(true)),
        _ => Ok(MankaiObject::Bool(false)),
    }
}

/// Check if the given argument is a Mankai number.
pub fn is_number(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity. {
    if arguments.len() != 1 {
        return Err(RuntimeError::new(
            "'number?' requires exactly one argument!",
        ));
    }

    match arguments.get(0).unwrap() {
        MankaiObject::Number(_) => Ok(MankaiObject::Bool(true)),
        _ => Ok(MankaiObject::Bool(false)),
    }
}

/// Check if the given argument is a Mankai string.
pub fn is_string(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity. {
    if arguments.len() != 1 {
        return Err(RuntimeError::new(
            "'string?' requires exactly one argument!",
        ));
    }

    match arguments.get(0).unwrap() {
        MankaiObject::String(_) => Ok(MankaiObject::Bool(true)),
        _ => Ok(MankaiObject::Bool(false)),
    }
}

/// Create a new Mankai list from the given Mankai objects.
pub fn list(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    let mut list = Vec::new();

    for object in arguments {
        list.push(object);
    }

    Ok(MankaiObject::List(list))
}

/// Logic NOR.
pub fn not(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.len() != 1 {
        return Err(RuntimeError::new("'not' requires exactly one argument!"));
    }

    match arguments.get(0).unwrap() {
        MankaiObject::Bool(true) => Ok(MankaiObject::Bool(false)),
        MankaiObject::Bool(false) => Ok(MankaiObject::Bool(true)),
        _ => Err(RuntimeError::new("1st argument to 'not' is not a boolean!")),
    }
}

/// Logic OR with unfixed arity.
pub fn or(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'or' requires at least one argument!"));
    }

    // Perform OR.
    for (i, value) in arguments.iter().enumerate() {
        match value {
            MankaiObject::Bool(true) => return Ok(MankaiObject::Bool(true)),
            MankaiObject::Bool(false) => (),
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument to 'or' is nor a boolean!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Bool(false))
}

/// Concatenate strings.
pub fn string_concat(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new(
            "'string-concat' requires at least one argument!",
        ));
    }

    // Perform concatenation.
    let mut result = String::new();
    for (i, value) in arguments.iter().enumerate() {
        match value {
            MankaiObject::String(s) => result.push_str(s),
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument must be a string!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::String(result))
}

/// Convert a mankai object to a Mankai string.
pub fn to_string(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.len() != 1 {
        return Err(RuntimeError::new(
            "'to-string' requires exactly one argument!",
        ));
    }

    // Perform conversion.
    let value = arguments.get(0).unwrap();
    match value {
        MankaiObject::String(_) => Ok(value.clone()),
        _ => Ok(MankaiObject::String(value.to_string())),
    }
}
