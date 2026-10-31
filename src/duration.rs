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
use serde::{Deserialize, Serialize};

/// A time-based amount of time, such as '34.5 seconds'.
///
/// This struct models a quantity or amount of time in terms of seconds and nanoseconds.
/// It can be accessed using other duration-based units, such as minutes and hours.
/// In addition, the [`DAYS'] unit can be used and is treated as
/// exactly equal to 24 hours, thus ignoring daylight savings effects.
/// See [`Period`] for the date-based equivalent to this struct.
///
/// A physical duration could be of infinite length.
/// For practicality, the duration is stored with constraints similar to [`Instant`](super::Instant).
/// The duration uses nanosecond resolution with a maximum value of the seconds that can
/// be held in an `i64`. This is greater than the current estimated age of the universe.
///
/// The range of a duration requires the storage of a number larger than an `i64`.
/// To achieve this, the struct stores an `i64` representing seconds and a `u32`
/// representing nanosecond-of-second, which will always be between `0` and `999,999,999`.
/// The model is of a directed duration, meaning that the duration may be negative.
///
/// The duration is measured in "seconds", but these are not necessarily identical to
/// the scientific "SI second" definition based on atomic clocks.
/// This difference only impacts durations measured near a leap-second and should not affect
/// most applications.
/// See [`Instant`](super::Instant) for a discussion as to the meaning of the second and time-scales.
///
/// This is a value-based struct; programmers should treat instances that are
/// [equal](std::ops::Eq) as interchangeable and should not
/// use instances for synchronization, or unpredictable behavior may
/// occur. For example, in a future release, synchronization may fail.
/// The [eq](PartialEq::eq) method should be used for comparisons.
///
/// This struct is immutable and thread-safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Duration
{
    seconds: i64,
    nanos:   u32,
}

impl Duration
{
    pub const ZERO: Self = Self { seconds: 0, nanos: 0 };

    pub const fn of_seconds(seconds: i64) -> Self
    {
        Self::create_(seconds, 0)
    }

    pub const fn of_seconds_adj(seconds: i64, nanos: u32) -> Option<Self>
    {
        let secs = seconds.checked_add(nanos.div_euclid(1_000_000_000) as i64);

        if secs.is_none() { return None; }

        let nos = nanos.rem_euclid(1_000_000_000);

        Some(Self::create_(secs.unwrap(), nos))
    }

    pub const fn of_nanos(nanos: i64) -> Self
    {
        let mut secs = nanos.div_euclid(1_000_000_000);
        let mut nos = nanos.rem_euclid(1_000_000_000) as i32;

        if nos < 0
        {
            nos += 1_000_000_000;
            secs -= 1;
        }

        Self::create_(secs, nos.cast_unsigned())
    }

    pub const unsafe fn new_unchecked(seconds: i64, nanos: u32) -> Self
    {
        Self::create_(seconds, nanos)
    }

    pub const fn is_positive(&self) -> bool
    {
        (self.seconds | (self.nanos as i64)) > 0
    }

    pub const fn is_zero(&self) -> bool
    {
        (self.seconds | (self.nanos as i64)) == 0
    }

    pub const fn is_negative(&self) -> bool
    {
        self.seconds < 0
    }

    pub const fn seconds(&self) -> i64 { self.seconds }

    pub const fn nanos(&self) -> u32 { self.nanos }

    pub const fn with_seconds(self, seconds: i64) -> Self
    {
        Self::create_(seconds, self.nanos)
    }

    pub const fn with_nanos(self, nanos: u32) -> Option<Self>
    {
        todo!()
    }

    const fn create_(seconds: i64, nanos: u32) -> Self
    {
        Self { seconds, nanos }
    }
}
