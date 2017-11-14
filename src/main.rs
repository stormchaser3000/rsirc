extern crate pancurses;
extern crate irc;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use pancurses::*;
use std::default::Default;
use irc::client::prelude::*;

fn main()
{
    let pathtofile = Path::new("doc/copyright.md");

    let mut f = File::open(&pathtofile).expect("could not read file: file not found");
    let mut copyright = String::new();
    f.read_to_string(&mut copyright).expect("failed read to string");
    println!("Welcome to rsirc, an irc client written in rust");
    println!("\n{}", copyright);

    let args: Vec<String> = env::args.collect();
    let cfg = Config {
        nickname: Some(format!(&args[1])),
        server: Some(format!(&args[2])),
        channel: Some(vec![format![&args[3]]]),
    };

    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();

    let window = initscr();
    server.for_each_incoming(|message| {
        window.printw(&message)
        while (true)
        {
            window.refresh()
        }
    })

}
