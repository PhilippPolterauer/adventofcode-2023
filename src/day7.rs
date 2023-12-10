use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;

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
struct CardN {
    char: char,
}
#[derive(PartialEq, Eq)]
struct CardJ {
    char: char,
}
impl Debug for CardJ {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.char.to_string())
    }
}

trait Card {
    fn from_char(char: char) -> Self;
    fn char(&self) -> &char;
}
impl Card for CardN {
    fn from_char(char: char) -> Self {
        Self { char }
    }
    fn char(&self) -> &char {
        &self.char
    }
}
impl Card for CardJ {
    fn from_char(char: char) -> Self {
        Self { char }
    }
    fn char(&self) -> &char {
        &self.char
    }
}

impl Ord for CardN {
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
impl Ord for CardJ {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.char == other.char {
            return Ordering::Equal;
        }
        match (self.char, other.char) {
            ('A', _) => Ordering::Greater,
            ('J', _) => Ordering::Less,
            ('K', 'A') => Ordering::Less,
            ('K', _) => Ordering::Greater,
            ('Q', 'A') => Ordering::Less,
            ('Q', 'K') => Ordering::Less,
            ('Q', _) => Ordering::Greater,
            ('T', 'A') => Ordering::Less,
            ('T', 'K') => Ordering::Less,
            ('T', 'Q') => Ordering::Less,
            ('T', _) => Ordering::Greater,
            (_, 'J') => Ordering::Greater,
            (a, b) => a.cmp(&b),
        }
    }
}

impl PartialOrd for CardN {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for CardJ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait CardCount {
    fn get_counts(&self) -> Vec<i64>;
}

impl CardCount for Vec<CardN> {
    fn get_counts(&self) -> Vec<i64> {
        let mut card_count: HashMap<&char, i64> = HashMap::new();
        for card in self {
            if let Some(count) = card_count.get_mut(card.char()) {
                *count += 1;
            } else {
                card_count.insert(card.char(), 1);
            }
        }
        let mut counts: Vec<i64> = card_count.into_values().collect();
        counts.sort();
        counts.reverse();
        counts
    }
}
impl CardCount for Vec<CardJ> {
    fn get_counts(&self) -> Vec<i64> {
        let mut card_count: HashMap<&char, i64> = HashMap::new();
        for card in self {
            if let Some(count) = card_count.get_mut(card.char()) {
                *count += 1;
            } else {
                card_count.insert(card.char(), 1);
            }
        }

        let joker = CardJ::from_char('J');
        let jokers = card_count.remove(joker.char());

        let mut counts: Vec<i64> = card_count.into_values().collect();

        counts.sort();
        counts.reverse();

        if let Some(jokers) = jokers {
            if let Some(val) = counts.get_mut(0) {
                *val += jokers;
            } else {
                counts.push(jokers);
            }
        }
        counts
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<T>
where
    T: Card,
{
    cards: Vec<T>,
    bid: i64,
}
impl<T> Hand<T>
where
    T: Card,
    Vec<T>: CardCount,
{
    fn parse(line: &str) -> Self {
        let cards: Vec<T> = line
            .split_whitespace()
            .next()
            .unwrap()
            .chars()
            .map(|char| T::from_char(char))
            .collect();
        let bid = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i64>()
            .unwrap();
        Hand { cards, bid }
    }

    fn get_type(&self) -> HandType {
        let counts = self.cards.get_counts();

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
impl<T> Ord for Hand<T>
where
    T: Card + Eq + Ord,
    Vec<T>: CardCount,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (self.get_type(), &self.cards).cmp(&(other.get_type(), &other.cards))
    }
}

impl<T> PartialOrd for Hand<T>
where
    T: Card + PartialEq + Ord,
    Vec<T>: CardCount,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> i64 {
    let mut hands: Vec<Hand<CardN>> = input.lines().map(Hand::parse).collect();
    hands.sort();
    let mut solution = 0;
    for (rank, hand) in hands.iter().enumerate() {
        // &hand;
        solution += hand.bid * (rank as i64 + 1)
    }
    solution
}
pub fn part2(input: &str) -> i64 {
    let mut hands: Vec<Hand<CardJ>> = input.lines().map(Hand::parse).collect();
    hands.sort();
    let mut solution = 0;
    for (rank, hand) in hands.iter().enumerate() {
        solution += hand.bid * (rank as i64 + 1)
    }
    solution
}
