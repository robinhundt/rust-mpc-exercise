use crate::circuit::Circuit;



struct Party {
    circuit: (),
    // also include a send and receive channel to other party
    // have a look at https://doc.rust-lang.org/std/sync/mpsc/index.html
    // for the std lib provided in-memory channel.
}


/// Creates a new pair of parties for the provided circuit that can communicate with each other
/// to execute the provided circuit.
pub fn new_party_pair(circuit: Circuit) -> (Party, Party) {
    todo!("setup and return parties")
}


impl Party {
    /// Create a new party.
    pub fn new(circuit: (), other_args: ()) -> Self {
        todo!()
    }

    /// Executes the GMW protocol with the linked party for the stored circuit.
    pub fn execute(&mut self, input: Vec<bool>) -> Result<Vec<bool>, ()> { // TODO change error type
        // Iterate over the stored circuit in topological order. `match` on the gate type and
        // evaluate it, potentially using a multiplication triple for and And Gate and communication
        // over the shared channel.
        todo!()
    }
}