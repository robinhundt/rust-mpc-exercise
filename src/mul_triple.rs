

/// A MulTriple (short for multiplication triple) is used to efficiently perform a multiplication
/// of secret values in the online phase of the GMW protocol. A MulTriple comprises the random values
/// a,b,c in {0,1} s.t. c = a & b. These random values are secret-shared between the parties, so e.g.
/// Party 0 has [a]_0, [b]_0, and [c]_0 with [a]_0 ^ [a]_1 = a (likewise for b and c).
pub struct MulTriple {
    // secret-shared parts of multiplication triple. So a is [a]_i for Party i
    pub a: (),
    pub b: (),
    pub c: ()
}


/// The MTProvider trait abstracts over different implementations of generating MulTriples. A trivial
/// implementation always returns a = 0, b = 0, c = 0, as 0 ^ 0 = (0 ^ 0) & (0 ^ 0).
/// A slightly more realistic implementation could sample triples based on a shared seed used for
/// a PRNG. Both parties have an MTProvider with the same shared seed. The PRNG is used to randomly
/// sample [a]_i, [b]_i, and [c]_i (which have no sub-script in the MulTriple struct).
/// Because these values are the same for both parties, we end up with a = 0, b = 0, c = 0, which
/// fulfills the multiplication triple property (but is still insecure!).
trait MTProvider {

    fn get_triple(&mut self) -> MulTriple {
        todo!()
    }
}