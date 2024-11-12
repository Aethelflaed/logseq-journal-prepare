use chrono::{Datelike, Days, IsoWeek, Months, NaiveDate, Weekday};

#[derive(Debug, Default, Clone, Copy, PartialEq, derive_more::From, derive_more::Display)]
#[display("{:04}", _0)]
pub struct Year(i32);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Month {
    year: i32,
    month: u32,
}

impl Month {
    pub fn name(&self) -> &str {
        chrono::Month::try_from(self.month as u8).unwrap().name()
    }

    pub fn year(&self) -> Year {
        self.year.into()
    }
}

impl From<NaiveDate> for Month {
    fn from(date: NaiveDate) -> Self {
        Month {
            year: date.year(),
            month: date.month(),
        }
    }
}

pub trait DateRange {
    type Element;

    fn first(&self) -> Self::Element;
    fn last(&self) -> Self::Element;
}
impl DateRange for IsoWeek {
    type Element = NaiveDate;

    fn first(&self) -> NaiveDate {
        NaiveDate::from_isoywd_opt(self.year(), self.week(), Weekday::Mon).unwrap()
    }
    fn last(&self) -> NaiveDate {
        NaiveDate::from_isoywd_opt(self.year(), self.week(), Weekday::Sun).unwrap()
    }
}
impl DateRange for Month {
    type Element = NaiveDate;

    fn first(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, self.month, 1).unwrap()
    }
    fn last(&self) -> NaiveDate {
        self.first() + Months::new(1) - Days::new(1)
    }
}
impl DateRange for Year {
    type Element = Month;

    fn first(&self) -> Month {
        Month {
            year: self.0,
            month: 1,
        }
    }
    fn last(&self) -> Month {
        Month {
            year: self.0,
            month: 12,
        }
    }
}

pub trait Navigation {
    fn next(&self) -> Self;
    fn prev(&self) -> Self;
}

impl Navigation for NaiveDate {
    fn next(&self) -> Self {
        *self + Days::new(1)
    }
    fn prev(&self) -> Self {
        *self - Days::new(1)
    }
}

impl Navigation for Month {
    fn next(&self) -> Self {
        (self.first() + Months::new(1)).into()
    }
    fn prev(&self) -> Self {
        (self.first() - Months::new(1)).into()
    }
}

impl Navigation for Year {
    fn next(&self) -> Self {
        Year(self.0 + 1)
    }
    fn prev(&self) -> Self {
        Year(self.0 - 1)
    }
}

impl Navigation for IsoWeek {
    fn next(&self) -> Self {
        (self.last() + Days::new(1)).iso_week()
    }
    fn prev(&self) -> Self {
        (self.first() - Days::new(1)).iso_week()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod date_range {
        use super::*;

        #[test]
        fn date() {
            let date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();
            assert_eq!(date.next(), NaiveDate::from_ymd_opt(2024, 9, 2).unwrap());
            assert_eq!(date.prev(), NaiveDate::from_ymd_opt(2024, 8, 31).unwrap());
        }

        #[test]
        fn week() {
            let week = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap().iso_week();
            let prev = week.prev();
            assert_eq!(52, prev.week());
            assert_eq!(2024, prev.year());

            let next = week.next();
            assert_eq!(2, next.week());
            assert_eq!(2025, next.year());
        }

        #[test]
        fn month() {
            let month = Month::from(NaiveDate::from_ymd_opt(2024, 12, 1).unwrap());

            assert_eq!(
                Month {
                    year: 2024,
                    month: 11
                },
                month.prev()
            );
            assert_eq!(
                Month {
                    year: 2025,
                    month: 1
                },
                month.next()
            );
        }

        #[test]
        fn year() {
            let year = Year::from(2024);
            assert_eq!(Year::from(2023), year.prev());
            assert_eq!(Year::from(2025), year.next());
        }
    }
    mod navigation {
        use super::*;

        #[test]
        fn week() {
            let week = NaiveDate::from_ymd_opt(2024, 9, 24).unwrap().iso_week();
            assert_eq!(week.first(), NaiveDate::from_ymd_opt(2024, 9, 23).unwrap());
            assert_eq!(week.last(), NaiveDate::from_ymd_opt(2024, 9, 29).unwrap());
        }

        #[test]
        fn month() {
            let month = Month::from(NaiveDate::from_ymd_opt(2024, 2, 5).unwrap());
            assert_eq!(month.first(), NaiveDate::from_ymd_opt(2024, 2, 1).unwrap());
            assert_eq!(month.last(), NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
        }

        #[test]
        fn year() {
            let year = Year::from(2024);
            assert_eq!(
                year.first(),
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().into()
            );
            assert_eq!(
                year.last(),
                NaiveDate::from_ymd_opt(2024, 12, 1).unwrap().into()
            );
        }
    }
}