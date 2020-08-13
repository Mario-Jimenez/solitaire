mod card;
mod deck;
mod input;
mod logger;
mod moves;

fn main() {
    logger::init();
    input::start_game();
}
