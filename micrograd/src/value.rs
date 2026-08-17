use std::Collections::HashSet;

struct Value {
    data:f64,
    grad:f64,
    _backward: None,
    _prev: HashSet,
    _op: f64
}