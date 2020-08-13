# solitaire
Simple solitaire game in rust.



## Game Info

### Piles

###### The Tableau (`1-7`)

Seven piles that make up the main table.

###### The Foundations (`q,w,e,r`)

Four piles on which a whole suit or sequence must be built up. In most Solitaire games, the four aces are the bottom card or base of the foundations. The foundation piles are hearts, diamonds, spades, and clubs.

###### The Stock (or “Hand”) Pile (`<Ret>`)

If the entire pack is not laid out in a tableau at the beginning of a game, the remaining cards form the stock pile from which additional cards are brought into play according to the rules.

###### The Talon (or “Waste”) Pile (`h`)

Cards from the stock pile that have no place in the tableau or on foundations are laid face up in the waste pile.



## Game Commands

Execute the following commands pressing the key with the same value:

**ESC**: Exit game.

**n/N**: Start new game with a random deck.

**RET**: Get a card from the **stock** and place it face up on the **talon**.

**h**: Move the card from the **talon**.

**1-7**: Make a move from the **tableau**.

**q,w,e,r**: Make a move from the **foundations**.

**u/U**: Undo last move.

**i/I**: Redo last move.



## Run project

Run project with `$ cargo run [game]` or with binary file `./solitaire [game]`.

- `game` is a number to pick a specific game to play.



