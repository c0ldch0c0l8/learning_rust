// primitives, aka fully stack vars, always use COPY
// other types use either MOVE or .clone(). MOVE is default
// behavior and so it's implied, just like COPY is for 
// primitives, but .clone() is explicit. shallow and deep
// copying for primitives are == and so its 1 COPY for all
// while shallow and deep copying are different for adv types.
// MOVE = a shallow copy + removal of first var to ensure no 2
// owners?? for a value
//
// References are easy. &object and u have a reference to it
// that you can use in a function without moving ownership.
// Either 1 mutable reference or any number of immutable 
// references, and references must be valid (not dangling)
//
// slices are slices of data. ptr from w to z (including x and y)
// can have ptr[x..], ptr[..y], etc

pub fn func(string: &str) -> &str {
    
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }

    &string[..]
}