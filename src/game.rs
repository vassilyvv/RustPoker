use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Rank {
    Ace,
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
}

#[derive(Debug, Clone, Copy)]
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
        self.cards.iter().fold("Cards: ".to_string(), |line, &card| -> String {
            let string = card.to_string();
            line + string.as_str()
        })
    }
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}