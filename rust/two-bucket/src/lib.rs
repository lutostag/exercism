use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::ops::Not;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One = 0,
    Two = 1,
}

impl Not for Bucket {
    type Output = Bucket;
    fn not(self) -> Self::Output {
        match self {
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::One,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

#[derive(Debug, Copy, Clone)]
struct Buckets {
    pub moves: u8,
    pub buckets: [(u8, u8); 2],
}

impl Buckets {
    fn setup(capacity_1: u8, capacity_2: u8, start_full: Bucket) -> Self {
        let state = Buckets {
            moves: 0,
            buckets: [(capacity_1, 0), (capacity_2, 0)],
        };
        state.fill(start_full)
    }

    fn dump(&self, bucket: Bucket) -> Self {
        let mut buckets = self.buckets.clone();
        buckets[bucket as usize].1 = 0;

        Buckets {
            moves: self.moves + 1,
            buckets,
        }
    }

    fn fill(&self, bucket: Bucket) -> Self {
        let mut buckets = self.buckets.clone();
        buckets[bucket as usize].1 = buckets[bucket as usize].0;

        Buckets {
            moves: self.moves + 1,
            buckets,
        }
    }

    fn transfer_to(&self, bucket_idx: Bucket) -> Self {
        let mut buckets = self.buckets.clone();
        let bucket = buckets[bucket_idx as usize];
        let other = buckets[!bucket_idx as usize];
        let total = bucket.1 + other.1;
        buckets[bucket_idx as usize].1 = min(total, bucket.0);
        buckets[!bucket_idx as usize].1 = total.checked_sub(bucket.0).unwrap_or(0);

        Buckets {
            moves: self.moves + 1,
            buckets,
        }
    }

    fn is_illegal(&self, start_bucket: Bucket) -> bool {
        let bucket = self.buckets[start_bucket as usize];
        let other = self.buckets[!start_bucket as usize];
        bucket.1 == 0 && other.1 == other.0
    }

    fn goal(&self, amount: u8) -> Option<BucketStats> {
        for &bucket in &[Bucket::One, Bucket::Two] {
            if self.buckets[bucket as usize].1 == amount {
                return Some(BucketStats {
                    moves: self.moves,
                    goal_bucket: bucket,
                    other_bucket: self.buckets[!bucket as usize].1,
                });
            }
        }
        None
    }
}

/// Solve the bucket problem use a BFS
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let mut pours = VecDeque::<Buckets>::new();
    let mut already_tried = HashSet::new();
    pours.push_back(Buckets::setup(capacity_1, capacity_2, *start_bucket));

    while let Some(buckets) = pours.pop_front() {
        if buckets.is_illegal(*start_bucket) {
            continue;
        }
        if let Some(stats) = buckets.goal(goal) {
            return stats;
        }

        if already_tried.insert(buckets.buckets) {
            pours.push_back(buckets.dump(Bucket::One));
            pours.push_back(buckets.dump(Bucket::Two));
            pours.push_back(buckets.fill(Bucket::One));
            pours.push_back(buckets.fill(Bucket::Two));
            pours.push_back(buckets.transfer_to(Bucket::One));
            pours.push_back(buckets.transfer_to(Bucket::Two));
        }
    }
    unreachable!();
}
