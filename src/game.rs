use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::vec::Vec;


#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, EnumString)]
pub enum Hand {
    High,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{:?} of {:?}", self.rank, self.suit)
    }
}

#[derive(Debug)]
pub struct Pack {
    pub cards: Vec<Card>,
}

impl Pack {
    pub fn new() -> Self {
        let mut pack = Pack { cards: Vec::new() };
        Suit::iter().for_each(|suit| {
            Rank::iter().for_each(|rank| { pack.cards.push(Card { rank, suit }) });
        });
        pack
    }
    pub fn print(&self) -> String {
        let amount_of_cards = 5;
        let mut result = self.cards[..amount_of_cards].iter().fold("Cards:\n".to_string(), |line, &card| -> String {
            let string = card.to_string();
            line + string.as_str()
        });
        result.push_str(format!("\n\n{:?}", get_combination(&self.cards[..amount_of_cards])).as_str());
        result
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

fn get_combination(set_of_cards: &[Card]) -> Hand {
    let mut sorted_set_of_cards = set_of_cards.into_iter().collect::<Vec<&Card>>();
    sorted_set_of_cards.sort_by(|card1, card2| card2.rank.cmp(&card1.rank));
    for card1 in sorted_set_of_cards.iter().filter(|card| card.rank as u8 > Rank::Five as u8).collect::<Vec<&&Card>>().iter() {
        if sorted_set_of_cards.iter().any(|card2| card2.rank as u8 == (card1.rank as u8) - 1 && card2.suit == card1.suit) {
            if sorted_set_of_cards.iter().any(|card3| card3.rank as u8 == (card1.rank as u8) - 2 && card3.suit == card1.suit) {
                if sorted_set_of_cards.iter().any(|card4| card4.rank as u8 == (card1.rank as u8) - 3 && card4.suit == card1.suit) {
                    if sorted_set_of_cards.iter().any(|card5| card5.rank as u8 == (card1.rank as u8) - 4 && card5.suit == card1.suit) {
                        if card1.rank == Rank::Ace {
                            return Hand::RoyalFlush;
                        }
                        return Hand::StraightFlush;
                    }
                }
            }
        }
    }

    for card1 in sorted_set_of_cards.iter() {
        if set_of_cards.iter().filter(|card2| card2.rank == card1.rank).count() > 3 {
            return Hand::FourOfAKind;
        }
    }

    for card1 in sorted_set_of_cards.iter() {
        if set_of_cards.iter().filter(|card2| card2.rank == card1.rank).count() == 2 {
            for card3 in sorted_set_of_cards.iter() {
                if set_of_cards.iter().filter(|card4| card4.rank == card3.rank).count() == 3 {
                    return Hand::FullHouse;
                }
            }
        }
    }

    for card1 in sorted_set_of_cards.iter() {
        if set_of_cards.iter().filter(|card2| card2.suit == card1.suit).count() > 4 {
            return Hand::Flush;
        }
    }

    for card1 in sorted_set_of_cards.iter().filter(|card| card.rank as u8 > Rank::Five as u8).collect::<Vec<&&Card>>() {
        if set_of_cards.iter().any(|card2| card2.rank as u8 == (card1.rank as u8) - 1) {
            if set_of_cards.iter().any(|card3| card3.rank as u8 == (card1.rank as u8) - 2) {
                if set_of_cards.iter().any(|card4| card4.rank as u8 == (card1.rank as u8) - 3) {
                    if set_of_cards.iter().any(|card5| card5.rank as u8 == (card1.rank as u8) - 4) {
                        return Hand::Straight;
                    }
                }
            }
        }
    };
    for card1 in sorted_set_of_cards.iter() {
        if set_of_cards.iter().filter(|card2| card1.rank == card2.rank).count() == 3 {
            return Hand::ThreeOfAKind;
        }
    }
    for card1 in sorted_set_of_cards.iter() {
        if set_of_cards.iter().filter(|card2| card2.rank == card1.rank).count() == 2 {
            for card3 in sorted_set_of_cards.iter() {
                if set_of_cards.iter().filter(|card4| card4.rank == card3.rank && card3.rank != card1.rank).count() == 2 {
                    return Hand::TwoPairs;
                }
            }
        }
    }
    for card1 in sorted_set_of_cards.iter() {
        if set_of_cards.iter().filter(|card2| card1.rank == card2.rank).count() == 2 {
            return Hand::Pair;
        }
    }
    Hand::High
}