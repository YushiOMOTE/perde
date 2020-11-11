// Generated 2020-11-10T09:24:17.083826175+09:00

use crate::gen::{GenExt, Random};
use derive_new::new;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase", deny_unknown_fields, default)]
pub struct Bowl {
    #[serde(rename = "meteor")]
    carrot: HashMap<String, i64>,
}

#[allow(unused)]
impl Random for Bowl {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bowl {
        Bowl::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Shoes {
    A(i64),
    B(Vec<Vec<u8>>),
    C(HashSet<bool>),
    D(Bowl),
}

impl Random for Shoes {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Shoes {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<Vec<u8>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<bool> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Bowl = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Album", deny_unknown_fields)]
pub struct Signature {
    sandwich: Vec<u8>,
}

#[allow(unused)]
impl Random for Signature {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Signature {
        Signature::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Mist {
    A(i64),
    B(String),
    C(HashMap<String, HashSet<i64>>),
}

impl Random for Mist {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Mist {
        let v: usize = rng.gen_range(0, 3);
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
                let v: HashMap<String, HashSet<i64>> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(default)]
pub struct Worm {
    compass1: String,
}

#[allow(unused)]
impl Random for Worm {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Worm {
        Worm::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Bee")]
pub struct Butterfly {
    record1: Option<Vec<u8>>,
}

#[allow(unused)]
impl Random for Butterfly {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Butterfly {
        Butterfly::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Leg {
    #[serde(skip)]
    square: bool,
    torpedo: (Vec<u8>, bool),
}

#[allow(unused)]
impl Random for Leg {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Leg {
        Leg::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Gloves", deny_unknown_fields)]
pub struct Thermometer {
    mouth: i64,
}

#[allow(unused)]
impl Random for Thermometer {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Thermometer {
        Thermometer::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Carrot {
    A(bool),
    B(i64),
    C(Vec<u8>),
}

impl Random for Carrot {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Carrot {
        let v: usize = rng.gen_range(0, 3);
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
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Needle1 {
    A(bool),
    B(String),
    C(Vec<u8>),
    D(HashSet<bool>),
}

impl Random for Needle1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Needle1 {
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
                let v: HashSet<bool> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Planet {
    #[serde(rename = "guitar")]
    room: Carrot,
    carpet: Needle1,
}

#[allow(unused)]
impl Random for Planet {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Planet {
        Planet::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Needle {
    A(String),
    B(Thermometer),
    C(Planet),
    D(Option<i64>),
}

impl Random for Needle {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Needle {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Thermometer = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Planet = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<i64> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Eraser {}

#[allow(unused)]
impl Random for Eraser {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Eraser {
        Eraser::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", default)]
pub struct Floodlight {
    #[serde(skip_deserializing)]
    cycle: Vec<String>,
}

#[allow(unused)]
impl Random for Floodlight {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Floodlight {
        Floodlight::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(
    rename_all = "SCREAMING-KEBAB-CASE",
    rename = "Game",
    deny_unknown_fields,
    default
)]
pub struct Rope {
    mouth1: Option<Vec<u8>>,
}

#[allow(unused)]
impl Random for Rope {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Rope {
        Rope::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Man", deny_unknown_fields, default)]
pub struct Sandwich {
    #[serde(skip)]
    fan: bool,
    girl: f64,
}

#[allow(unused)]
impl Random for Sandwich {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sandwich {
        Sandwich::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", rename = "Television", default)]
pub struct Satellite {
    restaurant: Option<Vec<u8>>,
}

#[allow(unused)]
impl Random for Satellite {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Satellite {
        Satellite::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Book {
    A(Vec<u8>),
    B(Sandwich),
    C(Satellite),
}

impl Random for Book {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Book {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Sandwich = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Satellite = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "PascalCase")]
pub struct Weapon {
    #[serde(rename = "rifle1")]
    book: Book,
    #[serde(rename = "toilet", skip_deserializing)]
    ring: String,
}

#[allow(unused)]
impl Random for Weapon {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Weapon {
        Weapon::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Coffee {
    A(String),
    B((bool,)),
    C(Rope),
    D(Weapon),
}

impl Random for Coffee {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Coffee {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: (bool,) = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Rope = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Weapon = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Chisel {}

#[allow(unused)]
impl Random for Chisel {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Chisel {
        Chisel::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Circle {
    A(bool),
    B(Vec<u8>),
    C((Vec<String>, bool)),
    D(Chisel),
}

impl Random for Circle {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Circle {
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
                let v: (Vec<String>, bool) = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Chisel = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(
    rename_all = "snake_case",
    rename = "CoffeeShop",
    deny_unknown_fields,
    default
)]
pub struct Bottle {
    arm: Vec<f64>,
}

#[allow(unused)]
impl Random for Bottle {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bottle {
        Bottle::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Money {
    A(bool),
    B(f64),
    C(Vec<u8>),
}

impl Random for Money {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Money {
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

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Potato", default)]
pub struct Sex {
    restaurant1: bool,
}

#[allow(unused)]
impl Random for Sex {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sex {
        Sex::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Apple")]
pub struct Flower {
    #[serde(flatten, skip_deserializing)]
    explosive: Sex,
}

#[allow(unused)]
impl Random for Flower {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Flower {
        Flower::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Skeleton", default)]
pub struct Vulture {}

#[allow(unused)]
impl Random for Vulture {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Vulture {
        Vulture::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Magnet {
    A(bool),
    B(HashSet<String>),
    C((Vulture,)),
}

impl Random for Magnet {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Magnet {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashSet<String> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: (Vulture,) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum CoffeeShop1 {
    A(bool),
    B(i64),
    C(String),
    D(HashSet<Vec<u8>>),
}

impl Random for CoffeeShop1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> CoffeeShop1 {
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
                let v: HashSet<Vec<u8>> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Prison {
    A(f64),
    B(String),
    C((String, (Vec<f64>, String))),
}

impl Random for Prison {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Prison {
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
                let v: (String, (Vec<f64>, String)) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Ring {
    A(String),
    B(Vec<bool>),
    C(Prison),
}

impl Random for Ring {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Ring {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<bool> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Prison = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", rename = "Bed")]
pub struct Film {}

#[allow(unused)]
impl Random for Film {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Film {
        Film::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case", rename = "Bird")]
pub struct Salt {
    tennisracquet: Option<(Option<(bool,)>,)>,
}

#[allow(unused)]
impl Random for Salt {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Salt {
        Salt::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Computer {
    A(bool),
    B(i64),
    C(f64),
    D(HashMap<String, i64>),
}

impl Random for Computer {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Computer {
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
                let v: HashMap<String, i64> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Torpedo")]
pub struct Bird1 {
    stomach: (Vec<String>,),
}

#[allow(unused)]
impl Random for Bird1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bird1 {
        Bird1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Drill {
    A(Vec<u8>),
    B(HashMap<String, Option<Vec<f64>>>),
    C(HashSet<bool>),
    D(Bird1),
}

impl Random for Drill {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Drill {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashMap<String, Option<Vec<f64>>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<bool> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Bird1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(
    rename_all = "SCREAMING-KEBAB-CASE",
    rename = "Kaleidoscope",
    deny_unknown_fields
)]
pub struct Aircraft1 {
    #[serde(rename = "rocket")]
    plane: (Computer, i64),
    pocket: Drill,
}

#[allow(unused)]
impl Random for Aircraft1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Aircraft1 {
        Aircraft1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Vampire {
    milk: f64,
}

#[allow(unused)]
impl Random for Vampire {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Vampire {
        Vampire::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Robot {
    crystal: f64,
}

#[allow(unused)]
impl Random for Robot {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Robot {
        Robot::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Prison1", default)]
pub struct Feather {}

#[allow(unused)]
impl Random for Feather {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Feather {
        Feather::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Ship {
    A(bool),
    B(Vec<u8>),
    C(HashSet<Vec<u8>>),
    D(Feather),
}

impl Random for Ship {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Ship {
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
                let v: HashSet<Vec<u8>> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Feather = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "lowercase")]
pub struct Boss {
    #[serde(skip)]
    kaleidoscope: f64,
    #[serde(rename = "restaurant2")]
    electricity: Ship,
}

#[allow(unused)]
impl Random for Boss {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Boss {
        Boss::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Book2", default)]
pub struct Meteor {
    sportscar: i64,
    #[serde(skip)]
    button: Vec<u8>,
}

#[allow(unused)]
impl Random for Meteor {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Meteor {
        Meteor::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Car4 {
    A(String),
    B(HashMap<String, Meteor>),
    C(HashSet<bool>),
    D((String,)),
}

impl Random for Car4 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Car4 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashMap<String, Meteor> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<bool> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: (String,) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields, default)]
pub struct Pendulum {
    #[serde(skip)]
    chisel: f64,
    festival: Vec<u8>,
}

#[allow(unused)]
impl Random for Pendulum {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pendulum {
        Pendulum::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Snail {}

#[allow(unused)]
impl Random for Snail {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Snail {
        Snail::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase", rename = "Bible", default)]
pub struct Radar {}

#[allow(unused)]
impl Random for Radar {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Radar {
        Radar::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case", rename = "Software", default)]
pub struct Grapes1 {}

#[allow(unused)]
impl Random for Grapes1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Grapes1 {
        Grapes1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Shoes1 {
    A(String),
    B(Vec<u8>),
    C(HashMap<String, HashMap<String, i64>>),
    D((f64, bool)),
}

impl Random for Shoes1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Shoes1 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashMap<String, HashMap<String, i64>> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: (f64, bool) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Chair {
    A(i64),
}

impl Random for Chair {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Chair {
        let v: usize = rng.gen_range(0, 1);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub struct Radar1 {}

#[allow(unused)]
impl Random for Radar1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Radar1 {
        Radar1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Man1", deny_unknown_fields)]
pub struct Parachute1 {
    #[serde(skip)]
    window: bool,
    star: bool,
}

#[allow(unused)]
impl Random for Parachute1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Parachute1 {
        Parachute1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(
    rename_all = "camelCase",
    rename = "Air1",
    deny_unknown_fields,
    default
)]
pub struct Drum {
    #[serde(skip_deserializing)]
    spiral: String,
    #[serde(rename = "horse2")]
    flower: bool,
}

#[allow(unused)]
impl Random for Drum {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Drum {
        Drum::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase", rename = "Wheelchair", default)]
pub struct Hat {}

#[allow(unused)]
impl Random for Hat {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Hat {
        Hat::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Satellite2", deny_unknown_fields)]
pub struct Spiral {
    bathtub1: Vec<Hat>,
    #[serde(skip_deserializing)]
    book1: i64,
}

#[allow(unused)]
impl Random for Spiral {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Spiral {
        Spiral::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", rename = "Ring1")]
pub struct Apple1 {}

#[allow(unused)]
impl Random for Apple1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Apple1 {
        Apple1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename = "Pyramid", default)]
pub struct XRay1 {
    #[serde(skip)]
    boss1: Spiral,
    #[serde(skip_deserializing)]
    kitchen: Apple1,
}

#[allow(unused)]
impl Random for XRay1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> XRay1 {
        XRay1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Tiger")]
pub struct Bee2 {}

#[allow(unused)]
impl Random for Bee2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bee2 {
        Bee2::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Army {
    A(i64),
    B(f64),
    C(Vec<u8>),
}

impl Random for Army {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Army {
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
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(
    rename_all = "kebab-case",
    rename = "Pendulum1",
    deny_unknown_fields,
    default
)]
pub struct Aeroplane3 {
    tongue1: bool,
}

#[allow(unused)]
impl Random for Aeroplane3 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Aeroplane3 {
        Aeroplane3::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields, default)]
pub struct Torpedo2 {
    #[serde(skip)]
    necklace: bool,
    tiger: HashSet<Vec<u8>>,
}

#[allow(unused)]
impl Random for Torpedo2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Torpedo2 {
        Torpedo2::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Diamond {
    A(i64),
    B(String),
    C(Vec<u8>),
}

impl Random for Diamond {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Diamond {
        let v: usize = rng.gen_range(0, 3);
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
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Onion {
    A(HashMap<String, Vec<bool>>),
    B((i64, Aeroplane3)),
    C(Torpedo2),
    D(Diamond),
}

impl Random for Onion {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Onion {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: HashMap<String, Vec<bool>> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: (i64, Aeroplane3) = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Torpedo2 = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Diamond = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum CompactDisc {
    A(f64),
    B(Option<Army>),
    C(Onion),
}

impl Random for CompactDisc {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> CompactDisc {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Option<Army> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Onion = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Bible1 {
    A(()),
    B(Bee2),
    C(Option<Option<Vec<u8>>>),
    D(CompactDisc),
}

impl Random for Bible1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bible1 {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: () = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Bee2 = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Option<Option<Vec<u8>>> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: CompactDisc = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(
    rename_all = "UPPERCASE",
    rename = "Fungus",
    deny_unknown_fields,
    default
)]
pub struct CompactDisc1 {
    #[serde(skip_deserializing)]
    guitar1: f64,
    map: HashMap<String, f64>,
}

#[allow(unused)]
impl Random for CompactDisc1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> CompactDisc1 {
        CompactDisc1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Foot {
    A(f64),
    B(Vec<u8>),
    C(HashMap<String, i64>),
    D(CompactDisc1),
}

impl Random for Foot {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Foot {
        let v: usize = rng.gen_range(0, 4);
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
                let v: HashMap<String, i64> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: CompactDisc1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Hieroglyph {
    A(bool),
    B(HashSet<bool>),
    C(Foot),
}

impl Random for Hieroglyph {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Hieroglyph {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashSet<bool> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Foot = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(deny_unknown_fields)]
pub struct Bottle2 {
    meteor2: Option<Hieroglyph>,
    #[serde(skip)]
    cappuccino: bool,
}

#[allow(unused)]
impl Random for Bottle2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bottle2 {
        Bottle2::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Festival")]
pub struct Map {
    #[serde(rename = "pebble")]
    rocket1: Bottle2,
    man: f64,
}

#[allow(unused)]
impl Random for Map {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Map {
        Map::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Prison2 {}

#[allow(unused)]
impl Random for Prison2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Prison2 {
        Prison2::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Saddle {
    A(String),
    B(HashMap<String, Vec<Option<()>>>),
    C((Prison2,)),
}

impl Random for Saddle {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Saddle {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: String = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashMap<String, Vec<Option<()>>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: (Prison2,) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Shop {
    #[serde(skip)]
    satellite: i64,
    #[serde(flatten)]
    hose1: HashMap<String, Vec<u8>>,
}

#[allow(unused)]
impl Random for Shop {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Shop {
        Shop::new(rng.gen_ext(), rng.gen_ext())
    }
}

macro_rules! gen {
    ($encoder:path) => {{
        let mut rng = rand::thread_rng();
        let mut ret = Vec::<Vec<u8>>::new();

        let v: HashSet<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Shoes = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Signature> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<f64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Mist, Vec<Vec<u8>>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, f64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: ((bool,), HashSet<i64>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Worm = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<(Vec<u8>, String)> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Butterfly = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Leg = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (HashSet<String>, Option<Needle>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, HashSet<Vec<u8>>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, HashMap<String, HashMap<String, bool>>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Eraser = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Floodlight = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: () = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: () = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Option<Coffee>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Circle = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Bottle = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<i64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Vec<Money>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Flower = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Magnet = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: CoffeeShop1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Ring = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Film> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: () = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Salt = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, HashSet<bool>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (String, Aircraft1) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, HashMap<String, HashSet<Vec<u8>>>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vampire = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Robot = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Boss = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Car4 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: () = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Pendulum = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Snail = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Radar = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Vec<Grapes1>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<f64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<f64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Shoes1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Chair = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<HashMap<String, bool>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Radar1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Parachute1> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Drum = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: XRay1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, i64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Bible1> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Map = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<i64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Saddle = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Shop,) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        ret
    }};
}
