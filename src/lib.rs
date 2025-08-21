//! Traits and type for SIMD.
//!
//! # Example
//! ```
//! use carbon_simd::*;
//!
//! let mut left = [1, 2, 3, 4, 5, 6, 7, 8];
//! let right = [7, 6, 5, 4, 3, 2, 1, 0];
//!
//! let mut left_simd = SimdMut::new(&mut left);
//! let right_simd = SimdRef::new(&right);
//!
//! left_simd += &right_simd;
//!
//! assert_eq!(left, [8, 8, 8, 8, 8, 8, 8, 8]);
//!
//! ```
//!

#[cfg(target_arch = "x86_64")]
mod x86_64;

use core::ops::*;
use num_traits::Float;
use num_traits::Num;

/// SIMD vector trait
pub trait Simd<T: SimdElement>: Deref<Target = [T]> {}

/// A trait for types that may be used as SIMD vector elements.
pub unsafe trait SimdElement: Sized {
    /// raw SIMD vector type like `__m256i`
    type Vector: Sized + Copy;
    /// Capacity of `Vector`
    const VECTOR_LEN: usize;

    /// Returns these SIMD functions are available.
    fn is_available() -> bool;

    /// Loads values to raw SIMD vector.
    /// # Safety
    /// Make sure `Self::is_available()` returns true and length of `src` is not less than `Self::VECTOR_LEN`.
    unsafe fn load(src: *const Self) -> Self::Vector;

    /// Loads values to raw SIMD vector.
    /// # Safety
    /// Make sure `Self::is_available()` returns true and length of `src` is not less than `len`.
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector;

    /// Stores raw SIMD vector to `dst`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true and capacity of `dst` is not less than `Self::VECTOR_LEN`.
    unsafe fn store(dst: *mut Self, src: Self::Vector);

    /// Stores raw SIMD vector to `dst`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true and capacity of `dst` is not less than `len`.
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize);

    /// Creates a raw SIMD vector filled with `value`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn set(value: Self) -> Self::Vector;
}

/// A trait for type that may be used as numeric SIMD vector elements.
pub unsafe trait SimdNumElement: SimdElement + Num {
    /// Adds `left` and `right`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector;

    /// Substructs `left` and `right`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector;

    /// Multiples `left` and `right`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector;

    /// Divides `left` by `right`, then return the result.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector;

    /// Calculates `a * b + c`. You should overload this when the architecture has dedicated operator.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    #[inline(always)]
    unsafe fn fma(a: Self::Vector, b: Self::Vector, c: Self::Vector) -> Self::Vector {
        unsafe { <Self as SimdNumElement>::add(<Self as SimdNumElement>::mul(a, b), c) }
    }

    /// Raises a number to an integer power.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    #[inline(always)]
    unsafe fn powi(x: Self::Vector, n: i32) -> Self::Vector {
        let mut result = unsafe { Self::set(Self::one()) };
        if n < 0 {
            for _ in 0..-n {
                result = unsafe { <Self as SimdNumElement>::div(result, x) };
            }
        }
        if 0 < n {
            for _ in 0..n {
                result = unsafe { <Self as SimdNumElement>::mul(result, x) };
            }
        }

        result
    }
}

/// A trait for types that may be used as floating point SIMD vector elements.
pub unsafe trait SimdFloatingElement: SimdNumElement + Float {
    /// Returns sqrt of `x`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn sqrt(x: Self::Vector) -> Self::Vector;

    /// Returns exp(x).
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn exp(x: Self::Vector) -> Self::Vector;

    /// Returns tanh of `x`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn tanh(x: Self::Vector) -> Self::Vector;
}

/// A trait for types that may be used as integer SIMD vector elements.
pub unsafe trait SimdIntegerElement: SimdNumElement + Num {
    /// Calculate and of `left` and `right`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector;

    /// Calculates or of `left` and `right`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector;

    /// Calculates not of `left` and `right`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn not(left: Self::Vector) -> Self::Vector;

    /// Calculates xor of `left` and `right`.
    /// # Safety
    /// Make sure `Self::is_available()` returns true.
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector;
}

/// Mutable SIMD wrapper structure
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SimdRef<'a, T: SimdElement>(&'a [T]);

impl<'a, T: SimdElement> SimdRef<'a, T> {
    /// Creates new `SimdRef<T>`.
    pub fn new(slice: &'a [T]) -> Self {
        Self(slice)
    }
}

impl<'a, T: SimdElement> Deref for SimdRef<'a, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0
    }
}

impl<'a, T: SimdElement> Simd<T> for SimdRef<'a, T> {}

/// Mutable SIMD wrapper structure
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct SimdMut<'a, T: SimdElement>(&'a mut [T]);

impl<'a, T: SimdElement> SimdMut<'a, T> {
    /// Creates new `SimdMut<T>`.
    pub fn new(slice: &'a mut [T]) -> Self {
        Self(slice)
    }
}

impl<'a, T: SimdNumElement> SimdMut<'a, T> {
    /// Raises a number to an integer power.
    pub fn powi(&mut self, n: i32) {
        if !T::is_available() {
            panic!("simd is not available");
        }

        let len = self.len();
        let x = self.as_mut_ptr();

        unsafe {
            for i in 0..len / T::VECTOR_LEN {
                let offset = i * T::VECTOR_LEN;
                let x_vector = T::load(x.add(offset));
                let result_vector = T::powi(x_vector, n);
                T::store(x.add(offset), result_vector);
            }

            let remaining = len % T::VECTOR_LEN;
            if remaining != 0 {
                let offset = len - remaining;
                let x_vector = T::load_partial(x.add(offset), remaining);
                let result_vector = T::powi(x_vector, n);
                T::store(x.add(offset), result_vector);
            }
        }
    }
}

impl<'a, T: SimdFloatingElement> SimdMut<'a, T> {
    /// Calculates square root.
    pub fn sqrt(&mut self) {
        if !T::is_available() {
            panic!("simd is not available");
        }

        let len = self.len();
        let x = self.as_mut_ptr();

        unsafe {
            for i in 0..len / T::VECTOR_LEN {
                let offset = i * T::VECTOR_LEN;
                let x_vector = T::load(x.add(offset));
                let result_vector = <T as SimdFloatingElement>::sqrt(x_vector);
                T::store(x.add(offset), result_vector);
            }

            let remaining = len % T::VECTOR_LEN;
            if remaining != 0 {
                let offset = len - remaining;
                let x_vector = T::load_partial(x.add(offset), remaining);
                let result_vector = <T as SimdFloatingElement>::sqrt(x_vector);
                T::store(x.add(offset), result_vector);
            }
        }
    }

    /// Calculates `e^self`
    pub fn exp(&mut self) {
        if !T::is_available() {
            panic!("simd is not available");
        }

        let len = self.len();
        let x = self.as_mut_ptr();

        unsafe {
            for i in 0..len / T::VECTOR_LEN {
                let offset = i * T::VECTOR_LEN;
                let x_vector = T::load(x.add(offset));
                let result_vector = <T as SimdFloatingElement>::exp(x_vector);
                T::store(x.add(offset), result_vector);
            }

            let remaining = len % T::VECTOR_LEN;
            if remaining != 0 {
                let offset = len - remaining;
                let x_vector = T::load_partial(x.add(offset), remaining);
                let result_vector = <T as SimdFloatingElement>::exp(x_vector);
                T::store(x.add(offset), result_vector);
            }
        }
    }

    /// Calculates `tanh`
    pub fn tanh(&mut self) {
        if !T::is_available() {
            panic!("simd is not available");
        }

        let len = self.len();
        let x = self.as_mut_ptr();

        unsafe {
            for i in 0..len / T::VECTOR_LEN {
                let offset = i * T::VECTOR_LEN;
                let x_vector = T::load(x.add(offset));
                let result_vector = <T as SimdFloatingElement>::tanh(x_vector);
                T::store(x.add(offset), result_vector);
            }

            let remaining = len % T::VECTOR_LEN;
            if remaining != 0 {
                let offset = len - remaining;
                let x_vector = T::load_partial(x.add(offset), remaining);
                let result_vector = <T as SimdFloatingElement>::tanh(x_vector);
                T::store(x.add(offset), result_vector);
            }
        }
    }
}

impl<'a, T: SimdElement + SimdNumElement, R: Simd<T>> AddAssign<&R> for SimdMut<'a, T> {
    fn add_assign(&mut self, rhs: &R) {
        if !T::is_available() {
            panic!("simd is not available");
        }

        let len = self.len().min(rhs.len());
        let left = self.as_mut_ptr();
        let right = rhs.as_ptr();

        unsafe {
            for i in 0..len / T::VECTOR_LEN {
                let offset = i * T::VECTOR_LEN;
                let left_vector = T::load(left.add(offset));
                let right_vector = T::load(right.add(offset));
                let result_vector = <T as SimdNumElement>::add(left_vector, right_vector);
                T::store(left.add(offset), result_vector);
            }

            let remaining = len % T::VECTOR_LEN;
            if remaining != 0 {
                let offset = len - remaining;
                let left_vector = T::load_partial(left.add(offset), remaining);
                let right_vector = T::load_partial(right.add(offset), remaining);
                let result_vector = <T as SimdNumElement>::add(left_vector, right_vector);
                T::store_partial(left.add(offset), result_vector, remaining);
            }
        }
    }
}

impl<'a, T: SimdElement + SimdNumElement, R: Simd<T>> SubAssign<&R> for SimdMut<'a, T> {
    fn sub_assign(&mut self, rhs: &R) {
        let len = self.len().min(rhs.len());
        let left = self.as_mut_ptr();
        let right = rhs.as_ptr();

        if !T::is_available() {
            panic!("simd is not available");
        }

        unsafe {
            for i in 0..len / T::VECTOR_LEN {
                let offset = i * T::VECTOR_LEN;
                let left_vector = T::load(left.add(offset));
                let right_vector = T::load(right.add(offset));
                let result_vector = <T as SimdNumElement>::sub(left_vector, right_vector);
                T::store(left.add(offset), result_vector);
            }

            let remaining = len % T::VECTOR_LEN;
            if remaining != 0 {
                let offset = len - remaining;
                let left_vector = T::load_partial(left.add(offset), remaining);
                let right_vector = T::load_partial(right.add(offset), remaining);
                let result_vector = <T as SimdNumElement>::sub(left_vector, right_vector);
                T::store_partial(left.add(offset), result_vector, remaining);
            }
        }
    }
}

impl<'a, T: SimdElement + SimdNumElement, R: Simd<T>> MulAssign<&R> for SimdMut<'a, T> {
    fn mul_assign(&mut self, rhs: &R) {
        let len = self.len().min(rhs.len());
        let left = self.as_mut_ptr();
        let right = rhs.as_ptr();

        if !T::is_available() {
            panic!("simd is not available");
        }

        unsafe {
            for i in 0..len / T::VECTOR_LEN {
                let offset = i * T::VECTOR_LEN;
                let left_vector = T::load(left.add(offset));
                let right_vector = T::load(right.add(offset));
                let result_vector = <T as SimdNumElement>::mul(left_vector, right_vector);
                T::store(left.add(offset), result_vector);
            }

            let remaining = len % T::VECTOR_LEN;
            if remaining != 0 {
                let offset = len - remaining;
                let left_vector = T::load_partial(left.add(offset), remaining);
                let right_vector = T::load_partial(right.add(offset), remaining);
                let result_vector = <T as SimdNumElement>::mul(left_vector, right_vector);
                T::store_partial(left.add(offset), result_vector, remaining);
            }
        }
    }
}

impl<'a, T: SimdElement + SimdNumElement, R: Simd<T>> DivAssign<&R> for SimdMut<'a, T> {
    fn div_assign(&mut self, rhs: &R) {
        let len = self.len().min(rhs.len());
        let left = self.as_mut_ptr();
        let right = rhs.as_ptr();

        if !T::is_available() {
            panic!("simd is not available");
        }

        unsafe {
            for i in 0..len / T::VECTOR_LEN {
                let offset = i * T::VECTOR_LEN;
                let left_vector = T::load(left.add(offset));
                let right_vector = T::load(right.add(offset));
                let result_vector = <T as SimdNumElement>::div(left_vector, right_vector);
                T::store(left.add(offset), result_vector);
            }

            let remaining = len % T::VECTOR_LEN;
            if remaining != 0 {
                let offset = len - remaining;
                let left_vector = T::load_partial(left.add(offset), remaining);
                let right_vector = T::load_partial(right.add(offset), remaining);
                let result_vector = <T as SimdNumElement>::div(left_vector, right_vector);
                T::store_partial(left.add(offset), result_vector, remaining);
            }
        }
    }
}

impl<'a, T: SimdElement> Deref for SimdMut<'a, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0
    }
}

impl<'a, T: SimdElement> DerefMut for SimdMut<'a, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<'a, T: SimdElement> Simd<T> for SimdMut<'a, T> {}
