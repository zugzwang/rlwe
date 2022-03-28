use std::ops::{Add, Mul};

use generic_array::{ArrayLength, GenericArray};
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use typenum::{PowerOfTwo, Unsigned};

pub trait Characteristic: Clone {
    fn to_biguint() -> BigUint;
}

#[macro_export]
macro_rules! characteristic {
    ($name: ident, $value: expr) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {}
        impl Characteristic for $name {
            fn to_biguint() -> BigUint {
                $value
            }
        }
    };
}

pub trait FieldElement: From<BigInt> + Clone + Add + Mul {
    type Char: Characteristic;
}

pub trait RlweRing: Sized {
    type Coefficient: FieldElement;
    type Degree: ArrayLength<Self::Coefficient> + PowerOfTwo;

    #[must_use]
    fn mul(a: Element<Self>, b: Element<Self>) -> Element<Self>;

    #[must_use]
    fn degree() -> usize {
        Self::Degree::to_usize()
    }

    #[must_use]
    fn characteristic() -> BigUint {
        <Self::Coefficient as FieldElement>::Char::to_biguint()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element<R: RlweRing> {
    pub(crate) coefficients: GenericArray<R::Coefficient, R::Degree>,
}

impl<R: RlweRing> Element<R> {
    pub fn coefficients(&self) -> &GenericArray<R::Coefficient, R::Degree> {
        &self.coefficients
    }

    pub fn at(&self, i: usize) -> &R::Coefficient {
        &self.coefficients[i]
    }
}

impl<R: RlweRing> Add<&Element<R>> for Element<R>
where
    Vec<R::Coefficient>: FromIterator<<R::Coefficient as Add>::Output>,
{
    type Output = Self;

    #[must_use]
    fn add(self, other: &Self) -> Self::Output {
        // TODO: Perform modular reduction on the vec, to avoid using the
        // characteristic too often.
        let slice: Vec<R::Coefficient> = self
            .coefficients
            .into_iter()
            .zip(other.coefficients.clone())
            .map(|(x, y)| x + y)
            .collect();
        let coeffs =
            GenericArray::<R::Coefficient, R::Degree>::clone_from_slice(&slice);
        Element::<R> {
            coefficients: coeffs,
        }
    }
}

impl<R: RlweRing> Element<R>
where
    Vec<R::Coefficient>: FromIterator<<R::Coefficient as Mul>::Output>,
{
    pub fn hadamard(self, other: &Self) -> Self {
        // TODO: Perform modular reduction on the vec, to avoid using the
        // characteristic too often.
        let slice: Vec<R::Coefficient> = self
            .coefficients
            .clone()
            .into_iter()
            .zip(other.coefficients.clone())
            .map(|(x, y)| x * y)
            .collect();
        let coeffs =
            GenericArray::<R::Coefficient, R::Degree>::clone_from_slice(&slice);
        Element::<R> {
            coefficients: coeffs,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    pub(crate) coordinates: Vec<BigInt>,
}

impl Vector {
    pub fn coordinates(&self) -> &Vec<BigInt> {
        &self.coordinates
    }
}

impl From<Vec<i64>> for Vector {
    fn from(x: Vec<i64>) -> Self {
        let coordinates: Vec<BigInt> = x.iter().map(|x| (*x).into()).collect();
        Self { coordinates }
    }
}

impl From<Vec<BigInt>> for Vector {
    fn from(coordinates: Vec<BigInt>) -> Self {
        Self { coordinates }
    }
}

characteristic!(CharZero, Zero::zero());
