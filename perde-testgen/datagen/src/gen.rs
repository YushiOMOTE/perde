use rand::{
    distributions::{Alphanumeric, Distribution},
    Rng,
};
use std::collections::{HashMap, HashSet};

pub trait Random: Sized {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

pub trait GenExt {
    fn gen_ext<T>(&mut self) -> T
    where
        T: Random;
}

impl<T> GenExt for T
where
    T: Rng + ?Sized,
{
    fn gen_ext<U>(&mut self) -> U
    where
        U: Random,
    {
        U::random(self)
    }
}

impl Random for u8 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.gen()
    }
}

impl Random for bool {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.gen()
    }
}

impl Random for i64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.gen()
    }
}

impl Random for u64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.gen()
    }
}

impl Random for f64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.gen()
    }
}

impl Random for String {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> String {
        Alphanumeric.sample_iter(rng).take(7).collect()
    }
}

impl<T> Random for Vec<T>
where
    T: Random,
{
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Vec<T> {
        let len: usize = rng.gen_range(0, 10);
        (0..len).map(|_| rng.gen_ext()).collect()
    }
}

impl<K, V> Random for HashMap<K, V>
where
    K: Random + std::hash::Hash + Eq,
    V: Random,
{
    fn random<R: Rng + ?Sized>(rng: &mut R) -> HashMap<K, V> {
        let len: usize = rng.gen_range(0, 10);
        (0..len).map(|_| (rng.gen_ext(), rng.gen_ext())).collect()
    }
}

impl<T> Random for HashSet<T>
where
    T: Random + std::hash::Hash + Eq,
{
    fn random<R: Rng + ?Sized>(rng: &mut R) -> HashSet<T> {
        let len: usize = rng.gen_range(0, 10);
        (0..len).map(|_| rng.gen_ext()).collect()
    }
}

impl<T> Random for Option<T>
where
    T: Random,
{
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Option<T> {
        if rng.gen() {
            Some(rng.gen_ext())
        } else {
            None
        }
    }
}

impl Random for () {
    fn random<R: Rng + ?Sized>(_: &mut R) -> () {
        ()
    }
}

macro_rules! impl_tuple {
    ($(($($tp:tt),*);)*) => {
        $(impl<$($tp),*> Random for ($($tp,)*)
        where
            $($tp: Random),*
        {
            fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
                ($({ let v: $tp = rng.gen_ext(); v },)*)
            }
        })*
    }
}

impl_tuple!(
    (A);
    (A, B);
    (A, B, C);
    (A, B, C, D);
    (A, B, C, D, E);
    (A, B, C, D, E, F);
);
