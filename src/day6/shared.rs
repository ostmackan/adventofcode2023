pub struct RaceRecord{
    pub time: i64,
    pub distance: i64,
}

impl RaceRecord{
    pub fn find_better_runs(&self) -> Vec<RaceRecord>{
        let mut race_records: Vec<RaceRecord> = Vec::new();

        //naive
        for i in 0..self.time {
            let remaining_time = self.time -i;
            let distance = i*remaining_time;
            if distance > self.distance {
                race_records.push(RaceRecord{
                   time: i,
                    distance: distance
                });
            }
        }

        race_records
    }
}
