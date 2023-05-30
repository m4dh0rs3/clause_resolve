use crate::clause::Clause;

/// Contains further resolvents of initial clauses
/// with references to their indices
pub(crate) struct ClauseSet(Vec<(usize, usize, Clause)>);

impl ClauseSet {
    /// Resolve all clauses starting from index
    fn res_batch(&self, from: usize) -> Vec<(usize, usize, Clause)> {
        let len = self.0.len();
        // Batch has at most this many new resolvents
        let mut batch = Vec::with_capacity((len - from) * (len - 1));

        'outer: for i in from..len {
            for j in 0..len {
                if i != j {
                    if let Some(res) = Clause::res(self.0[i].2.clone(), self.0[j].2.clone()) {
                        // println!("{}", res);
                        if res.is_empty() {
                            batch.push((i, j, res));
                            // Found empty clause
                            break 'outer;
                        } else {
                            batch.push((i, j, res));
                        }
                    }
                }
            }
        }

        batch
    }

    /// Resolve until either empty clause
    /// is contained or no new clauses can
    /// be resolved
    fn res(&mut self) -> bool {
        // No clauses to resolve
        if self.0.is_empty() {
            return false;
        }

        // Was already resolved
        if self.0[self.0.len() - 1].2.is_empty() {
            return true;
        }

        // Previous length
        let mut len = 0;

        loop {
            let batch = self.res_batch(len);
            len = self.0.len();

            for (i, j, clause) in batch {
                if let None = self.0.iter().find(|(_, _, c)| c == &clause) {
                    self.0.push((i, j, clause));
                }
            }

            // Now contains empty clause
            if self.0[len - 1].2.is_empty() {
                return true;
            }

            // No new clauses
            if len == self.0.len() {
                return false;
            }
        }
    }

    /// Try proving unsatisfiability by constructing
    /// the empty clause via clause resolution
    pub(crate) fn proof_unsat(&mut self) -> Option<Vec<Clause>> {
        // If resolution finds empty clause
        if self.res() {
            let empty = &self.0[self.0.len() - 1];
            let mut proof = self.construct_unsat_proof(empty.0, empty.1);
            proof.push(Clause::new_empty());

            Some(proof)
        } else {
            None
        }
    }

    /// Filter and sort the relevant clauses for the proof
    fn construct_unsat_proof(&self, i: usize, j: usize) -> Vec<Clause> {
        // Proof sequence contains at least the two clauses
        let mut seq = Vec::with_capacity(2);

        let c0 = &self.0[i];
        let c1 = &self.0[j];

        if c0.0 != c0.1 {
            seq.append(&mut self.construct_unsat_proof(c0.0, c0.1))
        }

        if c1.0 != c1.1 {
            seq.append(&mut self.construct_unsat_proof(c1.0, c1.1))
        }

        seq.push(c0.2.clone());
        seq.push(c1.2.clone());

        seq
    }
}

impl From<Vec<Clause>> for ClauseSet {
    fn from(vec: Vec<Clause>) -> Self {
        Self(
            vec.into_iter()
                .enumerate()
                .map(|(i, clause)| (i, i, clause))
                .collect(),
        )
    }
}

use std::fmt;

impl fmt::Display for ClauseSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (_, _, clause)) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", clause)?;
        }

        write!(f, "}}")
    }
}
