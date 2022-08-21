use std::collections::HashSet;
use std::cmp::Ordering;

use crate::back_end::major::{Major, Class, Req};

#[derive(PartialEq, Eq, PartialOrd)]
struct ScheduledClass {
    class: Class,
    prereqs_met: bool,
}

struct Schedule {
    sems: Vec<Semester>
}

#[derive(PartialEq, Eq)]
struct Semester {
    season: Season,
    year: u32,
    classes: Vec<ScheduledClass>,
    credits: u32,
}

impl Semester {
    fn new(season: Season, year: u32) -> Semester {
        Semester { season, year, classes: Vec::new(), credits: 0 }
    }

}

impl Ord for Semester {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Semester {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.year != other.year { 
            Some(self.year.cmp(&other.year))
        } 
        else {
            Some(self.season.to_num().cmp(&other.season.to_num()))
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
enum Season {
    Summer,
    Fall,
    Spring,
}

impl Season {
    fn to_num(&self) -> i32 {
        match self {
            Season::Spring => 1,
            Season::Summer => 2,
            Season::Fall => 3
        }
    }
}
