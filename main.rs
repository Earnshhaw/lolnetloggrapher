mod config_reader;
mod netlog_parser;
mod new_plotter;
use anyhow::Result;
use netlog_parser::{get_param_vec, get_timeloss, get_timeping};
use new_plotter::graphit;
use std::io;

use crate::config_reader::get_cfg;

fn io() -> Result<String> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)?;
    Ok(inp.trim().to_string())
}

fn main() -> Result<()> {
    //Take in the file name
    println!("Copy paste path to network log file");
    let iol = io()?;
    let target = iol.as_str();
    ///////////////////////

    //Take in operation type
    let paramms = get_param_vec(target)?;
    println!("(P)ing over time");
    println!("(L)oss over time");
    let operation = io()?;
    let ex = operation.trim();
    ////////////////////////

    //Match Ping and Loss to function calls or call an exception
    if ex.to_lowercase() == "p" {
        let tmpng = get_timeping(&paramms)?;
        //visualize(tmpng, "#35C788", "Ping")?;
        graphit(tmpng, "Ping", get_cfg()?)?;
    } else if ex.to_lowercase() == "l" {
        let lstm = get_timeloss(&paramms)?;
        //visualize(lstm, "#DD3355", "Loss")?;
        graphit(lstm, "Loss", get_cfg()?)?;
    } else {
        println!("Invalid operation type");
    }
    Ok(())
}
