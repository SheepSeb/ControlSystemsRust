pub struct TimeSeries{
    time: Vec<f64>,
    data: Vec<f64>,
}

impl TimeSeries{
    pub fn new(time: Vec<f64>, data: Vec<f64>) -> TimeSeries{
        TimeSeries{time, data}
    }

    pub fn get_time(&self) -> Vec<f64>{
        self.time.clone()
    }

    pub fn get_data(&self) -> Vec<f64>{
        self.data.clone()
    }
}