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
            .description("This is a description.")
            .starts(Utc::now() + Duration::days(1))
            .ends(Utc::now() + Duration::days(1) + Duration::hours(2))
            .status(EventStatus::Confirmed)
            .location("Brann stadion")
            .append_property(
                Property::new("X-APPLE-STRUCTURED-LOCATION", "geo:60.366941,5.357363")
                    .add_parameter("VALUE", "URI")
                    .add_parameter("X-APPLE-MAPKIT-HANDLE", "CAESiAIIrk0Qu+aas87c9sj2ARoSCY1J2uj3Lk5AEVs6i4PwbRVAIl0KBk5vcndheRICTk8aCUhvcmRhbGFuZDIGQmVyZ2VuOgQ1MDYzUg5Lbmlrc2VucyBQbGFzc1oBMWIQS25pa3NlbnMgUGxhc3MgMYoBBkJlcmdlbooBB8OFcnN0YWQqDUJyYW5uIFN0YWRpb24yEEtuaWtzZW5zIFBsYXNzIDEyCzUwNjMgQmVyZ2VuMgVOb3JnZTgvWk4KJQi75pqzztz2yPYBEhIJjUna6PcuTkARWzqLg/BtFUAYrk2QAwGiHyQIu+aas87c9sj2ARoXCg1CcmFubiBTdGFkaW9uEAAqAmVuQAA=")
                    .add_parameter("X-APPLE-RADIUS", "271.0740051269531")
                    .add_parameter("X-APPLE-REFERENCEFRAME", "0")
                    .add_parameter("X-TITLE", "Brann stadion")
                    .done()
            )
            .done()
        )
        .done();

    // println!("{}", my_calendar);
    // println!("{}", venue.to_string());
    let mut file = std::fs::File::create("calendar.ics").unwrap();
    writeln!(file, "{}", my_calendar).unwrap();
}
