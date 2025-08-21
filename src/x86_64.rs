mod floating;
mod integer;
mod num;

use crate::SimdElement;
use core::arch::x86_64::*;

unsafe impl SimdElement for i8 {
    type Vector = __m256i;
    const VECTOR_LEN: usize = 32;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_si256(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0i8; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_si256(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0i8; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_epi8(value) }
    }
}

unsafe impl SimdElement for u8 {
    type Vector = __m256i;
    const VECTOR_LEN: usize = 32;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_si256(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0u8; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_si256(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0u8; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_epi8(value as i8) }
    }
}

unsafe impl SimdElement for i16 {
    type Vector = __m256i;
    const VECTOR_LEN: usize = 16;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_si256(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0i16; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_si256(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0i16; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_epi16(value) }
    }
}

unsafe impl SimdElement for u16 {
    type Vector = __m256i;
    const VECTOR_LEN: usize = 16;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_si256(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0u16; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_si256(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0u16; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_epi16(value as i16) }
    }
}

unsafe impl SimdElement for i32 {
    type Vector = __m256i;
    const VECTOR_LEN: usize = 8;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_si256(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0i32; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_si256(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0i32; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_epi32(value) }
    }
}

unsafe impl SimdElement for u32 {
    type Vector = __m256i;
    const VECTOR_LEN: usize = 8;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_si256(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0u32; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_si256(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0u32; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_epi32(value as i32) }
    }
}

unsafe impl SimdElement for i64 {
    type Vector = __m256i;
    const VECTOR_LEN: usize = 4;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_si256(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0i64; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_si256(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0i64; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_epi64x(value) }
    }
}

unsafe impl SimdElement for u64 {
    type Vector = __m256i;
    const VECTOR_LEN: usize = 4;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_si256(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0u64; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_si256(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0u64; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_epi64x(value as i64) }
    }
}

unsafe impl SimdElement for f32 {
    type Vector = __m256;
    const VECTOR_LEN: usize = 8;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_ps(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0.0f32; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_ps(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0.0f32; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_ps(value) }
    }
}

unsafe impl SimdElement for f64 {
    type Vector = __m256d;
    const VECTOR_LEN: usize = 4;

    #[inline(always)]
    fn is_available() -> bool {
        std::is_x86_feature_detected!("avx2")
    }

    #[inline(always)]
    unsafe fn load(src: *const Self) -> Self::Vector {
        unsafe { _mm256_loadu_pd(src as _) }
    }

    #[inline(always)]
    unsafe fn load_partial(src: *const Self, len: usize) -> Self::Vector {
        let mut buff = [0.0f64; Self::VECTOR_LEN];
        unsafe {
            src.copy_to_nonoverlapping(buff.as_mut_ptr(), len);
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn store(dst: *mut Self, src: Self::Vector) {
        unsafe {
            _mm256_storeu_pd(dst as _, src);
        }
    }

    #[inline(always)]
    unsafe fn store_partial(dst: *mut Self, src: Self::Vector, len: usize) {
        let mut buff = [0.0f64; Self::VECTOR_LEN];
        unsafe {
            Self::store(buff.as_mut_ptr(), src);
            buff.as_ptr().copy_to_nonoverlapping(dst, len);
        }
    }

    #[inline(always)]
    unsafe fn set(value: Self) -> Self::Vector {
        unsafe { _mm256_set1_pd(value) }
    }
}
