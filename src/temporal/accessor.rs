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
use std::any::Any;
use crate::DateTimeError;
use crate::temporal::field::TemporalField;
use crate::temporal::{ChronoField, ValueRange};

/// Framework-level interface defining read-only access to a temporal object,
/// such as a date, time, offset, or some combination of these.
///
/// This is the base interface type for date, time, and offset objects.
/// It is implemented by those classes that can provide information
/// as [fields] or [queries].
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
/// A sub-interface, [`Temporal`], extends this definition to one that also
/// supports adjustment and manipulation on more complete temporal objects.
///
/// This interface is a framework-level interface that should not be widely
/// used in application code. Instead, applications should create and pass
/// around instances of concrete types, such as `LocalDate`.
/// There are many reasons for this, part of which is that implementations
/// of this interface may be in calendar systems other than ISO.
/// See [`ChronoLocalDate`] for a fuller discussion of the issues.
pub trait TemporalAccessor
{
    /// Checks if the specified field is supported.
    ///
    /// This checks if the date-time can be queried for the specified field.
    /// If false, then calling the {@link #range(TemporalField) range} and {@link #get(TemporalField) get}
    /// methods will throw an exception.
    ///
    /// Implementations must check and handle all fields defined in [`ChronoField`].
    /// If the field is supported, then true must be returned, otherwise false must be returned.
    /// <p>
    /// If the field is not a {@code ChronoField}, then the result of this method
    /// is obtained by invoking {@code TemporalField.isSupportedBy(TemporalAccessor)}
    /// passing {@code this} as the argument.
    /// <p>
    /// Implementations must ensure that no observable state is altered when this
    /// read-only method is invoked.
    ///
    /// @param field the field to check, null returns false
    /// @return true if this date-time can be queried for the field, false if not
    fn supports_field<F: TemporalField>(&self, field: &F) -> bool;

    fn range<F>(&self, field: &F) -> Result<ValueRange, DateTimeError>
    where F: TemporalField
    {
        let any: &dyn Any = field;

        if let Some(chrono_field) = any.downcast_ref::<ChronoField>()
        {
            if self.supports_field(chrono_field)
            {
                Ok(field.range())
            }
            else
            {
                Err(DateTimeError::UnsupportedTemporalType)
            }
        }
        else
        {
            field.range_refined_by(self)
        }
    }

    fn get_i32<F>(&self, field: &F) -> Result<i32, DateTimeError>
    where F: TemporalField
    {
        let range = self.range(field)?;

        if !range.is_int_value()
        {
            return Err(DateTimeError::UnsupportedTemporalType)
        }

        let value = self.get_i64(field)?;

        range.check_valid_int_value(value)
             .ok_or_else(|| DateTimeError::Unspecified(
                 format!("Invalid value for {} (valid values {}): {}", field, range, value).into()))
    }

    fn get_i64<F>(&self, field: &F) -> Result<i64, DateTimeError>
    where F: TemporalField;

    fn query<R, F>(&self, query: &F) -> R
    where F: Fn(&Self) -> R
    {
        query(self)
    }
}
