// server

extern crate colored;
use colored::*;

pub fn affiche_serveur() {
    println!("{}","
                        ░██████╗███████╗██████╗░██╗░░░██╗███████╗██╗░░░██╗██████╗░
                        ██╔════╝██╔════╝██╔══██╗██║░░░██║██╔════╝██║░░░██║██╔══██╗
                        ╚█████╗░█████╗░░██████╔╝╚██╗░██╔╝█████╗░░██║░░░██║██████╔╝
                        ░╚═══██╗██╔══╝░░██╔══██╗░╚████╔╝░██╔══╝░░██║░░░██║██╔══██╗
                        ██████╔╝███████╗██║░░██║░░╚██╔╝░░███████╗╚██████╔╝██║░░██║
                        ╚═════╝░╚══════╝╚═╝░░╚═╝░░░╚═╝░░░╚══════╝░╚═════╝░╚═╝░░╚═╝
    ".blink().bold().red());
}