/*
 * This file is available under and governed by the GNU General Public
 *  License version 2 only, as published by the Free Software Foundation.
 *  However, the following notice accompanied the original version of this
 *  file:
 *
 * Copyright (c) 2011-2012, Stephen Colebourne & Michael Nascimento Santos
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *  * Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 *  * Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 *  * Neither the name of JSR-310 nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*
 * Rust port of JSR-310 by Alec Clawson
 */
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::{DateTimeError, Duration};
use crate::temporal::Temporal;

/// A unit of date-time, such as Days or Hours.
///
/// Measurement of time is built on units, such as years, months, days, hours, minutes, and seconds.
/// Implementations of this trait represent those units.
///
/// An instance of this trait represents the unit itself, rather than an amount of the unit.
/// See [`Period`] for a struct that represents an amount in terms of the common units.
///
/// The most commonly used units are defined in [`ChronoUnit`](ChronoUnit).
/// Further units are supplied in [`IsoFields`].
/// Units can also be written by application code by implementing this trait.
///
/// The unit works using double dispatch. Client code calls methods on a date-time like
/// `LocalDateTime` which check if the unit is a `ChronoUnit`.
/// If it is, then the date-time must handle it.
/// Otherwise, the method call is re-dispatched to the matching method in this trait.
///
/// This trait must be implemented with care to ensure other classes operate correctly.
/// All implementations that can be instantiated must be final, immutable, and thread-safe.
/// It is recommended to use an enum where possible.
pub trait TemporalUnit: Display + 'static
{
    /// Gets the duration of this unit, which may be an estimate.
    ///
    /// All units return a duration measured in standard nanoseconds from this method.
    /// The duration will be positive and non-zero.
    /// For example, an hour has a duration of `60 * 60 * 1,000,000,000ns`.
    ///
    /// Some units may return an accurate duration while others return an estimate.
    /// For example, days have an estimated duration due to the possibility of
    /// daylight saving time changes.
    ///
    /// To determine if the duration is an estimate, use [`is_duration_estimated()`](Self::is_duration_estimated).
    fn duration(&self) -> Duration;

    /// Checks if the duration of the unit is an estimate.
    ///
    /// All units have a duration. However, the duration is not always accurate.
    /// For example, days have an estimated duration due to the possibility of
    /// daylight saving time changes.
    ///
    /// This method returns true if the duration is an estimate and false if it is
    /// accurate. Note that accurate/estimated ignores leap seconds.
    fn is_duration_estimated(&self) -> bool;

    /// Checks if this unit represents a component of a date.
    ///
    /// A date is time-based if it can be used to imply meaning from a date.
    /// It must have a [duration](Self::duration) that is an integral
    /// multiple of the length of a standard day.
    ///
    /// Note that it is valid for both `is_date_based()` and `is_time_based()`
    /// to return false, such as when representing a unit like 36 hours.
    fn is_date_based(&self) -> bool;

    /// Checks if this unit represents a component of a time.
    ///
    /// A unit is time-based if it can be used to imply meaning from a time.
    /// It must have a [duration](Self::duration) that divides into
    /// the length of a standard day without a remainder.
    ///
    /// Note that it is valid for both `is_date_based()` and `is_time_based()`
    /// to return false, such as when representing a unit like 36 hours.
    fn is_time_based(&self) -> bool;

    /// Checks if this unit is supported by the specified temporal object.
    ///
    /// This checks that the implementing date-time can add/subtract this unit.
    /// This can be used to avoid returning an error.
    ///
    /// This default implementation derives the value using
    /// [`Temporal::plus(i64, &TemporalUnit)`].
    fn is_supported_by<T>(&self, _temporal: &T) -> bool
    where T: Temporal
    {
        todo!()
    }

    /// Returns a copy of the specified temporal object with the specified period added.
    ///
    /// The period added is a multiple of this unit. For example, this method
    /// could be used to add "3 days" to a date by calling this method on the
    /// instance representing "days", passing the date and the period "3".
    /// The period to be added may be negative, which is equivalent to subtraction.
    ///
    /// There are two equivalent ways of using this method.
    /// The first is to invoke this method directly.
    /// The second is to use [`Temporal::plus(i64, &TemporalUnit)`]:
    /// <pre>
    ///   // these two lines are equivalent, but the second approach is recommended
    ///   temporal = this_unit.add_to(temporal);
    ///   temporal = temporal.plus(this_unit);
    /// </pre>
    /// It is recommended to use the second approach, `plus(&TemporalUnit)`,
    /// as it is a lot clearer to read in code.
    ///
    /// Implementations should perform any queries or calculations using the units
    /// available in [`ChronoUnit`](ChronoUnit) or the fields available in [`ChronoField`](ChronoField).
    /// If the unit is not supported, an `UnsupportedTemporalTypeError` must be returned.
    ///
    /// ## Returns
    /// - `Ok`: The adjusted temporal object
    /// - `Err(DateTimeError::Unspecified)`: If the amount cannot be added
    /// - `Err(DateTimeError::UnsupportedTemporalType)`: If the unit is not supported by the temporal
    fn add_to<T>(&self, temporal: &T, amount: i64) -> Result<T, DateTimeError>
    where T: Temporal;

    /// Calculates the amount of time between two temporal objects.
    ///
    /// This calculates the amount in terms of this unit. The start and end
    /// points are supplied as temporal objects and must be of compatible types.
    /// The implementation will convert the second type to be an instance of the
    /// first type before calculating the amount.
    /// The result will be negative if the end is before the start.
    /// For example, the amount in hours between two temporal objects can be
    /// calculated using `Self::HOURS.between(start_time, end_time)`.
    ///
    /// The calculation returns a whole number, representing the number of
    /// complete units between the two temporals. If there are smaller unit
    /// fields, their values are considered when determining the final
    /// whole number.
    ///
    /// For example, the amount in hours between the times 11:30 and 13:29
    /// will only be one hour as it is one minute short of two hours, or
    /// the amount in months between the dates 2024-09-29 and 2025-02-28
    /// (the last day in February) will be 4 months as it is one day short
    /// of 5 months.
    ///
    /// There are two equivalent ways of using this method.
    /// The first is to invoke this method directly.
    /// The second is to use [`Temporal::until(&Temporal, &TemporalUnit)`]:
    /// <pre>
    ///   // these two lines are equivalent
    ///   between = this_unit.between(start, end);
    ///   between = start.until(end, this_unit);
    /// </pre>
    /// The choice should be made based on which makes the code more readable.
    ///
    /// For example, this method allows the number of days between two dates to
    /// be calculated:
    /// <pre>
    ///  let days_between: i64 = Self::DAYS.between(start, end);
    ///  // or alternatively
    ///  let days_between: i64 = start.until(end, Self::DAYS);
    /// </pre>
    ///
    /// Implementations should perform any queries or calculations using the units
    /// available in [`ChronoUnit`](ChronoUnit) or the fields available in [`ChronoField`](ChronoField).
    /// If the unit is not supported, an `UnsupportedTemporalTypeError` must be returned.
    ///
    /// Implementations must begin by checking if the two temporals have the
    /// same type. If they do not, then the result must be
    /// obtained by calling `start_inc.until(end_exc, self)`.
    ///
    /// ## Returns
    /// - `Ok`: The amount of time between temporal1Inclusive and temporal2Exclusive
    /// in terms of this unit; positive if temporal2Exclusive is later than
    /// temporal1Inclusive, negative if earlier
    /// - `Err(DateTimeError::Unspecified)`: If the amount cannot be calculated, or the end
    /// temporal cannot be converted to the same type as the start temporal, or if numeric
    /// overflow occurs
    /// - `Err(DateTimeError::UnsupportedTemporalType`: If the unit is not supported by the temporal
    fn between<T1, T2>(&self, start_inc: &T1, end_exc: &T2) -> Result<i64, DateTimeError>
    where
        T1: Temporal,
        T2: Temporal;
}

/// A standard set of date period units.
///
/// This set of units provides unit-based access to manipulate a date, time or date-time.
/// The standard set of units can be extended by implementing [`TemporalUnit`](TemporalUnit).
///
/// These units are intended to be applicable in multiple calendar systems.
/// For example, most non-ISO calendar systems define units of years, months, and days,
/// just with slightly different rules.
/// The documentation of each unit explains how it operates.
///
/// This is a final, immutable, and thread-safe "enum" struct.
pub struct ChronoUnit
{
    name: &'static str,
    duration: Duration,
}

impl ChronoUnit
{
    /// Unit that represents the concept of a nanosecond, the smallest supported unit of time.
    /// For the ISO calendar system, it is equal to the 1,000,000,000th part of the second unit.
    pub const NANOS: &Self = &Self::new("Nanos", Duration::of_nanos(1));

    /// Unit that represents the concept of a microsecond.
    /// For the ISO calendar system, it is equal to the 1,000,000th part of the second unit.
    pub const MICROS: &Self = &Self::new("Micros", Duration::of_nanos(1000));

    /// Unit that represents the concept of a millisecond.
    /// For the ISO calendar system, it is equal to the 1000th part of the second unit.
    pub const MILLIS: &Self = &Self::new("Millis", Duration::of_nanos(1000_000));

    /// Unit that represents the concept of a second. For the ISO calendar system,
    /// it is equal to the second in the SI system of units, except around a leap-second.
    pub const SECONDS: &Self = &Self::new("Seconds", Duration::of_seconds(1));

    /// Unit that represents the concept of a minute.
    /// For the ISO calendar system, it is equal to 60 seconds.
    pub const MINUTES: &Self = &Self::new("Minutes", Duration::of_seconds(60));

    /// Unit that represents the concept of an hour.
    /// For the ISO calendar system, it is equal to 60 minutes.
    pub const HOURS: &Self = &Self::new("Hours", Duration::of_seconds(3600));

    /// Unit that represents the concept of half a day, as used in AM/PM.
    /// For the ISO calendar system, it is equal to 12 hours.
    pub const HALF_DAYS: &Self = &Self::new("HalfDays", Duration::of_seconds(43200));

    /// Unit that represents the concept of a day.
    /// For the ISO calendar system, it is the standard day from midnight to midnight.
    /// The estimated duration of a day is 24 hours.
    ///
    /// When used with other calendar systems, it must correspond to the day defined by
    /// the rising and setting of the Sun on Earth. It is not required that days begin
    /// at midnight - when converting between calendar systems, the date should be
    /// equivalent at midday.
    pub const DAYS: &Self = &Self::new("Days", Duration::of_seconds(86400));

    /// Unit that represents the concept of a week.
    /// For the ISO calendar system, it is equal to 7 days.
    ///
    /// When used with other calendar systems it must correspond to an integral number of days.
    pub const WEEKS: &Self = &Self::new("Weeks", Duration::of_seconds(604_800)); // 86,400 * 7

    /// Unit that represents the concept of a month.
    /// For the ISO calendar system, the length of the month varies by month-of-year.
    /// The estimated duration of a month is one twelfth of 365.2425 days.
    ///
    /// When used with other calendar systems it must correspond to an integral number of days.
    pub const MONTHS: &Self = &Self::new("Months", Duration::of_seconds(2_629_746)); // 31,556,952 / 12

    /// Unit that represents the concept of a year.
    /// For the ISO calendar system, it is equal to 12 months.
    /// The estimated duration of a year is 365.2425 days.
    ///
    /// When used with other calendar systems, it must correspond to an integral number of days
    /// or months roughly equal to a year defined by the passage of the Earth around the Sun.
    pub const YEARS: &Self = &Self::new("Years", Duration::of_seconds(31_556_952));

    /// Unit that represents the concept of a decade.
    /// For the ISO calendar system, it is equal to 10 years.
    ///
    /// When used with other calendar systems, it must correspond to an integral number of days
    /// and is normally an integral number of years.
    pub const DECADES: &Self = &Self::new("Decades", Duration::of_seconds(315_569_520)); // 31,556,952 * 10

    /// Unit that represents the concept of a century.
    /// For the ISO calendar system, it is equal to 100 years.
    ///
    /// When used with other calendar systems, it must correspond to an integral number of days
    /// and is normally an integral number of years.
    pub const CENTURIES: &Self = &Self::new("Centuries", Duration::of_seconds(3155695200)); // 31,556,952 * 100

    /// Unit that represents the concept of a millennium.
    /// For the ISO calendar system, it is equal to 1000 years.
    ///
    /// When used with other calendar systems, it must correspond to an integral number of days
    /// and is normally an integral number of years.
    pub const MILLENNIA: &Self = &Self::new("Millennia", Duration::of_seconds(31556952000)); // 31,556,952 * 1000

    /// Unit that represents the concept of an era.
    /// The ISO calendar system doesn't have eras thus it is impossible to add
    /// an era to a date or date-time.
    /// The estimated duration of the era is artificially defined as one billion years.
    ///
    /// When used with other calendar systems there are no restrictions on the unit.
    pub const ERAS: &Self = &Self::new("Eras", Duration::of_seconds(31_556_952_000_000_000)); // 31,556,952 * 1,000,000,000

    /// Artificial unit that represents the concept of forever.
    /// This is primarily used with [`TemporalField`](super::TemporalField) to represent unbounded fields
    /// such as the year or era.
    /// The estimated duration of this unit is artificially defined as the largest duration
    /// supported by [`Duration`](Duration).
    pub const FOREVER: &Self = &Self::new("Forever", Duration::of_seconds_adj(i64::MAX, 999_999_999).unwrap());

    const VALUES: &[&Self] = &[
        &Self::NANOS,
        &Self::MICROS,
        &Self::MILLIS,
        &Self::SECONDS,
        &Self::MINUTES,
        &Self::HOURS,
        &Self::HALF_DAYS,
        &Self::DAYS,
        &Self::WEEKS,
        &Self::MONTHS,
        &Self::YEARS,
        &Self::DECADES,
        &Self::CENTURIES,
        &Self::MILLENNIA,
        &Self::ERAS,
        &Self::FOREVER,
    ];

    const fn new(name: &'static str, duration: Duration) -> Self
    {
        Self { name, duration }
    }

    fn ordinal(&self) -> u8
    {
        for i in 0..Self::VALUES.len()
        {
            if self.name == Self::VALUES[i].name { return i as u8; }
        }
        panic!("Unknown unit: {}", self.name);
    }
}

impl PartialEq for ChronoUnit
{
    fn eq(&self, other: &Self) -> bool
    {
        self.ordinal() == other.ordinal()
    }
}

impl Eq for ChronoUnit {}

impl PartialOrd for ChronoUnit
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for ChronoUnit
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.ordinal().cmp(&other.ordinal())
    }
}

impl Display for ChronoUnit
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.name)
    }
}

impl TemporalUnit for ChronoUnit
{
    /// Gets the estimated duration of this unit in the ISO calendar system.
    ///
    /// All the units in this struct have an estimated duration.
    /// Days vary due to daylight saving time, while months have different lengths.
    fn duration(&self) -> Duration { self.duration }

    /// Checks if the duration of the unit is an estimate.
    ///
    /// All time units in this struct are considered to be accurate, while all date
    /// units in this struct are considered to be estimated.
    ///
    /// This definition ignores leap seconds but considers that Days vary due to
    /// daylight saving time and months have different lengths.
    fn is_duration_estimated(&self) -> bool {
        self >= Self::DAYS
    }

    /// Checks if this unit is a date unit.
    ///
    /// All units from days to eras inclusive are date-based.
    /// Time-based units and `FOREVER` return false.
    fn is_date_based(&self) -> bool
    {
        self >= Self::DAYS && self != Self::FOREVER
    }

    /// Checks if this unit is a time unit.
    ///
    /// All units from nanos to half-days inclusive are time-based.
    /// Date-based units and `FOREVER` return false.
    fn is_time_based(&self) -> bool
    {
        self < Self::DAYS
    }

    fn is_supported_by<T>(&self, temporal: &T) -> bool
    where T: Temporal
    {
        todo!()
    }

    fn add_to<T>(&self, temporal: &T, amount: i64) -> Result<T, DateTimeError>
    where T: Temporal
    {
        todo!()
    }

    fn between<T1, T2>(&self, start_inc: &T1, end_exc: &T2) -> Result<i64, DateTimeError>
    where
        T1: Temporal,
        T2: Temporal,
    {
        todo!()
    }
}
