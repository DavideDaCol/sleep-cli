pub struct Averages{
    pub rolling_average: Vec::<f64>,
    pub total_average: f64
}
impl Averages {
    pub fn calc_averages(&mut self, data: Vec::<f64>){
        let mut counter = 0.0;
        let mut total_sum = 0.0;

        for time in data {
            counter = counter + 1.0;
            total_sum = total_sum + time;
            self.rolling_average.push(total_sum/counter);
        }
        self.total_average = total_sum / counter;
    }

    pub fn print_all(self){
        println!("rolling average: {:?}", self.rolling_average);
        println!("final average: {} hours this month", self.total_average);
    }
}