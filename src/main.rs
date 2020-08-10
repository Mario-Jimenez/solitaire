mod card;
mod deck;
mod input;
mod moves;

/*
Piles:
The Tableau: Seven piles that make up the main table.
The Foundations: Four piles on which a whole suit or sequence must be built up.
    In most Solitaire games, the four aces are the bottom card or base of the foundations.
    The foundation piles are hearts, diamonds, spades, and clubs.
The Stock (or “Hand”) Pile: If the entire pack is not laid out in a tableau at the beginning
    of a game, the remaining cards form the stock pile from which additional cards are brought
    into play according to the rules.
The Talon (or “Waste”) Pile: Cards from the stock pile that have no place in the tableau or
    on foundations are laid face up in the waste pile.
*/

/*
Empty column
If a vacancy in the tableau is created by the removal of cards elsewhere it is called a “space”,
    and it is of major importance in manipulating the tableau. If a space is created, it can
    only be filled in with a king. Filling a space with a king could potentially unblock one
    of the face down cards in another pile in the tableau.
*/

fn main() {
    let mut piles = deck::set_up();
    deck::print_piles(&piles);

    input::actions(&mut piles);
}
