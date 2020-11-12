#![allow(unused)]
// Generated 2020-11-12T10:34:45.713872147+09:00

use crate::gen::{GenExt, Random};
use derive_new::new;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preset0 {
    apple_pen: bool,
    pen_pineapple: i64,
}

#[allow(unused)]
impl Random for Preset0 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Preset0 {
        Preset0::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Preset1 {
    #[serde(rename = "hage")]
    a: bool,
    b: i64,
}

#[allow(unused)]
impl Random for Preset1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Preset1 {
        Preset1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Preset3 {
    x: bool,
    y: i64,
}

#[allow(unused)]
impl Random for Preset3 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Preset3 {
        Preset3::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Preset2 {
    #[serde(flatten)]
    a: Preset3,
    b: i64,
}

#[allow(unused)]
impl Random for Preset2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Preset2 {
        Preset2::new(rng.gen_ext(), rng.gen_ext())
    }
}

macro_rules! gen {
    ($encoder:path) => {{
        let mut rng = rand::thread_rng();
        let mut ret = Vec::<Vec<u8>>::new();

        let v: Preset0 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Preset1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Preset2 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        ret
    }};
}
