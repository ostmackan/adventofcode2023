pub struct Mapper{
    pub name: String,
    pub input: String,
    pub rows: Vec<MappperRow>
}

impl Mapper{
    pub fn transform(&self, value:i64) -> i64{
        for row in self.rows.iter() {
            if row.check(value) {
                return row.transform(value);
            }
        }

        value
    }
}

pub struct MappperRow{
    pub destination: i64,
    pub source: i64,
    pub range: i64,
    pub diff: i64,
}

impl MappperRow {

    pub fn check(&self, value: i64) -> bool{
        self.source <= value && (self.source + self.range) >= value
    }

    pub fn transform(&self, value: i64) -> i64{
        value+self.diff
    }
}
