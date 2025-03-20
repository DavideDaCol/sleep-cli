use std::env;
use log::LogController;
use mathutils::Averages;
use color_eyre::Result;

//rust modules
pub mod log;
pub mod mathutils;
pub mod display;

fn main() -> Result<()>{
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();

    let data_path = &args[1];

    let mut sleep_log = LogController {
        hours_slept: Vec::<f64>::new(),
        line_amount: 0
    };

    let mut calc = Averages{
        rolling_average: Vec::new(),
        total_average: 0.0
    };

    //TODO: create an app controller to hold all state

    let terminal = ratatui::init();
    let result = display::run(terminal);
    ratatui::restore();
    result

    //the following needs to be moved to "business logic" structs

    //match sleep_log.read_file(data_path) {
    //    Ok(()) => println!("read successfully"),
    //    Err(_) => exit(1)
    //}

    //sleep_log.print_data();

    //calc.calc_averages(sleep_log.hours_slept);
    //calc.print_all();

    //let mut _output_file = fs::File::create(".///rolling_average.txt").unwrap();

    //let mut buffer_out = String::new();

    //for value in rolling_average {
    //    let final_data = value.to_string() + "\n";
    //    buffer_out.push_str(&final_data);
    //}

    //fs::write("./rolling_average.txt", buffer_out).//expect("invalid output data");

    //Ok(())
}