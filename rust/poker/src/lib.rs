use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Rank = u8;
type Suit = char;
type Card = (Rank, Suit);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    FiveOfAKind = 9,
    StraightFlush = 8,
    FourOfAKind = 7,
    FullHouse = 6,
    Flush = 5,
    Straight = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(hand: &str) -> Self {
        let cards = hand
            .split(' ')
            .map(|card| {
                let (rank, suit) = card.split_at(card.len() - 1);
                let rank = match rank {
                    "A" => 14,
                    "K" => 13,
                    "Q" => 12,
                    "J" => 11,
                    _ if rank.parse::<u8>().map(|r| (2..=10).contains(&r)) == Ok(true) => {
                        rank.parse::<u8>().unwrap()
                    }
                    _ => panic!("invalid card {}, rank unknown", card),
                };
                let suit = match suit {
                    "D" | "C" | "S" | "H" => suit.chars().next().unwrap(),
                    _ => panic!("invalid card {}, suit unknown", card),
                };
                (rank, suit)
            })
            .collect();
        Self { cards }
    }

    /// Returns an opaque key that can be used for comparing `Hand`s
    fn sort_key(&self) -> (HandRank, Vec<(usize, Rank)>) {
        let mut suits = HashSet::new();
        let mut ranks_counted = HashMap::new();
        for card in &self.cards {
            suits.insert(card.1);
            *ranks_counted.entry(card.0).or_insert(0) += 1;
        }
        let mut kind_rank: Vec<_> = ranks_counted.into_iter().map(|(r, c)| (c, r)).collect();
        kind_rank.sort();
        kind_rank.reverse();
        let unique_ranks = kind_rank.len();

        let is_normal_straight = unique_ranks == 5 && kind_rank[0].1 - kind_rank[4].1 == 4;
        let is_ace_low_straight = unique_ranks == 5 && kind_rank[0].1 - kind_rank[1].1 == 9;
        if is_ace_low_straight {
            kind_rank.swap(0, 4); // moves the ace from the high position, to low
        }
        let is_straight = is_normal_straight || is_ace_low_straight;
        let is_flush = suits.len() == 1;
        let most_copies = kind_rank[0].0;

        let hand_ranking = match unique_ranks {
            1 => HandRank::FiveOfAKind, // impossible w/o joker
            5 if is_flush && is_straight => HandRank::StraightFlush,
            2 if most_copies == 4 => HandRank::FourOfAKind,
            2 if most_copies == 3 => HandRank::FullHouse,
            5 if is_flush => HandRank::Flush,
            5 if is_straight => HandRank::Straight,
            3 if most_copies == 3 => HandRank::ThreeOfAKind,
            3 if most_copies == 2 => HandRank::TwoPair,
            4 => HandRank::Pair,
            _ => HandRank::HighCard,
        };
        (hand_ranking, kind_rank)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.sort_key() == other.sort_key()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.sort_key().cmp(&other.sort_key()))
    }
}

/// Given a list of poker hands, return a list of those hands which win.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let best_hand = hands.iter().max_by_key(|hand| Hand::new(hand).sort_key());
    best_hand.map(|best| {
        let best = Hand::new(best);
        hands
            .iter()
            .filter(|h| Hand::new(h) == best)
            .cloned()
            .collect()
    })
}
