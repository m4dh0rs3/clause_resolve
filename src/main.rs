mod clause;
mod clause_set;

/// Bitfield of n bits supporting up to n variables;
/// Size is number of variables `|VAR|`
type Word = u8;

fn main() {
    use clause_set::ClauseSet as S;

    // 3.1
    println!(
        "{}",
        S::from(
            "a -c
             b -a
             -b
             a c",
        )
        .proof_unsat()
    );

    // 3.2
    println!(
        "\n{}",
        S::from(
            "a -b -c -d
             a b
             a c
             a d
             -a",
        )
        .proof_unsat()
    );

    // 6.1
    println!(
        "\n{}",
        S::from(
            "-a -b
             a c -d
             c d
             a -c
             -a b",
        )
        .proof_unsat()
    );

    // 6.2
    println!(
        "\n{}",
        S::from(
            "c -d
             a b
             -b -c
             -a b
             d -b c",
        )
        .proof_unsat()
    );
}
