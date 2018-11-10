use std::cmp::min;
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(PartialEq, Hash, Eq, Debug)]
struct BucketStates {
    pub moves: u8,
    pub fullness: (u8, u8),
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let mut pours = VecDeque::<BucketStates>::new();
    let mut hit_states = HashSet::new();
    pours.push_back(match start_bucket {
        Bucket::One => {
            hit_states.insert((0, capacity_2));
            BucketStates {
                moves: 0,
                fullness: (capacity_1, 0),
            }
        }
        Bucket::Two => {
            hit_states.insert((capacity_1, 0));
            BucketStates {
                moves: 0,
                fullness: (0, capacity_2),
            }
        }
    });

    while let Some(state) = pours.pop_front() {
        let moves = state.moves + 1;
        
        // check if current state is the goal
        if state.fullness.0 == goal {
            return BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: state.fullness.1,
            };
        } else if state.fullness.1 == goal {
            return BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: state.fullness.0,
            };
        }

        // Prevent revisiting state
        if hit_states.contains(&state.fullness) {
            continue;
        }
        hit_states.insert(state.fullness);

        // Dump bucket
        pours.push_back(BucketStates {
            moves,
            fullness: (state.fullness.0, 0),
        });
        pours.push_back(BucketStates {
            moves,
            fullness: (0, state.fullness.1),
        });

        // Fill bucket
        pours.push_back(BucketStates {
            moves,
            fullness: (state.fullness.0, capacity_2),
        });
        pours.push_back(BucketStates {
            moves,
            fullness: (capacity_1, state.fullness.1),
        });

        // Transfer bucket
        let total = state.fullness.0 + state.fullness.1;
        pours.push_back(BucketStates {
            moves,
            fullness: (
                total.checked_sub(capacity_2).unwrap_or(0),
                min(total, capacity_2),
            ),
        });
        pours.push_back(BucketStates {
            moves,
            fullness: (
                min(total, capacity_1),
                total.checked_sub(capacity_1).unwrap_or(0),
            ),
        });
    }
    unreachable!();
}
