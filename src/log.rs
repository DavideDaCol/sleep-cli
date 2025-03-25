use std::fs::File;
use std::process::exit;
use std::io::{self,BufReader, BufRead};

use ratatui::text::Line;

#[derive(Clone)]
pub struct LogController {
    pub hours_slept: Vec::<Line<'static>>,
    pub line_amount: i32
}

//TODO: move from txt to csv file
impl LogController {

    pub fn new() -> Self {
        Self {
            hours_slept: Vec::new(),
            line_amount: 0,
        }
    }
    
    pub fn read_file(&mut self, data_path: &String) -> io::Result<()> {
        println!("Reading the contents of {data_path}...");
        let data_file = match File::open(data_path) {
            Ok(file) => file,
            Err(_) => {
                println!("ERROR: path does not exist!");
                exit(1);
            }
        };

        let read_buffer = BufReader::new(data_file);

        //parses the string to a float and puts it in the log struct
        for entry in read_buffer.lines() {
            let entry = entry?;
            self.hours_slept.push(Line::from(entry));
            //keeps track of number of entries
            self.line_amount += 1;
        }

        Ok(())
    }

    pub fn print_data(&mut self){
        println!("input data: {:?}", self.hours_slept);
        println!("total number of lines: {}", self.line_amount);
    }
}