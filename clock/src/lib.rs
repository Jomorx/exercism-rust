use std::fmt::Display;
#[derive(PartialEq, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32
}
// impl ToString for Clock {
//     fn to_string(&self) -> String {
//         format!("{:0>2}:{:0>2}",self.hours,self.minutes)
//     }
// }
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:0>2}:{:0>2}",self.hours,self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = hours*60 + minutes;
        let mut hours = minutes /60;
        minutes = minutes %60;
        if minutes < 0 {
            minutes +=60;
            hours -= 1; 
        }
        hours = (hours %24 +24)%24;
        return Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes+minutes;
        Clock::new(self.hours, minutes)
    }
}
