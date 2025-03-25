use std::env;
use display::TerminalDisplay;
use log::LogController;
use mathutils::Averages;
use color_eyre::Result;
use ratatui::widgets::ScrollbarState;

//rust modules
pub mod log;
pub mod mathutils;
pub mod display;

fn main() -> Result<()>{
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();

    let data_path = &args[1];

    let mut sleep_log = LogController::new();

    sleep_log.read_file(data_path).expect("input file cannot be read");

    let total_lines = sleep_log.line_amount.max(0) as usize;

    let mut calc = Averages{
        rolling_average: Vec::new(),
        total_average: 0.0
    };

    let display = TerminalDisplay {
        app_log: sleep_log,
        app_calc: calc,
        valid_state: true,
        vertical_scroll_state: ScrollbarState::new(total_lines),
        vertical_scroll: 0
    };

    //TODO: create an app controller to hold all state

    let terminal = ratatui::init();
    let result = display.run(terminal);
    ratatui::restore();
    result

    //the following needs to be moved to "business logic" structs

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