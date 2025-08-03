use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};

//Define a struct for parsing desired data

pub struct Params {
    pub time: u128,
    pub ping: u16,
    pub loss: u8,
}

pub fn get_param_vec(target: &str) -> Result<Vec<Params>> {
    let file = File::open(target)?; //Open file
    let reader = io::BufReader::new(file); //Wrap in BufReader

    let mut params_vec: Vec<Params> = Vec::new(); //Create Vec of custom struct to store

    for line in reader.lines().skip(28) {
        //Use trait of reader to sort by lines, then use core iterator method to skip to desired line
        let line = line?; //Exctract Result value from iterator
        let fields: Vec<&str> = line.split(',').map(|s| s.trim()).collect(); //Split String() line into several string slice separated by a comma and store them in a Vec&str

        if fields.len() > 8 {
            //Make sure Vec above has enough entries, catch errors
            let new = Params {
                //Create an instance of Params, pass in the properties by index position in fields: Vec<&str>
                time: fields[0].parse::<u128>()?,
                ping: fields[8].parse::<u16>()?, //Parse it into the data type the Struct demands
                loss: fields[6].parse::<u8>()?,
            };
            params_vec.push(new); //If you don't know what this does, you need to reconsider your career.
        } else {
            println!("Skipping invalid line: {}", line); //Handle irregularities
        }
    }
    Ok(params_vec) //Return success value
}

pub fn get_timeping(param: &Vec<Params>) -> Result<Vec<(f64, f64)>> {
    let mut get_timeping: Vec<(f64, f64)> = Vec::new(); //Create a Vec that stores tuples of f64
    for entry in param.iter() {
        //for Param(1 line) in Vec<Params>
        let x: f64 = entry.time as f64 / 1000 as f64 / 60 as f64; //Convert ms to minutes
        get_timeping.push((x, entry.ping as f64)); //Push a tuple of each Param.time and Param.ping as f64 into tuple Vec above
    }
    Ok(get_timeping) ////
}

pub fn get_timeloss(param: &Vec<Params>) -> Result<Vec<(f64, f64)>> {
    let mut get_timeloss: Vec<(f64, f64)> = Vec::new(); //Same as above
    for entry in param.iter() {
        let x: f64 = entry.time as f64 / 1000 as f64 / 60 as f64;
        get_timeloss.push((x, entry.loss as f64)); //Push a tuple of Param.time and Param.loss as f64 into tuple Vec above
    }
    Ok(get_timeloss)
}
