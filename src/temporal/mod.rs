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

mod accessor;
mod field;
mod range;
mod unit;
mod amount;

pub use accessor::*;
pub use field::*;
pub use range::*;
pub use unit::*;
use crate::DateTimeError;
use crate::temporal::amount::TemporalAmount;

/// Framework-level interface defining read-write access to a temporal object,
/// such as a date, time, offset, or some combination of these.
///
/// This is the base interface type for date, time, and offset objects that
/// are complete enough to be manipulated using plus and minus.
/// It is implemented by those classes that can provide and manipulate information
/// as [fields] or [queries].
/// See [`TemporalAccessor`](TemporalAccessor) for the read-only version of this interface.
///
/// Most date and time information can be represented as a number.
/// These are modeled using `TemporalField` with the number held using
/// an `i64` to handle large values. Year, month, and day-of-month are
/// simple examples of fields, but they also include instant and offsets.
/// See [`ChronoField`] for the standard set of fields.
///
/// Two pieces of date/time information cannot be represented by numbers,
/// the [chronology] and the [time-zone].
/// These can be accessed via [queries] using
/// the static methods defined on [`TemporalQuery`].
///
/// This interface is a framework-level interface that should not be widely
/// used in application code. Instead, applications should create and pass
/// around instances of concrete types, such as `LocalDate`.
/// There are many reasons for this, part of which is that implementations
/// of this interface may be in calendar systems other than ISO.
/// See [`ChronoLocalDate`] for a fuller discussion of the issues.
///
/// ## When to implement
/// A class should implement this interface if it meets three criteria:
/// - It provides access to date/time/offset information, as per `TemporalAccessor`
/// - The set of fields are contiguous from the largest to the smallest
/// - The set of fields are complete, such that no other field is needed to define the
/// valid range of values for the fields that are represented
///
/// Four examples make this clear:
/// - `LocalDate` implements this interface as it represents a set of fields
/// that are contiguous from days to forever and require no external information to determine
/// the validity of each date. It is therefore able to implement plus/minus correctly.
/// - `LocalTime` implements this interface as it represents a set of fields
/// that are contiguous from nanos to within days and require no external information to determine
/// validity. It is able to implement plus/minus correctly by wrapping around the day.
/// - `MonthDay`, the combination of month-of-year and day-of-month, does not implement
/// this interface. While the combination is contiguous, from days to months within years,
/// the combination does not have enough information to define the valid range of values
/// for day-of-month. As such, it is unable to implement plus/minus correctly.
/// - The combination day-of-week and day-of-month ("Friday the 13th") should not implement
/// this interface. It does not represent a contiguous set of fields, as days to weeks overlap
/// days to months.
///
/// All implementations must implement [`Ord`](Ord).
pub trait Temporal: TemporalAccessor
{
    fn supports_unit<U>(&self, unit: &U) -> bool
    where U: TemporalUnit;

    fn adjust<F>(&self, adjuster: &F) -> Self
    where
        F: Fn(&Self) -> Self,
        Self: Sized
    {
        adjuster(self)
    }

    fn with<F>(&self, field: &F, new_val: i64) -> Result<Self, DateTimeError>
    where
        F: TemporalField,
        Self: Sized;

    fn plus_amount<A>(&self, amount: &A) -> Result<Self, DateTimeError>
    where
        A: TemporalAmount,
        Self: Sized
    {
        amount.add_to(self)
    }

    fn plus<U>(&self, amount: i64, unit: &U) -> Result<Self, DateTimeError>
    where
        U: TemporalUnit,
        Self: Sized;

    fn minus_amount<A>(&self, amount: &A) -> Result<Self, DateTimeError>
    where
        A: TemporalAmount,
        Self: Sized
    {
        amount.sub_from(self)
    }

    fn minus<U>(&self, amount: i64, unit: &U) -> Result<Self, DateTimeError>
    where
        U: TemporalUnit,
        Self: Sized
    {
        if amount == i64::MIN
        {
            self.plus(i64::MAX, unit)?.plus(1, unit)
        }
        else
        {
            self.plus(-amount, unit)
        }
    }

    fn until<T, U>(&self, end_exc: &T, unit: &U) -> Result<i64, DateTimeError>
    where
        T: Temporal,
        U: TemporalUnit;
}
