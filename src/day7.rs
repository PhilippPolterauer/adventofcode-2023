use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
#[derive(Debug, PartialEq, Eq)]
struct Card {
    char: char,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.char == other.char {
            return Ordering::Equal;
        }
        match (self.char, other.char) {
            ('A', _) => Ordering::Greater,
            ('K', 'A') => Ordering::Less,
            ('K', _) => Ordering::Greater,
            ('Q', 'A') => Ordering::Less,
            ('Q', 'K') => Ordering::Less,
            ('Q', _) => Ordering::Greater,
            ('J', 'A') => Ordering::Less,
            ('J', 'K') => Ordering::Less,
            ('J', 'Q') => Ordering::Less,
            ('J', _) => Ordering::Greater,
            ('T', 'A') => Ordering::Less,
            ('T', 'K') => Ordering::Less,
            ('T', 'Q') => Ordering::Less,
            ('T', 'J') => Ordering::Less,
            ('T', _) => Ordering::Greater,
            (a, b) => a.cmp(&b),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: i64,
}
impl Hand {
    fn parse(line: &str) -> Self {
        let cards: Vec<Card> = line
            .split_whitespace()
            .next()
            .unwrap()
            .chars()
            .map(|char| Card { char })
            .collect();
        let bid = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i64>()
            .unwrap();
        Hand { cards: cards, bid }
    }
    fn get_counts(&self) -> Vec<i64> {
        let mut card_count: HashMap<&char, i64> = HashMap::new();
        for card in self.cards.iter() {
            if let Some(count) = card_count.get_mut(&card.char) {
                *count += 1;
            } else {
                card_count.insert(&card.char, 1);
            }
        }
        card_count.into_values().collect()
    }
    fn get_type(&self) -> HandType {
        let mut counts = self.get_counts();
        counts.sort();
        counts.reverse();

        if counts[0] == 5 {
            HandType::FiveOfAKind
        } else if counts[0] == 4 {
            HandType::FourOfAKind
        } else if counts[0] == 3 && counts[1] == 2 {
            HandType::FullHouse
        } else if counts[0] == 3 {
            HandType::ThreeOfAKind
        } else if counts[0] == 2 && counts[1] == 2 {
            HandType::TwoPair
        } else if counts[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.get_type(), &self.cards).cmp(&(other.get_type(), &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: String) {
    let mut hands: Vec<Hand> = input.lines().map(|l| Hand::parse(l)).collect();
    hands.sort();
    let mut solution = 0;
    for (rank, hand) in hands.iter().enumerate() {
        // dbg!(&hand);
        solution += hand.bid * (rank as i64 + 1)
    }
    dbg!(solution);
}
pub fn part2(input: String) {}
