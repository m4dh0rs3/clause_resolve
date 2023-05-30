mod clause;
mod clause_set;

/// Bitfield of n bits supporting up to n variables
type Word = u32;

fn main() {
    use clause::Clause as C;
    use clause_set::ClauseSet as S;

    let mut set = S::from(vec![
        C::from("a c !d"),
        C::from("c d"),
        C::from("a !c"),
        C::from("!a !b"),
        C::from("!a b"),
    ]);

    if let Some(proof) = set.proof_unsat() {
        for clause in proof {
            println!("{}", clause);
        }
    } else {
        println!("proof not found");
    }
}
