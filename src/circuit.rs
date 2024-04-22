

// An enum is a good choice for a type which can be of either one of several variants.
// Since there is a fixed choice of gate types in a Bristol circuit, an enum is a natural
// way to represent it.
// A rust enum is similar to a tagged union in C/C++.
pub enum Gate {
    // Enum variants can have fields themselves.
    AGateType {
        with_fields: usize
    },
    // Or no fields.
    AnotherGate
}


// We can 'derive' some traits like Debug and Clone on types via a derive attribute. This is a
// macro which expands to the corresponding trait implementation of the trait.
// cargo-expand (https://github.com/dtolnay/cargo-expand) can show you the expanded code.
#[derive(Debug, Clone)]
pub struct Header {
    // Header information of a bristol circuit
}

pub struct Circuit {
    // a circuit consists of a header and the gates of a circuit
}

impl Circuit {
    /// Parses the bristol file contents into a circuit
    pub fn parse(circuit: &str) -> Self {
        // This method parses the circuit string representation into the Circuit type
        todo!()
    }
}


// A `#[cfg(test)]` marks the following block as conditionally included only for test builds.
// cfg directives can achieve similar things as preprocessor directives in C/C++.
#[cfg(test)]
mod tests {

    // Functions marked with `#[test]` are automatically run when you execute `cargo test`.
    #[test]
    fn test() {
        todo!("Writing tests is good!")
    }

}