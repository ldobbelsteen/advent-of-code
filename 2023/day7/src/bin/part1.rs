#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::{char, collections::HashMap, fs, str::FromStr};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Card {
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
}

impl Card {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(anyhow!("char is not a valid card: {}", c)),
        }
    }

    fn rank(&self) -> u32 {
        match self {
            Self::Ace => 12,
            Self::King => 11,
            Self::Queen => 10,
            Self::Jack => 9,
            Self::Ten => 8,
            Self::Nine => 7,
            Self::Eight => 6,
            Self::Seven => 5,
            Self::Six => 4,
            Self::Five => 3,
            Self::Four => 2,
            Self::Three => 1,
            Self::Two => 0,
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
        let mut occs: HashMap<Card, u32> = HashMap::new();
        for card in cards {
            occs.entry(card.clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        if occs.iter().any(|(_, occ)| *occ == 5) {
            return Self::FiveOfAKind;
        }

        if occs.iter().any(|(_, occ)| *occ == 4) {
            return Self::FourOfAKind;
        }

        if occs.iter().any(|(_, occ)| *occ == 3) {
            if occs.iter().any(|(_, occ)| *occ == 2) {
                return Self::FullHouse;
            }
            return Self::ThreeOfAKind;
        }

        if occs.iter().any(|(_, occ)| *occ == 2) {
            let first_card = occs
                .iter()
                .find_map(|(card, occ)| if *occ == 2 { Some(card) } else { None })
                .unwrap();
            let second_card = occs.iter().find_map(|(card, occ)| {
                if *occ == 2 && card != first_card {
                    Some(card)
                } else {
                    None
                }
            });

            if second_card.is_none() {
                return Self::OnePair;
            }
            return Self::TwoPair;
        }

        Self::HighCard
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
    htype: HandType,
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
            htype: hand_type,
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
        let a_rank = a.htype.rank();
        let b_rank = b.htype.rank();
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
        .map(|(i, hand)| (i + 1) * (hand.bid as usize))
        .sum::<usize>();
    println!("{result}");

    Ok(())
}
