extern crate ncurses;
extern crate irc;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::default::Default;
use irc::client::prelude::*;
use std::char;
use ncurses::*;

fn main()
{
    let pathtofile = Path::new("README.md");

    let mut f = File::open(&pathtofile).expect("could not read file: file not found");
    let mut copyright = String::new();
    f.read_to_string(&mut copyright).expect("failed read to string");
    println!("Welcome to rsirc, an irc client written in rust");
    println!("\n{}", copyright);

    let args: Vec<String> = env::args().collect();

    let cfg = Config {
        nickname: Some(format!["rsirc_test"]),
        server: Some(format!["chat.freenode.net"]),
        channels: Some(vec![format!["##SC-Economy"]]),
        .. Default::default()
    };

    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();
    initscr();
    raw();
    keypad(stdscr(), true);
    let mut m = String::new();
    server.for_each_incoming(|message| {
        printw(&message.to_string());
        refresh();
    });

}
