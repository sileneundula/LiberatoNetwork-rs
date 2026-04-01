use clap::Parser;

pub struct MuscarineCommands {
    action: String,

    config: String,
}

pub enum MuscarineActionCommands {
    ls,
    retv, // retrieve
}

pub enum MuscarineListCommands {
    peers,
    
    files,
    trustnet,
    bootstrap,
    domains,
}