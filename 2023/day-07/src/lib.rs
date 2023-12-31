use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Hash)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    pub fn new(input: String, use_jokers: bool) -> Self {
        let split = input.split_once(' ').unwrap();
        let (cards, bid) = (split.0.to_string(), split.1.parse().unwrap());
        let cards = cards
            .chars()
            .map(|c| Card::from_char(&c, use_jokers))
            .collect();
        let hand_type = HandType::from_hand(&cards);

        Hand {
            cards,
            hand_type,
            bid,
        }
    }
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn hand_type(&self) -> HandType {
        self.hand_type
    }

    pub fn bid(&self) -> usize {
        self.bid
    }

    pub fn compare(&self, other: &Hand) -> Ordering {
        if self.hand_type() > other.hand_type() {
            Ordering::Less
        } else if self.hand_type() < other.hand_type() {
            Ordering::Greater
        } else {
            for (card, other_card) in self.cards().iter().zip(other.cards().iter()) {
                match card.cmp(other_card) {
                    Ordering::Greater => {
                        return Ordering::Less
                    },
                    Ordering::Less => {
                        return Ordering::Greater
                    },
                    _ => {},
                }
            }
            Ordering::Equal
        }
    }
}

impl Card {
    pub fn from_char(c: &char, use_jokers: bool) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' if use_jokers => Card::Joker,
            'J' if !use_jokers => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown card {c}"),
        }
    }
}

impl HandType {
    pub fn from_hand(hand: &Vec<Card>) -> Self {
        let mut cards: HashMap<Card, usize> = HashMap::new();

        for c in hand {
            *cards.entry(c.clone()).or_insert(0) += 1;
        }

        // Get a count of jokers in the hand. Jokers, for purposes of determining the type of hand
        // count as the card that will make the hand the best possible type
        let sum_jokers = *cards.get(&Card::Joker).unwrap_or(&0);

        if sum_jokers > 0 {
            cards.remove(&Card::Joker);
        }

        // If we have jokers, add them to the count of the highest frequency card found.
        let max = *cards
            .iter()
            .max_by_key(|e| e.1)
            .map(|e| e.1)
            .unwrap_or(&0)
            + sum_jokers;
        let min = *cards
            .iter()
            .min_by_key(|e| e.1)
            .map(|e| e.1)
            .unwrap_or(&0);

        let min_sum = cards.iter().fold(0, |acc, e| {
            // get a count of how many single cards we have
            acc + {
                if *e.1 == 1 {
                    1
                } else {
                    0
                }
            }
        }) as usize;

        match max {
            1 => HandType::HighCard,
            2 => {
                // We have only 1 solo card, meaning we have 2 pairs.
                if min_sum == 1 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            3 => {
                if min == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => panic!("Unknown hand type {hand:?}"),
        }
    }
}
