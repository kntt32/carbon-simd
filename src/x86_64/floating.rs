use crate::*;
use core::arch::x86_64::*;

unsafe impl SimdFloatingElement for f32 {
    #[inline(always)]
    unsafe fn sqrt(x: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sqrt_ps(x) }
    }

    #[inline(always)]
    unsafe fn exp(x: Self::Vector) -> Self::Vector {
        unsafe {
            const LN_2: f32 = core::f32::consts::LN_2;
            const C0: f32 = 1.0 / 1.0;
            const C1: f32 = 1.0 / 1.0;
            const C2: f32 = 1.0 / 2.0;
            const C3: f32 = 1.0 / 6.0;
            const C4: f32 = 1.0 / 24.0;
            const C5: f32 = 1.0 / 120.0;

            let fx = _mm256_mul_ps(x, Self::set(1.0 / LN_2));
            let n = _mm256_floor_ps(fx);
            let f = _mm256_sub_ps(x, _mm256_mul_ps(n, _mm256_set1_ps(LN_2)));

            let poly = _mm256_set1_ps(C5);
            let poly = _mm256_fmadd_ps(poly, f, _mm256_set1_ps(C4));
            let poly = _mm256_fmadd_ps(poly, f, _mm256_set1_ps(C3));
            let poly = _mm256_fmadd_ps(poly, f, _mm256_set1_ps(C2));
            let poly = _mm256_fmadd_ps(poly, f, _mm256_set1_ps(C1));
            let poly = _mm256_fmadd_ps(poly, f, _mm256_set1_ps(C0));

            const EXP_BIAS: i32 = 0x7f;
            const EXP_OFFSET: i32 = 23;

            let exp_bias = _mm256_set1_epi32(EXP_BIAS);
            let n_i32 = _mm256_cvtps_epi32(n);
            let pow_2_n = _mm256_castsi256_ps(_mm256_slli_epi32(
                _mm256_add_epi32(n_i32, exp_bias),
                EXP_OFFSET,
            ));

            _mm256_mul_ps(pow_2_n, poly)
        }
    }

    #[inline(always)]
    unsafe fn tanh(x: Self::Vector) -> Self::Vector {
        unsafe {
            let one = Self::set(1.0);
            let two = Self::set(2.0);

            let mul_2x = <Self as SimdNumElement>::mul(x, two);
            let exp_2x = <Self as SimdFloatingElement>::exp(mul_2x);
            let exp_2x_plus_1 = <Self as SimdNumElement>::add(exp_2x, one);

            let first_term = one;
            let second_term = <Self as SimdNumElement>::div(two, exp_2x_plus_1);

            <Self as SimdNumElement>::sub(first_term, second_term)
        }
    }
}

unsafe impl SimdFloatingElement for f64 {
    #[inline(always)]
    unsafe fn sqrt(x: Self::Vector) -> Self::Vector {
        unsafe { _mm256_sqrt_pd(x) }
    }

    #[inline(always)]
    unsafe fn exp(x: Self::Vector) -> Self::Vector {
        unsafe {
            let mut buff = [0.0f64; Self::VECTOR_LEN];
            Self::store(buff.as_mut_ptr(), x);
            for i in &mut buff {
                *i = i.exp();
            }
            Self::load(buff.as_ptr())
        }
    }

    #[inline(always)]
    unsafe fn tanh(x: Self::Vector) -> Self::Vector {
        unsafe {
            let one = Self::set(1.0);
            let two = Self::set(2.0);

            let mul_2x = <Self as SimdNumElement>::mul(x, two);
            let exp_2x = <Self as SimdFloatingElement>::exp(mul_2x);
            let exp_2x_plus_1 = <Self as SimdNumElement>::add(exp_2x, one);

            let first_term = one;
            let second_term = <Self as SimdNumElement>::div(two, exp_2x_plus_1);

            <Self as SimdNumElement>::sub(first_term, second_term)
        }
    }
}
