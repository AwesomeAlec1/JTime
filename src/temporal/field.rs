/*
 * This file is available under and governed by the GNU General Public
 * License version 2 only, as published by the Free Software Foundation.
 * However, the following notice accompanied the original version of this
 * file:
 *
 * Copyright (c) 2012, Stephen Colebourne & Michael Nascimento Santos
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

use std::fmt::Display;
use crate::DateTimeError;
use crate::temporal::{ChronoUnit, Temporal, TemporalAccessor, TemporalUnit, ValueRange};

/// A field of date-time, such as month-of-year or minute-of-hour.
///
/// Date and time are expressed using fields which partition the time-line into something
/// meaningful for humans. Implementations of this trait represent those fields.
///
/// The most commonly used units are defined in [`ChronoField`].
/// Further fields are supplied in [`IsoFields`], [`WeekFields`] and [`JulianFields`].
/// Fields can also be written by application code by implementing this trait.
///
/// The field works using double dispatch. Client code calls methods on a date-time like
/// `LocalDateTime` which check if the field is a `ChronoField`.
/// If it is, then the date-time must handle it.
/// Otherwise, the method call is re-dispatched to the matching method in this trait.
///
/// This trait must be implemented with care to ensure other classes operate correctly.
/// All implementations that can be instantiated must be final, immutable, and thread-safe.
/// Implementations should be [`Serializable`](serde::Serialize) where possible.
/// An enum is an effective implementation choice.
pub trait TemporalField: Display + 'static
{
    type BaseUnit: TemporalUnit;
    type RangeUnit: TemporalUnit;

    fn display_name(&self) -> &str;

    fn base_unit(&self) -> &Self::BaseUnit;

    fn range_unit(&self) -> &Self::RangeUnit;

    fn range(&self) -> ValueRange;

    fn is_date_based(&self) -> bool;

    fn is_time_based(&self) -> bool;

    fn is_supported_by<T>(&self, temporal: &T) -> bool
    where T: TemporalAccessor;

    fn range_refined_by<T>(&self, temporal: &T) -> Result<ValueRange, DateTimeError>
    where T: TemporalAccessor + ?Sized;

    fn get_from<T>(&self, temporal: &T) -> Result<i64, DateTimeError>
    where T: TemporalAccessor;

    fn adjust_into<T>(&self, temporal: &T, new_val: i64) -> Result<T, DateTimeError>
    where T: Temporal;
}

pub struct ChronoField
{
    name: &'static str,
    base_unit: &'static ChronoUnit,
    range_unit: &'static ChronoUnit,
    range: ValueRange,
    _display_key: Option<&'static str>,
}

impl ChronoField
{
    pub const NANO_OF_SECOND: &Self = &Self::new(
        "NanoOfSecond",
        ChronoUnit::NANOS,
        ChronoUnit::SECONDS,
        unsafe{ ValueRange::fixed_unchecked(0, 999_999_999) },
        None
    );

    pub const fn check_valid_value(&self, value: i64) -> Option<i64>
    {
        self.range.check_valid_value(value)
    }

    pub const fn check_valid_int_value(&self, value: i64) -> Option<i32>
    {
        self.range.check_valid_int_value(value)
    }

    const fn new(name: &'static str,
                 base_unit: &'static ChronoUnit,
                 range_unit: &'static ChronoUnit,
                 range: ValueRange,
                 display_key: Option<&'static str>) -> Self
    {
        Self { name, base_unit, range_unit, range, _display_key: display_key }
    }

    fn ordinal(&self) -> u8
    {
        todo!()
    }
}

impl Display for ChronoField
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for ChronoField
{
    fn eq(&self, other: &Self) -> bool
    {
        self.ordinal() == other.ordinal()
    }
}

impl Eq for ChronoField {}

impl PartialOrd for ChronoField
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for ChronoField
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.ordinal().cmp(&other.ordinal())
    }
}

impl TemporalField for ChronoField
{
    type BaseUnit = ChronoUnit;
    type RangeUnit = ChronoUnit;

    fn display_name(&self) -> &str { self.name }

    fn base_unit(&self) -> &Self::BaseUnit { self.base_unit }

    fn range_unit(&self) -> &Self::RangeUnit { self.range_unit }

    fn range(&self) -> ValueRange { self.range }

    fn is_date_based(&self) -> bool { todo!() }

    fn is_time_based(&self) -> bool { todo!() }

    fn is_supported_by<T>(&self, temporal: &T) -> bool
    where T: TemporalAccessor
    {
        temporal.supports_field(self)
    }

    fn range_refined_by<T>(&self, temporal: &T) -> Result<ValueRange, DateTimeError>
    where T: TemporalAccessor + ?Sized
    {
        temporal.range(self)
    }

    fn get_from<T>(&self, temporal: &T) -> Result<i64, DateTimeError>
    where T: TemporalAccessor
    {
        temporal.get_i64(self)
    }

    fn adjust_into<T>(&self, temporal: &T, new_val: i64) -> Result<T, DateTimeError>
    where T: Temporal
    {
        temporal.with(self, new_val)
    }
}
