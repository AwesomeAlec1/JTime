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

/// An instantaneous point on the time-line.
///
/// This struct models a single instantaneous point on the time-line.
/// This might be used to record event time-stamps in the application.
///
/// The range of an instant requires the storage of a number larger than an `i64`.
/// To achieve this, the struct stores an `i64` representing epoch-seconds and a
/// `u32` representing nanosecond-of-second, which will always be between `0` and `999,999,999`.
/// The epoch-seconds are measured from the standard epoch of `1970-01-01T00:00:00Z`,
/// where instants after the epoch have positive values and earlier instants have negative values.
/// For both the epoch-second and nanosecond parts, a larger value is always later on the time-line
/// than a smaller value.
///
/// ## Timescale
/// The length of the solar day is the standard way that humans measure time.
/// This has traditionally been subdivided into 24 hours of 60 minutes of 60 seconds,
/// forming an 86,400-second day.
///
/// Modern timekeeping is based on atomic clocks which precisely define an SI second
/// relative to the transitions of a Cesium atom. The length of an SI second was defined
/// to be very close to the 86,400th fraction of a day.
///
/// Unfortunately, as the Earth rotates, the length of the day varies.
/// In addition, over time the average length of the day is getting longer as the Earth slows.
/// As a result, the length of a solar day in 2012 is slightly longer than 86,400 SI seconds.
/// The actual length of any given day and the amount by which the Earth is slowing
/// are not predictable and can only be determined by measurement.
/// The UT1 timescale captures the accurate length of day, but is only available some
/// time after the day has completed.
///
/// The UTC timescale is a standard approach to bundle up all the additional fractions
/// of a second from UT1 into whole seconds, known as _leap-seconds_.
/// A leap-second may be added or removed depending on the Earth's rotational changes.
/// As such, UTC permits a day to have 86,399 SI seconds or 86,401 SI seconds where
/// necessary to keep the day aligned with the Sun.
///
/// The modern UTC timescale was introduced in 1972, introducing the concept of whole leap-seconds.
/// Between 1958 and 1972, the definition of UTC was complex, with minor sub-second leaps and
/// alterations to the length of the notional second. As of 2012, discussions are underway
/// to change the definition of UTC again, with the potential to remove leap seconds or
/// introduce other changes.
///
/// Given the complexity of accurate timekeeping described above, this API defines
/// its own timescale, based on the _Java Timescale_.
///
/// The Java Timescale divides each calendar day into exactly 86,400
/// subdivisions, known as seconds. These seconds may differ from the
/// SI second. It closely matches the de-facto international civil timescale,
/// the definition of which changes from time to time.
///
/// The Java Timescale has slightly different definitions for different
/// segments of the time-line, each based on the consensus international
/// timescale that is used as the basis for civil time. Whenever the
/// internationally agreed timescale is modified or replaced, a new
/// segment of the Java Timescale must be defined for it.  Each segment
/// must meet these requirements:
///
/// - The Java Timescale shall closely match the underlying international civil timescale;
/// - The Java Timescale shall exactly match the international civil timescale at noon each day;
/// - The Java Timescale shall have a precisely defined relationship to
/// the international civil timescale.
///
/// There are currently, as of 2013, two segments in the Java timescale.
///
/// For the segment from 1972-11-03 (exact boundary discussed below) until
/// further notice, the consensus international timescale is UTC (with
/// leap seconds). In this segment, the Java Timescale is identical to
/// [UTC-SLS](http://www.cl.cam.ac.uk/~mgk25/time/utc-sls/).
/// This is identical to UTC on days that do not have a leap second.
/// On days that do have a leap second, the leap second is spread equally
/// over the last 1000 seconds of the day, maintaining the appearance of
/// exactly 86,400 seconds per day.
///
/// For the segment prior to 1972-11-03, extending back arbitrarily far,
/// the consensus international timescale is defined to be UT1, applied
/// proleptically, which is equivalent to the (mean) solar time on the
/// prime meridian (Greenwich). In this segment, the Java Timescale is
/// identical to the consensus international timescale. The exact
/// boundary between the two segments is the instant where UT1 = UTC
/// between `1972-11-03T00:00` and `1972-11-04T12:00`.
///
/// Implementations of the Java timescale using the JSR-310 API are not
/// required to provide any clock that is sub-second accurate, or that
/// progresses monotonically or smoothly. Implementations are therefore
/// not required to actually perform the UTC-SLS slew or to otherwise be
/// aware of leap seconds. JSR-310 does, however, require that
/// implementations must document the approach they use when defining a
/// clock representing the current instant.
/// See [Clock]() for details on the available clocks.
///
/// The Java timescale is used for all date-time structs.
/// This includes `Instant`, `LocalDate`, `LocalTime`, `LocalDateTime`,
/// `OffsetDateTime`, `ZonedDateTime` and `Duration`.
///
/// This is a value-based struct; programmers should treat instances that are
/// [equal](std::ops::Eq) as interchangeable and should not
/// use instances for synchronization, or unpredictable behavior may
/// occur. For example, in a future release, synchronization may fail.
/// The [eq](PartialEq::eq) method should be used for comparisons.
///
/// This struct is immutable and thread-safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Instant
{
    second: i64,
    nanos:  u32
}
