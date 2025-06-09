use crate::object::Object;
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        // --- PRE-LOAD BUILT-IN FUNCTIONS ---
        store.insert(
            "print".to_string(),
            Object::Builtin(crate::object::Builtin {
                func: super::builtins::builtin_print,
            }), // Assuming builtins.rs
        );
        store.insert(
            "len".to_string(),
            Object::Builtin(crate::object::Builtin {
                func: super::builtins::builtin_length,
            }),
        );

        Self { store }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }
}
