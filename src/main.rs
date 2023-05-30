mod clause;
mod clause_set;

/// Bitfield of n bits supporting up to n variables
type Word = u32;

fn main() {
    // 3.1
    unsat_test(&["a, !c", "b, !a", "!b", "a, c"]);
    // 3.2
    unsat_test(&["a, !b, !c, !d", "a, b", "a, c", "a, d", "!a"]);
    // 4.1
    unsat_test(&["a, !c", "b, !a", "!b", "a, c"]);
    // 4.2
    unsat_test(&["a", "a, !b, !c", "!a, d, !e", "!b, !e", "c, !d, !e", "e"]);
}

fn unsat_test(clause_strings: &[&str]) {
    use clause::Clause as C;
    use clause_set::ClauseSet as S;

    let mut set = S::from(
        clause_strings
            .into_iter()
            .map(|string| C::from(*string))
            .collect::<Vec<C>>(),
    );

    println!("input: {}", set);

    if let Some(proof) = set.proof_unsat() {
        println!("proof:");
        for clause in proof {
            println!("{}", clause);
        }
    } else {
        println!("proof not found");
    }

    println!();
}
