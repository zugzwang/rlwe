use core::marker::PhantomData;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul, Rem, SubAssign};

use generic_array::{ArrayLength, GenericArray};
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use typenum::{PowerOfTwo, Unsigned};

use crate::traits::{Characteristic, Element, FieldElement, RlweRing, Vector};

#[derive(Clone, PartialEq)]
pub struct ModularBigInt<C: Characteristic> {
    representant: BigInt,
    modulus: PhantomData<C>,
}
impl<C: Characteristic> Debug for ModularBigInt<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.representant.fmt(f)
    }
}

impl<C: Characteristic> From<BigInt> for ModularBigInt<C> {
    fn from(x: BigInt) -> Self {
        let ch: BigUint = C::to_biguint();
        let res = ModularBigInt::<C> {
            representant: x,
            modulus: PhantomData,
        };
        res % ch
    }
}

impl<C: Characteristic> Zero for ModularBigInt<C> {
    fn zero() -> Self {
        Self {
            representant: Zero::zero(),
            modulus: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.representant.is_zero()
    }
}

impl<C: Characteristic> Add for ModularBigInt<C> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let val = self.representant.clone() + other.representant.clone();
        Self {
            representant: val,
            modulus: PhantomData,
        } % C::to_biguint()
    }
}

impl<C: Characteristic> AddAssign for ModularBigInt<C> {
    fn add_assign(&mut self, other: Self) {
        let val = self.representant.clone() + other.representant.clone();
        *self = Self {
            representant: val,
            modulus: PhantomData,
        } % C::to_biguint()
    }
}

impl<C: Characteristic> SubAssign for ModularBigInt<C> {
    fn sub_assign(&mut self, other: Self) {
        let val = self.representant.clone() - other.representant.clone();
        *self = Self {
            representant: val,
            modulus: PhantomData,
        } % C::to_biguint()
    }
}

impl<C: Characteristic> Mul for ModularBigInt<C> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let val = self.representant.clone() * other.representant.clone();
        Self {
            representant: val,
            modulus: PhantomData,
        } % C::to_biguint()
    }
}

impl<C: Characteristic> Rem<BigUint> for ModularBigInt<C> {
    type Output = Self;

    fn rem(self, modulus: BigUint) -> Self::Output {
        if modulus == Zero::zero() {
            return self;
        }
        let m: BigInt = modulus.into();
        let right = m.clone() / 2_u32;
        let left = right.clone() - m.clone();
        let rep = self.representant % m.clone();
        let true_rep = if rep <= left {
            rep + m
        } else if rep > right {
            rep - m
        } else {
            rep
        };
        ModularBigInt::<C> {
            representant: true_rep,
            modulus: PhantomData,
        }
    }
}

impl<C: Characteristic> FieldElement for ModularBigInt<C> {
    type Char = C;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cyclotomic<T: Unsigned + PowerOfTwo, C: Characteristic> {
    degree: PhantomData<T>,
    characteristic: PhantomData<C>,
}

impl<C: Characteristic, T: ArrayLength<ModularBigInt<C>> + PowerOfTwo> RlweRing
    for Cyclotomic<T, C>
{
    type Coefficient = ModularBigInt<C>;
    type Degree = T;

    fn mul(_a: Element<Self>, _b: Element<Self>) -> Element<Self> {
        todo!()
    }
}

impl<C, T> From<Vector> for Element<Cyclotomic<T, C>>
where
    C: Characteristic,
    T: ArrayLength<ModularBigInt<C>> + PowerOfTwo,
{
    fn from(p: Vector) -> Self {
        let degree = T::to_usize();
        let mut coordinates: Vec<ModularBigInt<C>> = p
            .coordinates
            .to_vec()
            .iter()
            .map(|x| x.clone().into())
            .collect();
        let coefficients = if coordinates.len() <= degree {
            coordinates.resize(degree, Zero::zero());
            GenericArray::<ModularBigInt<C>, T>::clone_from_slice(&coordinates)
        } else {
            let mut slice: Vec<ModularBigInt<C>> = vec![Zero::zero(); degree];
            // TODO: Parallelization
            for i in 0..coordinates.len() {
                if i / degree % 2 == 0 {
                    slice[i % degree] += coordinates[i].clone();
                } else {
                    slice[i % degree] -= coordinates[i].clone();
                }
            }

            GenericArray::<ModularBigInt<C>, T>::clone_from_slice(&slice)
        };

        Element::<Cyclotomic<T, C>> { coefficients }
    }
}

#[cfg(test)]
mod tests {
    use num_traits::FromPrimitive;
    use rand::Rng;
    use typenum::consts::{U16, U4};

    use super::*;
    use crate::characteristic;
    use crate::traits::CharZero;

    characteristic!(Char7, BigUint::from_u8(7).unwrap());

    #[test]
    fn convert_z() {
        type R = Cyclotomic<U4, CharZero>;
        let pairs = [
            (vec![1, 2, 3, 4, 5, 6, 7, 8], vec![-4, -4, -4, -4]),
            (vec![0, 1, 2, 3, -1], vec![1, 1, 2, 3]),
            (vec![0, 1, 2, 3, 0, 0, 0, 0, 1, 1, 1, 1], vec![1, 2, 3, 4]),
            (vec![42, 42, 42, 42, 42, 42, 42, 42], vec![0, 0, 0, 0]),
            (
                vec![0, 1, 2, 3, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                vec![0, 1, 3, 4],
            ),
        ];
        for pair in pairs {
            let vec: Vector = pair.0.into();
            let want: Vector = pair.1.clone().into();
            let got_e: Element<R> = vec.into();
            let want_e: Element<R> = want.clone().into();
            assert_eq!(got_e, want_e);
        }
    }

    #[test]
    fn convert_zp() {
        type R = Cyclotomic<U4, Char7>;

        let pairs = [
            (vec![1, 2, 3, 4, 5, 6, 7, 8], vec![3, 3, 3, 3]),
            (vec![0, 1, 2, 3, -1], vec![1, 1, 2, 3]),
            (vec![0, 1, 2, 3, 0, 0, 0, 0, 1, 1, 1, 1], vec![1, 2, 3, 4]),
            (vec![42, 42, 42, 42, 42, 42, 42, 42], vec![0, 0, 0, 0]),
            (
                vec![0, 1, 2, 3, 0, 0, 0, 0, 7, 1, 1, 1, 0, 0, 4],
                vec![0, 2, -1, 4],
            ),
        ];
        for pair in pairs {
            let vec: Vector = pair.0.into();
            let want: Vector = pair.1.clone().into();
            let got_e: Element<R> = vec.into();
            let want_e: Element<R> = want.clone().into();
            assert_eq!(got_e, want_e);
        }
    }

    #[test]
    fn double_z() {
        type R = Cyclotomic<U16, CharZero>;
        let v: Vector = rand::thread_rng().gen::<[i64; 32]>().to_vec().into();
        let x: Element<R> = v.into();
        let sum = x.clone() + &x;
        let want: Vec<ModularBigInt<CharZero>> = x
            .coefficients()
            .iter()
            .map(|c| c.clone() + c.clone())
            .collect();
        assert_eq!(sum.coefficients().as_slice(), want);
    }

    #[test]
    fn double_char_7() {
        type R = Cyclotomic<U16, Char7>;
        let v: Vector = rand::thread_rng().gen::<[i64; 32]>().to_vec().into();
        let x: Element<R> = v.into();
        let sum = x.clone() + &x;
        let want: Vec<ModularBigInt<Char7>> = x
            .coefficients()
            .iter()
            .map(|c| c.clone() + c.clone())
            .collect();
        assert_eq!(sum.coefficients().as_slice(), want);
    }

    #[test]
    fn hadamard_square() {
        type R = Cyclotomic<U16, CharZero>;
        let v: Vector = rand::thread_rng().gen::<[i64; 32]>().to_vec().into();
        let x: Element<R> = v.clone().into();
        let y: Element<R> = v.into();
        let hadamard_square = x.hadamard(&y);
        let want: Vec<ModularBigInt<CharZero>> = y
            .coefficients()
            .iter()
            .map(|c| c.clone() * c.clone())
            .collect();
        assert_eq!(hadamard_square.coefficients().as_slice(), want);
    }
}
