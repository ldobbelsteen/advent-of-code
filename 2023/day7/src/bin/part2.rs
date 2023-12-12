#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::{char, collections::HashMap, fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Card {
    Ace,
    King,
    Queen,
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

impl Card {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            'J' => Ok(Self::Joker),
            _ => Err(anyhow!("char is not a valid card: {}", c)),
        }
    }

    fn rank(&self) -> u32 {
        match self {
            Self::Ace => 12,
            Self::King => 11,
            Self::Queen => 10,
            Self::Ten => 9,
            Self::Nine => 8,
            Self::Eight => 7,
            Self::Seven => 6,
            Self::Six => 5,
            Self::Five => 4,
            Self::Four => 3,
            Self::Three => 2,
            Self::Two => 1,
            Self::Joker => 0,
        }
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut jokers = 0;
        let mut non_joker_occs: HashMap<Card, u32> = HashMap::new();

        for card in cards {
            if card == &Card::Joker {
                jokers += 1;
            } else {
                non_joker_occs
                    .entry(card.clone())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }

        let (max, max_occs) = non_joker_occs
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap_or((&Card::Joker, &0));

        match max_occs + jokers {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => {
                if non_joker_occs
                    .iter()
                    .any(|(card, occ)| card != max && *occ == 2)
                {
                    Self::FullHouse
                } else {
                    Self::ThreeOfAKind
                }
            }
            2 => {
                if non_joker_occs
                    .iter()
                    .any(|(card, occ)| card != max && *occ == 2)
                {
                    Self::TwoPair
                } else {
                    Self::OnePair
                }
            }
            _ => Self::HighCard,
        }
    }

    fn rank(&self) -> u32 {
        match self {
            Self::FiveOfAKind => 6,
            Self::FourOfAKind => 5,
            Self::FullHouse => 4,
            Self::ThreeOfAKind => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }
}

#[derive(Debug)]
struct Hand {
    bid: u32,
    cards: [Card; 5],
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (raw_cards, raw_bid) = s
            .split_once(' ')
            .ok_or(anyhow!("no space char in hand: {}", s))?;

        let bid = raw_bid.parse()?;
        let cards = raw_cards
            .chars()
            .map(Card::from_char)
            .collect::<Result<Vec<Card>>>()?
            .try_into()
            .map_err(|e| anyhow!("error while converting card vector to array: {:?}", e))?;
        let hand_type = HandType::from_cards(&cards);

        Ok(Self {
            bid,
            cards,
            hand_type,
        })
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;

    let mut hands = file
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<Hand>>>()?;

    hands.sort_by(|a, b| {
        let a_rank = a.hand_type.rank();
        let b_rank = b.hand_type.rank();
        if a_rank == b_rank {
            for i in 0..a.cards.len() {
                let a_card_rank = a.cards[i].rank();
                let b_card_rank = b.cards[i].rank();
                if a_card_rank != b_card_rank {
                    return a_card_rank.cmp(&b_card_rank);
                }
            }
            a.bid.cmp(&b.bid)
        } else {
            a_rank.cmp(&b_rank)
        }
    });

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum::<usize>();
    println!("{result}");

    Ok(())
}
