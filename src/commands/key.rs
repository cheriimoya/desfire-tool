use super::super::desfire::*;
use super::super::KeyCommand;
use colored::*;

pub fn key(command: KeyCommand, aid: u8) {
    match command {
        KeyCommand::Add { kid } => add(aid, kid),
        KeyCommand::Change { kid } => change(aid, kid),
        KeyCommand::Configure { kid } => configure(aid, kid),
        KeyCommand::Remove { kid } => remove(aid, kid),
        KeyCommand::Info { kid } => info(aid, kid),
        KeyCommand::List => list(),
    }
}

fn add(aid: u8, kid: u8) {
    panic!("key::add Not implemented, aid: {}, kid: {}", aid, kid);
}

fn change(aid: u8, kid: u8) {
    panic!("key::change Not implemented, aid: {}, kid: {}", aid, kid);
}

fn configure(aid: u8, kid: u8) {
    panic!("key::configure Not implemented, aid: {}, kid: {}", aid, kid);
}

fn remove(aid: u8, kid: u8) {
    panic!("key::remove Not implemented, aid: {}, kid: {}", aid, kid);
}

fn info(aid: u8, kid: u8) {
    let key_settings = get_key_settings(aid, kid);
    println!("{}", "Reading settings for the specified Key ...".green());
    println!(
        "Master Key is changeable:                 {}",
        key_settings.master_key_changeable
    );
    println!(
        "Directory List needs Master key:          {}",
        key_settings.list_needs_master
    );
    println!(
        "Creating Applications needs Master key:   {}",
        key_settings.create_needs_master
    );
    println!(
        "Key Settings are changeable:              {}",
        key_settings.settings_changeable
    );
}

fn list() {
    panic!("key::list Not implemented");
}
