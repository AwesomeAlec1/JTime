/*
 * This file is available under and governed by the GNU General Public
 * License version 2 only, as published by the Free Software Foundation.
 * However, the following notice accompanied the original version of this
 * file:
 *
 * Copyright (c) 2007-2012, Stephen Colebourne & Michael Nascimento Santos
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

use serde::{Deserialize, Serialize};

/// A date without a time-zone in the ISO-8601 calendar system,
/// such as `2007-12-03`.
///
/// `LocalDate` is an immutable date-time object that represents a date,
/// often viewed as year-month-day. Other date fields, such as day-of-year,
/// day-of-week, and week-of-year, can also be accessed.
/// For example, the value "2nd October 2007" can be stored in a `LocalDate`.
///
/// This struct does not store or represent a time or time-zone.
/// Instead, it is a description of the date, as used for birthdays.
/// It cannot represent an instant on the time-line without additional information
/// such as an offset or time-zone.
///
/// The ISO-8601 calendar system is the modern civil calendar system used today
/// in most of the world. It is equivalent to the proleptic Gregorian calendar
/// system, in which today's rules for leap years are applied for all time.
/// For most applications written today, the ISO-8601 rules are entirely suitable.
/// However, any application that makes use of historical dates and requires them
/// to be accurate will find the ISO-8601 approach unsuitable.
///
/// This is a value-based struct; programmers should treat instances that are
/// [equal](std::ops::Eq) as interchangeable and should not
/// use instances for synchronization, or unpredictable behavior may
/// occur. For example, in a future release, synchronization may fail.
/// The [eq](PartialEq::eq) method should be used for comparisons.
///
/// This struct is immutable and thread-safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LocalDate
{
    year:  i32,
    month: u16,
    day:   u16,
}
