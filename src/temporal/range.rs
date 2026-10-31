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

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// The range of valid values for a date-time field.
///
/// All [`TemporalField`](super::TemporalField) instances have a valid range of values.
/// For example, the ISO day-of-month runs from 1 to somewhere between 28 and 31.
/// This class captures that valid range.
///
/// It is important to be aware of the limitations of this class.
/// Only the minimum and maximum values are provided.
/// It is possible for there to be invalid values within the outer range.
/// For example, a weird field may have valid values of 1, 2, 4, 6, 7, thus
/// have a range of '1 - 7', despite that fact that values 3 and 5 are invalid.
///
/// Instances of this class are not tied to a specific field.
///
/// This struct is immutable and thread-safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ValueRange
{
    min_small: i64,
    min_large: i64,
    max_small: i64,
    max_large: i64,
}

impl ValueRange
{
    /// This method creates a range where the minimum and maximum values are fixed.
    /// For example, the ISO month-of-year always runs from 1 to 12.
    ///
    /// ## Returns
    /// - `Ok`: The `ValueRange` for `min`, `max`
    /// - `Err`: If the minimum is greater than the maximum
    pub const fn fixed(min: i64, max: i64) -> Result<Self, &'static str>
    {
        if min <= max
        {
            Self::variable(min, min, max, max)
        }
        else
        {
            Err("Min value must be less than or equal to max value")
        }
    }

    /// This method creates a range where the minimum value is fixed and the maximum value may vary.
    /// For example, the ISO day-of-month always starts at 1, but ends between 28 and 31.
    ///
    /// ## Returns
    /// - `Ok`: The `ValueRange` for `min`, `max_small`, `max_large`
    /// - `Err`: If the minimum is greater than the smallest maximum,
    /// or the smallest maximum is greater than the largest maximum
    pub const fn var_max(min: i64, max_small: i64, max_large: i64) -> Result<Self, &'static str>
    {
        if min <= max_small
        {
            Self::variable(min, min, max_small, max_large)
        }
        else
        {
            Err("Min value must be less than or equal to smallest maximum value")
        }
    }

    /// This method creates a range where both the minimum and maximum value may vary.
    ///
    /// ## Returns
    /// - `Ok`: The ValueRange for `min_small`, `min_large`, `max_small`, `max_large`
    /// - `Err`: If the smallest minimum is greater than the smallest maximum,
    /// or the smallest maximum is greater than the largest maximum,
    /// or the largest minimum is greater than the largest maximum,
    /// or the smallest minimum is greater than the largest minimum
    pub const fn variable(min_small: i64,
                          min_large: i64,
                          max_small: i64,
                          max_large: i64) -> Result<Self, &'static str>
    {
        if min_small > min_large
        {
            Err("Smallest minimum value must be less than largest minimum value")
        }
        else if max_small > max_large
        {
            Err("Smallest maximum value must be less than largest maximum value")
        }
        else if min_large > max_large
        {
            Err("Largest minimum value must be less than largest maximum value")
        }
        else if min_small > max_small
        {
            Err("Smallest minimum value must be less than smallest maximum value")
        }
        else
        {
            Ok(Self { min_small, min_large, max_small, max_large })
        }
    }

    /// Unsafe constructor for performance-critical or trusted instantiation.
    pub const unsafe fn fixed_unchecked(min: i64, max: i64) -> Self
    {
        Self { min_small: min, min_large: min, max_small: max, max_large: max }
    }

    /// Unsafe constructor for performance-critical or trusted instantiation.
    pub const unsafe fn new_unchecked(min_small: i64,
                                      min_large: i64,
                                      max_small: i64,
                                      max_large: i64) -> Self
    {
        Self { min_small, min_large, max_small, max_large }
    }

    /// Returns whether the value range fixed and fully known.
    ///
    /// For example, the ISO day-of-month runs from 1 to between 28 and 31.
    /// Since there is uncertainty about the maximum value, the range is not fixed.
    /// However, for the month of January, the range is always 1 to 31; thus it is fixed.
    pub const fn is_fixed(&self) -> bool
    {
        self.min_small == self.min_large && self.max_small == self.max_large
    }

    /// Gets the minimum value that the field can take.
    ///
    /// For example, the ISO day-of-month always starts at 1.
    /// The minimum would therefore be 1.
    pub const fn minimum(&self) -> i64 { self.min_small }

    /// Gets the largest possible minimum value that the field can take.
    ///
    /// For example, the ISO day-of-month always starts at 1.
    /// The largest minimum would therefore be 1.
    pub const fn largest_minimum(&self) -> i64 { self.min_large }

    /// Gets the smallest possible maximum value that the field can take.
    ///
    /// For example, the ISO day-of-month runs to between 28 and 31 days.
    /// The smallest maximum would therefore be 28.
    pub const fn smallest_maximum(&self) -> i64 { self.max_large }

    /// Gets the maximum value that the field can take.
    ///
    /// For example, the ISO day-of-month runs to between 28 and 31 days.
    /// The maximum would therefore be 31.
    pub const fn maximum(&self) -> i64 { self.max_small }

    /// Checks if all values in the range fit in an `i32`.
    ///
    /// This checks that all valid values are within the bounds of an `i32`.
    ///
    /// For example, the ISO month-of-year has values from 1 to 12, which fits in an `i32`.
    /// By comparison, ISO nano-of-day runs from 1 to 86,400,000,000,000 which does not fit
    /// in an `i32`.
    ///
    /// This implementation uses [`minimum()`](Self::minimum) and [`maximum()`](Self::maximum).
    pub const fn is_int_value(&self) -> bool
    {
        self.minimum() >= i32::MIN as i64 && self.maximum() <= i32::MAX as i64
    }

    /// Checks if the value is within the valid range.
    ///
    /// This checks that the given number is within the stored range of values.
    pub const fn is_valid_value(&self, value: i64) -> bool
    {
        value >= self.minimum() && value <= self.maximum()
    }

    /// Checks if the value is within the valid range and that all values
    /// in the range fit in an `i32`.
    ///
    /// This method combines [`is_int_value()`](Self::is_int_value) and
    /// [`is_valid_value()`](Self::is_valid_value).
    pub const fn is_valid_int_value(&self, value: i64) -> bool
    {
        self.is_int_value() && self.is_valid_value(value)
    }

    /// Checks that the specified value is valid.
    ///
    /// This validates that the value is within the valid range of values.
    ///
    /// ## Returns
    /// The passed-in value (if it was valid)
    ///
    /// > See also: [`is_valid_value(&self, i64)`](Self::is_valid_value)
    pub const fn check_valid_value(&self, value: i64) -> Option<i64>
    {
        if self.is_valid_value(value) { Some(value) } else { None }
    }

    /// Checks that the specified value is valid and fits in an `i32`.
    ///
    /// This validates that the value is within the valid range of values and that
    /// all valid values are within the bounds of an `i32`.
    ///
    /// ## Returns
    /// The passed-in value (if it was valid)
    ///
    /// > See also: [`is_valid_int_value(&self, i64)`](Self::is_valid_int_value)
    pub const fn check_valid_int_value(&self, value: i64) -> Option<i32>
    {
        if self.is_valid_int_value(value) { Some(value as i32) } else { None }
    }
}

impl Display for ValueRange
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        let mut string = self.min_small.to_string();
        if self.min_small != self.min_large
        {
            string.push_str("/");
            string.push_str(&self.min_large.to_string());
        }
        string.push_str(" - ");
        string.push_str(&self.max_small.to_string());
        if self.max_small != self.max_large
        {
            string.push_str("/");
            string.push_str(&self.max_large.to_string());
        }
        write!(f, "{}", string)
    }
}
