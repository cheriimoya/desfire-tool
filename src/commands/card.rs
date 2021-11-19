use super::super::desfire::*;
use super::super::CardCommand;
use colored::*;

pub fn card(command: CardCommand) {
    match command {
        CardCommand::Configure => configure(),
        CardCommand::Format => format(),
        CardCommand::Info => info(),
    }
}

fn configure() {
    panic!("card::configure Not implemented");
}

fn format() {
    panic!("card::format Not implemented");
}

fn info() {
    let card_version = get_card_info();
    println!("{}", "Reading your Card ...".green());
    println!("Card Version: {}", card_version.card_version);
    println!("Free Space:   {} b", card_version.free_memory);
    println!("Size:         {} kb", card_version.size);
    println!("UID:          {:X?}", card_version.uid);
}
