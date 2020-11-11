// Generated 2020-11-11T21:09:39.861362300+09:00

use crate::gen::{GenExt, Random};
use derive_new::new;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields, default)]
pub struct PaintBrush {
    #[serde(rename = "ears")]
    hieroglyph: HashMap<String, String>,
    #[serde(rename = "chief", skip_deserializing)]
    parachute: f64,
}

#[allow(unused)]
impl Random for PaintBrush {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> PaintBrush {
        PaintBrush::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Fire {
    spice: PaintBrush,
}

#[allow(unused)]
impl Random for Fire {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Fire {
        Fire::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Star {
    A(f64),
    B(Vec<u8>),
    C(Fire),
}

impl Random for Star {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Star {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Fire = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Desk")]
pub struct Perfume {}

#[allow(unused)]
impl Random for Perfume {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Perfume {
        Perfume::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Adult")]
pub struct Boss {
    #[serde(skip)]
    parachute1: Option<(i64,)>,
    square: Perfume,
}

#[allow(unused)]
impl Random for Boss {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Boss {
        Boss::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Navy {
    #[serde(rename = "child", skip_deserializing)]
    sword: bool,
}

#[allow(unused)]
impl Random for Navy {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Navy {
        Navy::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub struct Television {}

#[allow(unused)]
impl Random for Television {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Television {
        Television::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Spectrum {
    A(bool),
    B(String),
    C(HashSet<Vec<u8>>),
    D(Television),
}

impl Random for Spectrum {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Spectrum {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<Vec<u8>> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Television = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Videotape {
    prison: HashSet<bool>,
}

#[allow(unused)]
impl Random for Videotape {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Videotape {
        Videotape::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", default)]
pub struct Dung {}

#[allow(unused)]
impl Random for Dung {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Dung {
        Dung::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Horoscope {
    A(i64),
    B(f64),
    C(Vec<bool>),
    D(Dung),
}

impl Random for Horoscope {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Horoscope {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: f64 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Vec<bool> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Dung = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Pepper {
    A(String),
    B(HashMap<String, i64>),
    C(Horoscope),
}

impl Random for Pepper {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pepper {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashMap<String, i64> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Horoscope = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Sunglasses {
    A(f64),
    B(String),
    C(Videotape),
    D(Pepper),
}

impl Random for Sunglasses {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sunglasses {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Videotape = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Pepper = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", rename = "Album", deny_unknown_fields)]
pub struct Pillow {
    #[serde(rename = "sportscar")]
    chocolates: i64,
    #[serde(rename = "passport", skip_deserializing)]
    parachute2: Vec<u8>,
}

#[allow(unused)]
impl Random for Pillow {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pillow {
        Pillow::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Adult1 {
    A(bool),
    B(String),
    C(Pillow),
}

impl Random for Adult1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Adult1 {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Pillow = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase", rename = "Garden", default)]
pub struct Sex {
    #[serde(rename = "chair")]
    star: Vec<u8>,
}

#[allow(unused)]
impl Random for Sex {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sex {
        Sex::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Planet {
    electricity: Sex,
}

#[allow(unused)]
impl Random for Planet {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Planet {
        Planet::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Bathtub {
    A(Vec<u8>),
    B(HashMap<String, Sunglasses>),
    C(HashSet<Adult1>),
    D((Planet, i64)),
}

impl Random for Bathtub {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bathtub {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashMap<String, Sunglasses> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<Adult1> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: (Planet, i64) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "camelCase", rename = "Diamond", default)]
pub struct Record {
    #[serde(skip_deserializing)]
    bee: f64,
}

#[allow(unused)]
impl Random for Record {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Record {
        Record::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "School", default)]
pub struct Fork {
    #[serde(rename = "dress")]
    cycle: bool,
    #[serde(skip)]
    surveyor: Record,
}

#[allow(unused)]
impl Random for Fork {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Fork {
        Fork::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Navy1 {
    A(bool),
    B(f64),
    C(HashSet<String>),
    D((f64,)),
}

impl Random for Navy1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Navy1 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: f64 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<String> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: (f64,) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Ship {
    A(Fork),
    B(Navy1),
}

impl Random for Ship {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Ship {
        let v: usize = rng.gen_range(0, 2);
        match v {
            0 => Self::A({
                let v: Fork = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Navy1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Typewriter {
    A(bool),
    B(Vec<String>),
    C(Ship),
}

impl Random for Typewriter {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Typewriter {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<String> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Ship = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Rocket {
    vampire: Option<Vec<bool>>,
}

#[allow(unused)]
impl Random for Rocket {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Rocket {
        Rocket::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Parachute", default)]
pub struct Bible {}

#[allow(unused)]
impl Random for Bible {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bible {
        Bible::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Vulture {
    #[serde(skip)]
    sandpaper: String,
    wheelchair: Vec<i64>,
}

#[allow(unused)]
impl Random for Vulture {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Vulture {
        Vulture::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Church {
    A(i64),
    B(Vec<f64>),
    C(Vulture),
}

impl Random for Church {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Church {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<f64> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Vulture = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Passport {
    table: Option<(Vec<u8>,)>,
}

#[allow(unused)]
impl Random for Passport {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Passport {
        Passport::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Drink {
    A(i64),
    B(String),
    C(HashMap<String, i64>),
    D(HashSet<i64>),
}

impl Random for Drink {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Drink {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashMap<String, i64> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: HashSet<i64> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Fan {
    A(f64),
    B(HashSet<bool>),
    C(((Option<i64>, Vec<u8>), Passport)),
    D(Drink),
}

impl Random for Fan {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Fan {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashSet<bool> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: ((Option<i64>, Vec<u8>), Passport) = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Drink = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Comet {
    A(bool),
    B(i64),
    C(String),
    D(Vec<u8>),
}

impl Random for Comet {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Comet {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: i64 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: String = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Drill {
    #[serde(rename = "onion")]
    triangle: (String,),
}

#[allow(unused)]
impl Random for Drill {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Drill {
        Drill::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", default)]
pub struct Flower1 {}

#[allow(unused)]
impl Random for Flower1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Flower1 {
        Flower1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase", rename = "Flower", default)]
pub struct Circus {
    #[serde(flatten)]
    roof: HashMap<String, Flower1>,
    #[serde(skip_deserializing)]
    pebble: Vec<u8>,
}

#[allow(unused)]
impl Random for Circus {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Circus {
        Circus::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Boy {
    A(bool),
    B(f64),
    C(HashSet<i64>),
    D(Circus),
}

impl Random for Boy {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Boy {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: f64 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<i64> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Circus = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Girl {
    web: Boy,
    #[serde(skip_deserializing)]
    airforce1: f64,
}

#[allow(unused)]
impl Random for Girl {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Girl {
        Girl::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub struct Sandwich {
    #[serde(skip)]
    carrot: HashSet<Vec<u8>>,
    #[serde(rename = "sportscar1", skip_deserializing)]
    money: bool,
}

#[allow(unused)]
impl Random for Sandwich {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sandwich {
        Sandwich::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Butterfly {
    table1: Vec<String>,
}

#[allow(unused)]
impl Random for Butterfly {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Butterfly {
        Butterfly::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Microscope {
    A(i64),
    B(f64),
    C(Butterfly),
}

impl Random for Microscope {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Microscope {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: f64 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Butterfly = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct Bed {
    #[serde(rename = "meat", skip_deserializing)]
    egg: Vec<u8>,
}

#[allow(unused)]
impl Random for Bed {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bed {
        Bed::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Junk {
    A(f64),
    B(HashSet<i64>),
    C(()),
    D(Bed),
}

impl Random for Junk {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Junk {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashSet<i64> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: () = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Bed = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct Fruit {
    #[serde(rename = "man", skip_deserializing)]
    staircase: HashMap<String, String>,
    horoscope: Junk,
}

#[allow(unused)]
impl Random for Fruit {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Fruit {
        Fruit::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", default)]
pub struct Guitar {
    #[serde(rename = "monster")]
    carrier: String,
}

#[allow(unused)]
impl Random for Guitar {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Guitar {
        Guitar::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Telescope {
    A(bool),
    B(String),
    C(Guitar),
}

impl Random for Telescope {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Telescope {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Guitar = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Satellite", default)]
pub struct Map {}

#[allow(unused)]
impl Random for Map {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Map {
        Map::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", rename = "Hammer")]
pub struct Table1 {}

#[allow(unused)]
impl Random for Table1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Table1 {
        Table1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Tunnel {
    #[serde(skip_deserializing)]
    eyes: HashMap<String, HashMap<String, bool>>,
}

#[allow(unused)]
impl Random for Tunnel {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Tunnel {
        Tunnel::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Passport1 {
    A(bool),
    B(i64),
    C((String, Vec<u8>)),
    D(Tunnel),
}

impl Random for Passport1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Passport1 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: i64 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: (String, Vec<u8>) = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Tunnel = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Coffee {
    A(bool),
    B(f64),
    C(Vec<u8>),
}

impl Random for Coffee {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Coffee {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: f64 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Finger {
    A(bool),
    B(i64),
    C(f64),
    D(HashMap<String, Coffee>),
}

impl Random for Finger {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Finger {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: i64 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: f64 = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: HashMap<String, Coffee> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Fan1", deny_unknown_fields)]
pub struct Map1 {
    #[serde(rename = "apple")]
    flower: Option<Finger>,
    #[serde(skip)]
    onion1: String,
}

#[allow(unused)]
impl Random for Map1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Map1 {
        Map1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Roof {}

#[allow(unused)]
impl Random for Roof {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Roof {
        Roof::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Wheelchair {
    A(bool),
    B(Vec<u8>),
    C(HashMap<String, Vec<u8>>),
    D(HashSet<i64>),
}

impl Random for Wheelchair {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Wheelchair {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashMap<String, Vec<u8>> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: HashSet<i64> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(
    rename_all = "PascalCase",
    rename = "Aeroplane",
    deny_unknown_fields,
    default
)]
pub struct Room {
    balloon: i64,
}

#[allow(unused)]
impl Random for Room {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Room {
        Room::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields, default)]
pub struct Circus1 {
    spoon: i64,
}

#[allow(unused)]
impl Random for Circus1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Circus1 {
        Circus1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Pillow1 {
    A(Vec<u8>),
    B((Circus1,)),
}

impl Random for Pillow1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pillow1 {
        let v: usize = rng.gen_range(0, 2);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: (Circus1,) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Explosive {
    boy: Pillow1,
    #[serde(rename = "sunglasses")]
    rainbow: HashMap<String, Vec<u8>>,
}

#[allow(unused)]
impl Random for Explosive {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Explosive {
        Explosive::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(deny_unknown_fields)]
pub struct Sex1 {
    sword2: Vec<u8>,
}

#[allow(unused)]
impl Random for Sex1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sex1 {
        Sex1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Solid1 {
    #[serde(rename = "junk", skip_deserializing)]
    alphabet: Vec<i64>,
}

#[allow(unused)]
impl Random for Solid1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Solid1 {
        Solid1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Explosive1 {
    A(HashSet<String>),
    B((HashMap<String, f64>, Sex1)),
    C(Solid1),
}

impl Random for Explosive1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Explosive1 {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: HashSet<String> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: (HashMap<String, f64>, Sex1) = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Solid1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum PaintBrush2 {
    A(bool),
    B(String),
    C(Vec<u8>),
    D(HashMap<String, bool>),
}

impl Random for PaintBrush2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> PaintBrush2 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: HashMap<String, bool> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "School1")]
pub struct Radar {}

#[allow(unused)]
impl Random for Radar {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Radar {
        Radar::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Milkshake1 {
    A(Vec<u8>),
    B((PaintBrush2,)),
    C(Radar),
}

impl Random for Milkshake1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Milkshake1 {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: (PaintBrush2,) = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Radar = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case", rename = "Flower2", default)]
pub struct Child1 {}

#[allow(unused)]
impl Random for Child1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Child1 {
        Child1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Apple", default)]
pub struct Carrier {
    #[serde(rename = "icecream")]
    window: (),
}

#[allow(unused)]
impl Random for Carrier {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Carrier {
        Carrier::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Carpet")]
pub struct Computer {}

#[allow(unused)]
impl Random for Computer {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Computer {
        Computer::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(
    rename_all = "SCREAMING-KEBAB-CASE",
    rename = "Radar1",
    deny_unknown_fields
)]
pub struct Egg {
    finger: Computer,
}

#[allow(unused)]
impl Random for Egg {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Egg {
        Egg::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Planet2 {
    A(f64),
    B(Child1),
    C(Carrier),
    D(Option<Egg>),
}

impl Random for Planet2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Planet2 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Child1 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Carrier = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<Egg> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", default)]
pub struct Butterfly1 {
    #[serde(skip)]
    coffeeshop: Vec<u8>,
    #[serde(rename = "explosive", skip_deserializing)]
    meat1: bool,
}

#[allow(unused)]
impl Random for Butterfly1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Butterfly1 {
        Butterfly1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", default)]
pub struct Bible1 {}

#[allow(unused)]
impl Random for Bible1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bible1 {
        Bible1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Star1 {
    A(HashMap<String, String>),
    B(Vec<Vec<u8>>),
    C(Bible1),
    D(Option<Vec<u8>>),
}

impl Random for Star1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Star1 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: HashMap<String, String> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<Vec<u8>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Bible1 = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<Vec<u8>> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Crystal {
    A(f64),
    B(HashMap<String, f64>),
    C(Butterfly1),
    D(Star1),
}

impl Random for Crystal {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Crystal {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashMap<String, f64> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Butterfly1 = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Star1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "camelCase", rename = "Circus2", deny_unknown_fields)]
pub struct Highway1 {
    #[serde(rename = "passport1", skip_deserializing)]
    signature: i64,
    ring: Option<Crystal>,
}

#[allow(unused)]
impl Random for Highway1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Highway1 {
        Highway1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum PostOffice {
    A(String),
    B(HashSet<String>),
    C((bool,)),
}

impl Random for PostOffice {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> PostOffice {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashSet<String> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: (bool,) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Chair", deny_unknown_fields)]
pub struct Chocolates1 {
    window1: bool,
}

#[allow(unused)]
impl Random for Chocolates1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Chocolates1 {
        Chocolates1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Fungus {
    A(bool),
    B(String),
    C(Vec<i64>),
    D(Option<String>),
}

impl Random for Fungus {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Fungus {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Vec<i64> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<String> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub struct Chair1 {}

#[allow(unused)]
impl Random for Chair1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Chair1 {
        Chair1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename = "Hammer1")]
pub struct Eyes {}

#[allow(unused)]
impl Random for Eyes {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Eyes {
        Eyes::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Hat1", default)]
pub struct Potato1 {}

#[allow(unused)]
impl Random for Potato1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Potato1 {
        Potato1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", default)]
pub struct Gas {}

#[allow(unused)]
impl Random for Gas {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Gas {
        Gas::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Wheelchair2 {
    A(f64),
    B(String),
    C(Gas),
}

impl Random for Wheelchair2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Wheelchair2 {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Gas = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case", rename = "Microscope1")]
pub struct Meteor {}

#[allow(unused)]
impl Random for Meteor {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Meteor {
        Meteor::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
pub struct Passport2 {
    #[serde(rename = "leatherjacket", skip_deserializing)]
    spoon1: Meteor,
    #[serde(rename = "robot")]
    boy1: Vec<Vec<Option<Vec<f64>>>>,
}

#[allow(unused)]
impl Random for Passport2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Passport2 {
        Passport2::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Bible3", default)]
pub struct Foot {}

#[allow(unused)]
impl Random for Foot {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Foot {
        Foot::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Gloves1 {
    #[serde(flatten)]
    pillow: Foot,
    videotape: Vec<HashMap<String, Vec<Vec<u8>>>>,
}

#[allow(unused)]
impl Random for Gloves1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Gloves1 {
        Gloves1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpotLight {
    #[serde(rename = "bed", skip_deserializing)]
    pillow1: Option<Option<Vec<Vec<i64>>>>,
    monster1: Vec<u8>,
}

#[allow(unused)]
impl Random for SpotLight {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> SpotLight {
        SpotLight::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Sandpaper {
    #[serde(rename = "comet", skip_deserializing)]
    room: Option<((i64, HashMap<String, bool>),)>,
}

#[allow(unused)]
impl Random for Sandpaper {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sandpaper {
        Sandpaper::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Milk {
    A(i64),
    B(Sandpaper),
}

impl Random for Milk {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Milk {
        let v: usize = rng.gen_range(0, 2);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Sandpaper = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Carrier1", default)]
pub struct Tongue {}

#[allow(unused)]
impl Random for Tongue {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Tongue {
        Tongue::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Foot1 {
    #[serde(rename = "dress1", skip_deserializing)]
    finger1: Vec<bool>,
}

#[allow(unused)]
impl Random for Foot1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Foot1 {
        Foot1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Bank {
    #[serde(skip_deserializing)]
    sunglasses3: Option<bool>,
}

#[allow(unused)]
impl Random for Bank {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bank {
        Bank::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Potato2 {}

#[allow(unused)]
impl Random for Potato2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Potato2 {
        Potato2::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase", deny_unknown_fields, default)]
pub struct Sandwich1 {
    #[serde(skip)]
    software1: Vec<u8>,
    school1: bool,
}

#[allow(unused)]
impl Random for Sandwich1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sandwich1 {
        Sandwich1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Button2 {
    A(i64),
    B(Potato2),
    C(Sandwich1),
    D(Option<String>),
}

impl Random for Button2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Button2 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Potato2 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Sandwich1 = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<String> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct Stomach {
    #[serde(skip)]
    explosive2: Vec<String>,
    carrot1: f64,
}

#[allow(unused)]
impl Random for Stomach {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Stomach {
        Stomach::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Needle {
    A(HashMap<String, i64>),
    B((HashMap<String, HashMap<String, f64>>,)),
    C(Stomach),
}

impl Random for Needle {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Needle {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: HashMap<String, i64> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: (HashMap<String, HashMap<String, f64>>,) = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Stomach = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Button1 {
    A(HashMap<String, String>),
    B(Vec<Option<bool>>),
    C(HashSet<Button2>),
    D(Needle),
}

impl Random for Button1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Button1 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: HashMap<String, String> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<Option<bool>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<Button2> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Needle = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Navy3 {
    A(String),
    B(Bank),
    C(Button1),
}

impl Random for Navy3 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Navy3 {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Bank = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Button1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Gate {
    A(Vec<u8>),
    B(HashSet<i64>),
    C(Foot1),
    D(Navy3),
}

impl Random for Gate {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Gate {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashSet<i64> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Foot1 = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Navy3 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "PascalCase", rename = "IceCream")]
pub struct Chair2 {
    #[serde(skip)]
    brain: Option<Tongue>,
    #[serde(rename = "kaleidoscope")]
    gas: Gate,
}

#[allow(unused)]
impl Random for Chair2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Chair2 {
        Chair2::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Hammer2")]
pub struct Shower {}

#[allow(unused)]
impl Random for Shower {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Shower {
        Shower::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(
    rename_all = "SCREAMING_SNAKE_CASE",
    rename = "Pants",
    deny_unknown_fields,
    default
)]
pub struct Aircraft1 {
    crystal: Shower,
}

#[allow(unused)]
impl Random for Aircraft1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Aircraft1 {
        Aircraft1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields, default)]
pub struct Meat2 {
    #[serde(rename = "skeleton")]
    signature1: Aircraft1,
}

#[allow(unused)]
impl Random for Meat2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Meat2 {
        Meat2::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct Rifle {
    #[serde(skip_deserializing)]
    foot: i64,
    #[serde(rename = "pocket")]
    pyramid: Option<Option<Vec<u8>>>,
}

#[allow(unused)]
impl Random for Rifle {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Rifle {
        Rifle::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct Shop1 {
    #[serde(rename = "fruit")]
    teeth: Option<bool>,
}

#[allow(unused)]
impl Random for Shop1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Shop1 {
        Shop1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Baby {
    A(Vec<i64>),
    B(()),
    C(Shop1),
}

impl Random for Baby {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Baby {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: Vec<i64> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: () = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Shop1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct A {
    a: bool,
    b: i64,
}

#[allow(unused)]
impl Random for A {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> A {
        A::new(rng.gen_ext(), rng.gen_ext())
    }
}

macro_rules! gen {
    ($encoder:path) => {{
        let mut rng = rand::thread_rng();
        let mut ret = Vec::<Vec<u8>>::new();

        let v: Star = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Vec<u8>,) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Boss = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Vec<()>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, HashSet<bool>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Option<bool>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Navy = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Spectrum> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Bathtub = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Typewriter = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<i64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Rocket = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Bible> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: () = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Church = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Fan> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Comet = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Drill = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Girl = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<(Sandwich, HashSet<Vec<u8>>)> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Microscope = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Fruit = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Telescope = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Vec<Option<i64>>, HashMap<String, bool>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Map = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Table1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Passport1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Map1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Roof, (Vec<u8>,)) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Wheelchair = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Room> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<f64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Explosive> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: ((f64,), HashSet<bool>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Explosive1> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Milkshake1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Planet2 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Highway1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: PostOffice = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (HashMap<String, bool>, Vec<u8>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Chocolates1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Fungus = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Chair1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Eyes = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Potato1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Wheelchair2 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<(bool,)> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Passport2 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Gloves1, HashSet<i64>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: SpotLight = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Milk = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Chair2, (HashMap<String, Meat2>,)) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, HashMap<String, Rifle>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Baby = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, HashMap<String, f64>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: A = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        ret
    }};
}
