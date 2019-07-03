//! Data Watchpoint Trace (DWT) unit's CYCle CouNTer

use core::{
    cmp::Ordering,
    convert::{Infallible, TryInto},
    fmt,
    marker::PhantomData,
    ops,
};

use cortex_m::peripheral::DWT;

/// A measurement of the CYCCNT. Opaque and useful only with `Duration`
///
/// This data type is only available on ARMv7-M
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Instant {
    inner: i32,
    _not_send_or_sync: PhantomData<*mut ()>,
}

unsafe impl Sync for Instant {}

#[cfg(not(feature = "heterogeneous"))]
unsafe impl Send for Instant {}

impl Instant {
    /// Returns an instant corresponding to "now"
    pub fn now() -> Self {
        Instant {
            inner: DWT::get_cycle_count() as i32,
            _not_send_or_sync: PhantomData,
        }
    }

    /// Returns the amount of time elapsed since this instant was created.
    pub fn elapsed(&self) -> Duration {
        Instant::now() - *self
    }

    /// Returns the amount of time elapsed from another instant to this one.
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        let diff = self.inner - earlier.inner;
        assert!(diff >= 0, "second instant is later than self");
        Duration { inner: diff as u32 }
    }
}

impl fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Instant")
            .field(&(self.inner as u32))
            .finish()
    }
}

impl ops::AddAssign<Duration> for Instant {
    fn add_assign(&mut self, dur: Duration) {
        debug_assert!(dur.inner < (1 << 31));
        self.inner = self.inner.wrapping_add(dur.inner as i32);
    }
}

impl ops::Add<Duration> for Instant {
    type Output = Self;

    fn add(mut self, dur: Duration) -> Self {
        self += dur;
        self
    }
}

impl ops::SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, dur: Duration) {
        // XXX should this be a non-debug assertion?
        debug_assert!(dur.inner < (1 << 31));
        self.inner = self.inner.wrapping_sub(dur.inner as i32);
    }
}

impl ops::Sub<Duration> for Instant {
    type Output = Self;

    fn sub(mut self, dur: Duration) -> Self {
        self -= dur;
        self
    }
}

impl ops::Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}

impl Ord for Instant {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.inner.wrapping_sub(rhs.inner).cmp(&0)
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

/// A `Duration` type to represent a span of time.
///
/// This data type is only available on ARMv7-M
#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Duration {
    inner: u32,
}

impl Duration {
    /// Creates a new `Duration` from the specified number of clock cycles
    pub fn from_cycles(cycles: u32) -> Self {
        Duration { inner: cycles }
    }

    /// Returns the total number of clock cycles contained by this `Duration`
    pub fn as_cycles(&self) -> u32 {
        self.inner
    }
}

impl TryInto<u32> for Duration {
    type Error = Infallible;

    fn try_into(self) -> Result<u32, Infallible> {
        Ok(self.as_cycles())
    }
}

impl ops::AddAssign for Duration {
    fn add_assign(&mut self, dur: Duration) {
        self.inner += dur.inner;
    }
}

impl ops::Add<Duration> for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Duration {
            inner: self.inner + other.inner,
        }
    }
}

impl ops::SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        self.inner -= rhs.inner;
    }
}

impl ops::Sub<Duration> for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Duration {
            inner: self.inner - rhs.inner,
        }
    }
}

/// Adds the `cycles` method to the `u32` type
///
/// This trait is only available on ARMv7-M
pub trait U32Ext {
    /// Converts the `u32` value into clock cycles
    fn cycles(self) -> Duration;
}

impl U32Ext for u32 {
    fn cycles(self) -> Duration {
        Duration { inner: self }
    }
}

/// Implementation of the `Monotonic` trait based on CYCle CouNTer
#[cfg(not(feature = "heterogeneous"))]
pub struct CYCCNT;

#[cfg(not(feature = "heterogeneous"))]
impl crate::Monotonic for CYCCNT {
    type Instant = Instant;

    fn ratio() -> u32 {
        1
    }

    unsafe fn reset() {
        (0xE0001004 as *mut u32).write_volatile(0)
    }

    fn now() -> Instant {
        Instant::now()
    }

    fn zero() -> Instant {
        Instant {
            inner: 0,
            _not_send_or_sync: PhantomData,
        }
    }
}