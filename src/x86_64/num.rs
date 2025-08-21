use crate::*;
use core::arch::x86_64::*;

unsafe impl SimdNumElement for i8 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_epi8(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_epi8(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0i8; Self::VECTOR_LEN];
        let mut right_buff = [0i8; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                left_buff[i] *= right_buff[i];
            }

            Self::load(left_buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0i8; Self::VECTOR_LEN];
        let mut right_buff = [0i8; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                if right_buff[i] != 0 {
                    left_buff[i] /= right_buff[i];
                }
            }

            Self::load(left_buff.as_ptr())
        }
    }
}

unsafe impl SimdNumElement for u8 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_epi8(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_epi8(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0u8; Self::VECTOR_LEN];
        let mut right_buff = [0u8; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                left_buff[i] *= right_buff[i];
            }

            Self::load(left_buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0u8; Self::VECTOR_LEN];
        let mut right_buff = [0u8; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                if right_buff[i] != 0 {
                    left_buff[i] /= right_buff[i];
                }
            }

            Self::load(left_buff.as_ptr())
        }
    }
}

unsafe impl SimdNumElement for i16 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_epi16(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_epi16(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_mullo_epi16(left, right) }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0i16; Self::VECTOR_LEN];
        let mut right_buff = [0i16; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                if right_buff[i] != 0 {
                    left_buff[i] /= right_buff[i];
                }
            }

            Self::load(left_buff.as_ptr())
        }
    }
}

unsafe impl SimdNumElement for u16 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_epi16(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_epi16(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_mullo_epi16(left, right) }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0u16; Self::VECTOR_LEN];
        let mut right_buff = [0u16; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                if right_buff[i] != 0 {
                    left_buff[i] /= right_buff[i];
                }
            }

            Self::load(left_buff.as_ptr())
        }
    }
}

unsafe impl SimdNumElement for i32 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_epi32(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_epi32(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_mullo_epi32(left, right) }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0i32; Self::VECTOR_LEN];
        let mut right_buff = [0i32; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                if right_buff[i] != 0 {
                    left_buff[i] /= right_buff[i];
                }
            }

            Self::load(left_buff.as_ptr())
        }
    }
}

unsafe impl SimdNumElement for u32 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_epi32(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_epi32(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_mullo_epi32(left, right) }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0u32; Self::VECTOR_LEN];
        let mut right_buff = [0u32; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                if right_buff[i] != 0 {
                    left_buff[i] /= right_buff[i];
                }
            }

            Self::load(left_buff.as_ptr())
        }
    }
}

unsafe impl SimdNumElement for i64 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_epi64(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_epi64(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0i64; 4];
        let mut right_buff = [0i64; 4];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..4 {
                left_buff[i] *= right_buff[i];
            }

            Self::load(left_buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0i64; Self::VECTOR_LEN];
        let mut right_buff = [0i64; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                if right_buff[i] != 0 {
                    left_buff[i] /= right_buff[i];
                }
            }

            Self::load(left_buff.as_ptr())
        }
    }
}

unsafe impl SimdNumElement for u64 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_epi64(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_epi64(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0u64; 4];
        let mut right_buff = [0u64; 4];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..4 {
                left_buff[i] *= right_buff[i];
            }

            Self::load(left_buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        let mut left_buff = [0u64; Self::VECTOR_LEN];
        let mut right_buff = [0u64; Self::VECTOR_LEN];

        unsafe {
            Self::store(left_buff.as_mut_ptr(), left);
            Self::store(right_buff.as_mut_ptr(), right);

            for i in 0..Self::VECTOR_LEN {
                if right_buff[i] != 0 {
                    left_buff[i] /= right_buff[i];
                }
            }

            Self::load(left_buff.as_ptr())
        }
    }
}

unsafe impl SimdNumElement for f32 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_ps(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_ps(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_mul_ps(left, right) }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_div_ps(left, right) }
    }

    #[inline(always)]
    unsafe fn fma(a: Self::Vector, b: Self::Vector, c: Self::Vector) -> Self::Vector {
        unsafe { _mm256_fmadd_ps(a, b, c) }
    }
}

unsafe impl SimdNumElement for f64 {
    #[inline(always)]
    unsafe fn add(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_add_pd(left, right) }
    }

    #[inline(always)]
    unsafe fn sub(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sub_pd(left, right) }
    }

    #[inline(always)]
    unsafe fn mul(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_mul_pd(left, right) }
    }

    #[inline(always)]
    unsafe fn div(left: Self::Vector, right: Self::Vector) -> Self::Vector {
        unsafe { _mm256_div_pd(left, right) }
    }

    #[inline(always)]
    unsafe fn fma(a: Self::Vector, b: Self::Vector, c: Self::Vector) -> Self::Vector {
        unsafe { _mm256_fmadd_pd(a, b, c) }
    }
}
