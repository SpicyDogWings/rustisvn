use std::{io};
use colored::Colorize;
mod svn;
mod menu;
use menu::print_main_menu;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let directory = if args.len() > 1 {
        args[1].clone()
    } else {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        current_dir.strip_prefix(std::env::current_dir().unwrap())
            .unwrap_or(&current_dir)
            .to_string_lossy()
            .into_owned()
    };
    std::process::Command::new("clear").status().expect("Failed to clear screen");
    println!(" {} \n", "SVN Client  󰴻".blue());
    println!("󰘬 Status");
    svn::print_svn_status(&directory);
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let option_selected = user_input.trim();
        std::process::Command::new("clear").status().expect("Failed to clear screen");
        println!(" {} \n", "SVN Client  󰴻".blue());
        if option_selected == "s" {
            println!("󰘬 Status");
            svn::print_svn_status(&directory);
        } else if option_selected == "f" {
            println!(" Filter");
            let mut filter_input = String::new();
            io::stdin().read_line(&mut filter_input).expect("Failed to read line");
            let filter_string = filter_input.trim_end_matches("\n").to_string();
            std::process::Command::new("clear").status().expect("Failed to clear screen");
            svn::print_filtered_svn_status(&directory, filter_string);
        } else if option_selected == "q" {
            std::process::Command::new("clear").status().expect("Failed to clear screen");
            println!("Good by 󰴻");
            break;
        } else if option_selected == "?" {
            print_main_menu();
        } else {
            println!("󰘬 Status");
            svn::print_svn_status(&directory);
            println!("Invalid option");
        }
    }
}
