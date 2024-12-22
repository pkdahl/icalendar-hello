use std::io::Write;
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};
use icalendar::{EventStatus, Venue};
use chrono::{Duration, NaiveDate, NaiveTime, Utc};

fn main() {
    let venue = Venue::new()
        .uid("b0ab13d5-6298-4d46-835a-acbc4a24bee6")
        .street_address("Kniksens plass 1")
        .postal_code("5063")
        // .extended_address("5063 Bergen")
        .locality("Bergen")
        .region("Vestland")
        .country("Norge")
        .done();

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
            Event::new()
            .summary("Brann—PAOK")
            .starts(Utc::now() + Duration::days(1))
            .ends(Utc::now() + Duration::days(1) + Duration::hours(2))
            .venue("Brann stadion", venue.get_uid().unwrap())
            .status(EventStatus::Confirmed)
            .done()
        )
        .push(venue)
        .done();

    // println!("{}", my_calendar);
    // println!("{}", venue.to_string());
    let mut file = std::fs::File::create("calendar.ics").unwrap();
    writeln!(file, "{}", my_calendar).unwrap();
}
