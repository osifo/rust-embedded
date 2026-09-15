pub enum DS3231 {
    Seconds = 0,
    Minutes = 1,
    Hours = 2,
    Day = 3,
    Date = 4,
    Month = 5,
    Year = 6
}

pub enum DAY {
    Sun = 1,
    Mon = 2,
    Tue = 3,
    Wed = 4,
    Thu = 5,
    Fri = 6,
    Sat = 7
}

pub struct DateTime {
    pub sec: u8,
    pub min: u8,
    pub hrs: u8,
    pub day: u8,
    pub date: u8,
    pub month: u8,
    pub year: u8
}

impl DateTime {
    pub fn as_fields(&self) -> [u8; 7] {
        [self.sec, self.min, self.hrs, self.day, self.date, self.month, self.year]
    }
}
