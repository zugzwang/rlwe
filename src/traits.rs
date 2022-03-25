use std::ops::{Add, Mul};

use generic_array::{ArrayLength, GenericArray};
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use typenum::{PowerOfTwo, Unsigned};

pub trait Characteristic: Clone {
    fn to_biguint() -> BigUint;
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

impl<R: RlweRing> Add for Element<R>
where
    Vec<R::Coefficient>: FromIterator<<R::Coefficient as Add>::Output>,
{
    type Output = Self;

    #[must_use]
    fn add(self, other: Self::Output) -> Self::Output {
        let slice: Vec<R::Coefficient> = self
            .coefficients
            .into_iter()
            .zip(other.coefficients)
            .map(|(x, y)| x + y)
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

    pub fn hadamard(&self, _other: &Self) -> Self {
        todo!()
    }
}

impl From<Vec<i64>> for Vector {
    fn from(x: Vec<i64>) -> Self {
        let coordinates: Vec<BigInt> = x.iter().map(|x| (*x).into()).collect();
        Self { coordinates }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CharZero {}
impl Characteristic for CharZero { fn to_biguint() -> BigUint { Zero::zero() } }
