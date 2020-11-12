#![allow(unused)]
// Generated 2020-11-12T09:42:53.691070089+09:00

use crate::gen::{GenExt, Random};
use derive_new::new;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Preset0 {
    #[serde(rename = "hage")]
    a: bool,
    b: i64,
}

#[allow(unused)]
impl Random for Preset0 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Preset0 {
        Preset0::new(rng.gen_ext(), rng.gen_ext())
    }
}

macro_rules! gen {
    ($encoder:path) => {{
        let mut rng = rand::thread_rng();
        let mut ret = Vec::<Vec<u8>>::new();

        let v: Preset0 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        ret
    }};
}
