use crate::SimdIntegerElement;
use core::arch::x86_64::*;

unsafe impl SimdIntegerElement for i8 {
    #[inline(always)]
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_and_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_or_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn not(left: Self::Vector) -> Self::Vector {
        unsafe {
            let all_set = _mm256_set1_epi32(-1);
            Self::xor(left, all_set)
        }
    }

    #[inline(always)]
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_xor_si256(left, right) }
    }
}

unsafe impl SimdIntegerElement for u8 {
    #[inline(always)]
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_and_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_or_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn not(left: Self::Vector) -> Self::Vector {
        unsafe {
            let all_set = _mm256_set1_epi32(-1);
            Self::xor(left, all_set)
        }
    }

    #[inline(always)]
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_xor_si256(left, right) }
    }
}

unsafe impl SimdIntegerElement for i16 {
    #[inline(always)]
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_and_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_or_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn not(left: Self::Vector) -> Self::Vector {
        unsafe {
            let all_set = _mm256_set1_epi32(-1);
            Self::xor(left, all_set)
        }
    }

    #[inline(always)]
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_xor_si256(left, right) }
    }
}

unsafe impl SimdIntegerElement for u16 {
    #[inline(always)]
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_and_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_or_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn not(left: Self::Vector) -> Self::Vector {
        unsafe {
            let all_set = _mm256_set1_epi32(-1);
            Self::xor(left, all_set)
        }
    }

    #[inline(always)]
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_xor_si256(left, right) }
    }
}

unsafe impl SimdIntegerElement for i32 {
    #[inline(always)]
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_and_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_or_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn not(left: Self::Vector) -> Self::Vector {
        unsafe {
            let all_set = _mm256_set1_epi32(-1);
            Self::xor(left, all_set)
        }
    }

    #[inline(always)]
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_xor_si256(left, right) }
    }
}

unsafe impl SimdIntegerElement for u32 {
    #[inline(always)]
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_and_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_or_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn not(left: Self::Vector) -> Self::Vector {
        unsafe {
            let all_set = _mm256_set1_epi32(-1);
            Self::xor(left, all_set)
        }
    }

    #[inline(always)]
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_xor_si256(left, right) }
    }
}

unsafe impl SimdIntegerElement for i64 {
    #[inline(always)]
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_and_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_or_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn not(left: Self::Vector) -> Self::Vector {
        unsafe {
            let all_set = _mm256_set1_epi32(-1);
            Self::xor(left, all_set)
        }
    }

    #[inline(always)]
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_xor_si256(left, right) }
    }
}

unsafe impl SimdIntegerElement for u64 {
    #[inline(always)]
    unsafe fn and(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_and_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn or(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_or_si256(left, right) }
    }

    #[inline(always)]
    unsafe fn not(left: Self::Vector) -> Self::Vector {
        unsafe {
            let all_set = _mm256_set1_epi32(-1);
            Self::xor(left, all_set)
        }
    }

    #[inline(always)]
    unsafe fn xor(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_xor_si256(left, right) }
    }
}
