use std::io::Write;
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};
use chrono::{Duration, NaiveDate, NaiveTime, Utc};

fn main() {
    let my_calendar = Calendar::new()
        .name("example calendar")
        .push(
            // add an event
            Event::new()
                .summary("test event")
                .description("here I have something really important to do")
                .starts(Utc::now())
                .class(Class::Confidential)
                .ends(Utc::now() + Duration::days(1))
                .append_property(
                    Property::new("TEST", "FOOBAR")
                        .add_parameter("IMPORTANCE", "very")
                        .add_parameter("DUE", "tomorrow")
                        .done(),
                )
                .done(),
        )
        .push(
            // add a todo
            Todo::new()
                .summary("groceries")
                .description("Buy some milk")
                .done(),
        )
        .push(
            // add an all-day event
            Event::new()
                .all_day(NaiveDate::from_ymd_opt(2016, 3, 15).unwrap())
                .summary("My Birthday")
                .description("Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
                .done(),
        )
        .push(
            // event with utc timezone
            Event::new()
                .starts(CalendarDateTime::from(
                    NaiveDate::from_ymd_opt(2024, 10, 24).unwrap()
                        .and_time(NaiveTime::from_hms_opt(20, 10, 00).unwrap())
                        .and_utc()
                ))
                .summary("Birthday Party")
                .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
                .done(),
        )
        .done();

    println!("{}", my_calendar);
    let mut file = std::fs::File::create("calendar.ics").unwrap();
    writeln!(file, "{}", my_calendar).unwrap();
}
