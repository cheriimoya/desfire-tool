use super::super::ApplicationCommand;
use super::super::desfire::*;
use super::helper::*;
use colored::*;

pub fn application(command: ApplicationCommand) {
    match command {
        ApplicationCommand::Add { aid } => add(aid),
        ApplicationCommand::Change { aid } => change(aid),
        ApplicationCommand::Configure { aid } => configure(aid),
        ApplicationCommand::Info { aid } => info(aid),
        ApplicationCommand::Remove { aid } => remove(aid),
        ApplicationCommand::List => list(),
    }
}

fn add(aid: u8) {
    println!(
        "{} {} {}",
        "Adding new Application with ID".blue(),
        aid,
        "to your card".blue()
    );
    print!("Do you want to enable encryption on your new Application? [NONE|AES|3K3DES]");
    let generate_master_key: bool =
        ask_bool("Should I generate a new Key for this Application (no leaves default) [yes|no]: ");
    panic!(
        "application::add Not implemented, generate_master_key: {}",
        generate_master_key
    );
}

fn change(aid: u8) {
    panic!("application::change Not implemented, aid: {}", aid);
}

fn configure(aid: u8) {
    panic!("application::configure Not implemented, aid: {}", aid);
}

fn info(aid: u8) {
    panic!("application::info Not implemented, aid: {}", aid);
}

fn remove(aid: u8) {
    panic!("application::remove Not implemented, aid: {}", aid);
}

fn list() {
    panic!("application::list Not implemented");
}
