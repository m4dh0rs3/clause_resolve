use super::Word;

/// Contains literal of variable n if nth bit is 1
#[derive(Clone, PartialEq)]
pub(crate) struct Clause {
    pos: Word,
    neg: Word,
}

impl Clause {
    /// Resolve two clauses in `O(|VAR|)`
    pub(crate) fn res(mut c0: Self, mut c1: Self) -> Option<Self> {
        let mut mask;
        if {
            // All complementing literals
            mask = c0.pos & c1.neg;
            // If there are none
            mask == 0
        } {
            // Switch out variables and check again
            let tmp = c0;
            c0 = c1;
            c1 = tmp;

            if {
                mask = c0.pos & c1.neg;
                mask == 0
            } {
                return None;
            }
        }

        // Returns the next smaller power of two,
        // which is the first contained literal
        // and invert this for masking
        mask = !(2 as Word).pow(mask.trailing_zeros());

        // Remove said literal
        c0.pos &= mask;
        c1.neg &= mask;

        // Finally return all literals
        // with one complement filtered out
        Some(Clause::union(c0, c1))
    }

    /// Union of the set of literals
    fn union(c0: Self, c1: Self) -> Self {
        Self {
            pos: c0.pos | c1.pos,
            neg: c0.neg | c1.neg,
        }
    }

    /// Create new empty clause
    pub(crate) fn new_empty() -> Self {
        Self { pos: 0, neg: 0 }
    }

    /// Check if clause is empty
    pub(crate) fn is_empty(&self) -> bool {
        self.pos == 0 && self.neg == 0
    }
}

impl From<&str> for Clause {
    /// Parse a sequence of literals with
    /// characters `'A'..='z'` as variables
    fn from(string: &str) -> Self {
        let mut clause = Self::new_empty();
        let mut negate = false;

        for c in string.chars() {
            match c {
                '!' | '-' | '~' | '¬' => negate = true,
                'A'..='z' => {
                    let index = c as u32 - 97;
                    if negate {
                        clause.neg |= 1 << index;
                        negate = false;
                    } else {
                        clause.pos |= 1 << index;
                    }
                }
                _ => continue,
            }
        }

        clause
    }
}

use std::fmt;

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::mem;

        if self.is_empty() {
            write!(f, "◻")
        } else {
            write!(f, "{{")?;

            let mut pos = Vec::with_capacity((self.pos.count_ones()) as usize);
            let mut neg = Vec::with_capacity((self.neg.count_ones()) as usize);

            for i in 0..mem::size_of::<Word>() * 8 {
                if ((self.pos >> i) & 1) == 1 {
                    pos.push(i)
                }

                if ((self.neg >> i) & 1) == 1 {
                    neg.push(i)
                }
            }

            for (i, var) in pos.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }

                write!(
                    f,
                    "{}",
                    char::from_u32(*var as u32 + 97).expect("could not display variable")
                )?;
            }

            if !pos.is_empty() && !neg.is_empty() {
                write!(f, ", ")?;
            }

            for (i, var) in neg.iter().enumerate() {
                write!(
                    f,
                    "¬{}",
                    char::from_u32(*var as u32 + 97).expect("could not display variable")
                )?;

                if i != neg.len() - 1 {
                    write!(f, ", ")?;
                }
            }

            write!(f, "}}")
        }
    }
}

impl fmt::Debug for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C {{ pos: {:0b}, neg: {:0b} }}", self.pos, self.neg)
    }
}

impl Clause {
    #[deprecated(note = "replaced by `res`")]
    pub(crate) fn res_deprc(mut c0: Self, mut c1: Self) -> Option<Self> {
        /// Returns the next smaller power of two
        /// which is the first contained variable
        fn prev_power_of_two(word: Word) -> Word {
            (2 as Word).pow(word.trailing_zeros())
        }

        let mut mask;

        if {
            // All complementing literals
            mask = c0.pos & c1.neg;
            // If there are some
            mask != 0
        } {
            // Only resolve one literal
            mask = !prev_power_of_two(mask);

            // Remove said literal
            c0.pos &= mask;
            c1.neg &= mask;

            Some(Clause::union(c0, c1))
        } else if {
            // Also check for neg-pos pair
            mask = c0.neg & c1.pos;
            mask != 0
        } {
            mask = !prev_power_of_two(mask);

            c0.neg &= mask;
            c1.pos &= mask;

            Some(Clause::union(c0, c1))
        } else {
            None
        }
    }
}
