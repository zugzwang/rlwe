use core::marker::PhantomData;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul, Rem, SubAssign};

use generic_array::{ArrayLength, GenericArray};
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use typenum::{PowerOfTwo, Unsigned};

use crate::traits::{
    Characteristic, Element, FieldElement, RlweRing, Vector,
};

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
        let ch = C::to_biguint();
        if ch == Zero::zero() {
            ModularBigInt {
                representant: x,
                modulus: PhantomData,
            }
        } else {
            unimplemented!("reduction modulo p")
        }
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
        let modulus: BigInt = C::to_biguint().into();
        if modulus == Zero::zero() {
            let val = self.representant.clone() + other.representant.clone();
            Self {
                representant: val,
                modulus: PhantomData,
            }
        } else {
            unimplemented!()
        }
    }
}

impl<C: Characteristic> AddAssign for ModularBigInt<C> {
    fn add_assign(&mut self, other: Self) {
        let modulus: BigInt = C::to_biguint().into();
        if modulus == Zero::zero() {
            let val = self.representant.clone() + other.representant.clone();
            *self = Self {
                representant: val,
                modulus: PhantomData,
            }
        } else {
            unimplemented!()
        }
    }
}

impl<C: Characteristic> SubAssign for ModularBigInt<C> {
    fn sub_assign(&mut self, other: Self) {
        let modulus: BigInt = C::to_biguint().into();
        if modulus == Zero::zero() {
            let val = self.representant.clone() - other.representant.clone();
            *self = Self {
                representant: val,
                modulus: PhantomData,
            }
        } else {
            unimplemented!()
        }
    }
}

impl<C: Characteristic> Mul for ModularBigInt<C> {
    type Output = Self;

    fn mul(self, _other: Self) -> Self::Output {
        todo!()
    }
}

impl<C: Characteristic> Rem<BigUint> for ModularBigInt<C> {
    type Output = Self;

    fn rem(self, _other: BigUint) -> Self::Output {
        todo!()
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

impl<C: Characteristic, T: ArrayLength<ModularBigInt<C>> + PowerOfTwo>
    RlweRing for Cyclotomic<T, C>
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
    use rand::Rng;
    use typenum::consts::{U16, U4};

    use super::*;
    use crate::traits::CharZero;

    #[test]
    fn convert() {
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
    fn double() {
        type R = Cyclotomic<U16, CharZero>;
        let v: Vector = rand::thread_rng().gen::<[i64; 32]>().to_vec().into();
        let x: Element<R> = v.into();
        let sum = x.clone() + x.clone();
        let doubled: Vec<ModularBigInt<CharZero>> = x
            .coefficients()
            .iter()
            .map(|c| c.clone() + c.clone())
            .collect();
        assert_eq!(sum.coefficients().as_slice(), doubled);
    }
}
