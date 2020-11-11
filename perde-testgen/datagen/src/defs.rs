// Generated 2020-11-10T10:30:34.894755067+09:00

use crate::gen::{GenExt, Random};
use derive_new::new;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename = "Potato")]
pub struct Water {}

#[allow(unused)]
impl Random for Water {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Water {
        Water::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Child {
    A(f64),
    B(Water),
    C(Option<f64>),
}

impl Random for Child {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Child {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Water = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Option<f64> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Bed {
    pebble: Child,
}

#[allow(unused)]
impl Random for Bed {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bed {
        Bed::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Arm {
    A(HashSet<Vec<u8>>),
    B(Bed),
}

impl Random for Arm {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Arm {
        let v: usize = rng.gen_range(0, 2);
        match v {
            0 => Self::A({
                let v: HashSet<Vec<u8>> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Bed = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Shower {
    #[serde(rename = "egg")]
    sword: Arm,
}

#[allow(unused)]
impl Random for Shower {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Shower {
        Shower::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", default)]
pub struct Shower1 {}

#[allow(unused)]
impl Random for Shower1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Shower1 {
        Shower1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Kitchen", default)]
pub struct Rifle {
    #[serde(skip)]
    god: Option<Shower1>,
    swimmingpool: HashSet<bool>,
}

#[allow(unused)]
impl Random for Rifle {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Rifle {
        Rifle::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SwimmingPool {
    A(Vec<u8>),
    B(Vec<Vec<u8>>),
}

impl Random for SwimmingPool {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> SwimmingPool {
        let v: usize = rng.gen_range(0, 2);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<Vec<u8>> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename = "Baby2")]
pub struct Leg {
    #[serde(rename = "carrier")]
    backpack: HashMap<String, SwimmingPool>,
}

#[allow(unused)]
impl Random for Leg {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Leg {
        Leg::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Banana {
    A(bool),
    B(f64),
    C(HashMap<String, f64>),
    D(HashSet<String>),
}

impl Random for Banana {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Banana {
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
                let v: HashMap<String, f64> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: HashSet<String> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Vulture {
    A(bool),
    B(i64),
    C(HashMap<String, i64>),
}

impl Random for Vulture {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Vulture {
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
                let v: HashMap<String, i64> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", default)]
pub struct Bathroom {}

#[allow(unused)]
impl Random for Bathroom {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bathroom {
        Bathroom::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", rename = "Church", default)]
pub struct LeatherJacket {
    #[serde(skip)]
    rope: Vec<u8>,
    map: Bathroom,
}

#[allow(unused)]
impl Random for LeatherJacket {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> LeatherJacket {
        LeatherJacket::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Planet1 {
    ice: f64,
}

#[allow(unused)]
impl Random for Planet1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Planet1 {
        Planet1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Table {
    A(bool),
    B((HashMap<String, Vec<LeatherJacket>>,)),
    C(Planet1),
}

impl Random for Table {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Table {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: (HashMap<String, Vec<LeatherJacket>>,) = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Planet1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename = "Chisel", deny_unknown_fields)]
pub struct Ears {
    rope1: (),
    computer: f64,
}

#[allow(unused)]
impl Random for Ears {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Ears {
        Ears::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Gate {
    chisel: HashMap<String, Vec<bool>>,
}

#[allow(unused)]
impl Random for Gate {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Gate {
        Gate::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Rainbow {
    maze: String,
}

#[allow(unused)]
impl Random for Rainbow {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Rainbow {
        Rainbow::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Tiger {
    A(Vec<u8>),
    B(Vec<(Vec<u8>,)>),
    C(Gate),
    D(Option<Rainbow>),
}

impl Random for Tiger {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Tiger {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<(Vec<u8>,)> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Gate = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<Rainbow> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Bank {
    A(HashMap<String, HashMap<String, f64>>),
    B(Vec<Vec<HashSet<bool>>>),
    C(Ears),
    D(Tiger),
}

impl Random for Bank {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bank {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: HashMap<String, HashMap<String, f64>> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<Vec<HashSet<bool>>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Ears = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Tiger = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Baby3 {
    A(f64),
    B(Vec<u8>),
    C(Bank),
}

impl Random for Baby3 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Baby3 {
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
                let v: Bank = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Coffee", default)]
pub struct Square {
    #[serde(rename = "surveyor")]
    slave: String,
    #[serde(skip)]
    parachute: Vec<u8>,
}

#[allow(unused)]
impl Random for Square {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Square {
        Square::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Explosive {}

#[allow(unused)]
impl Random for Explosive {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Explosive {
        Explosive::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Bridge1 {
    A(i64),
    B(f64),
    C(HashMap<String, bool>),
    D(Option<bool>),
}

impl Random for Bridge1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bridge1 {
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
                let v: HashMap<String, bool> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<bool> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case", default)]
pub struct Liquid {}

#[allow(unused)]
impl Random for Liquid {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Liquid {
        Liquid::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum Eraser {
    A(bool),
    B(Vec<HashSet<String>>),
    C((i64, Option<String>)),
    D(Liquid),
}

impl Random for Eraser {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Eraser {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<HashSet<String>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: (i64, Option<String>) = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Liquid = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", default)]
pub struct Album {}

#[allow(unused)]
impl Random for Album {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Album {
        Album::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(
    rename_all = "SCREAMING-KEBAB-CASE",
    rename = "Church2",
    deny_unknown_fields
)]
pub struct Pepper {
    #[serde(rename = "butterfly")]
    circus: i64,
    #[serde(skip)]
    car: String,
}

#[allow(unused)]
impl Random for Pepper {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pepper {
        Pepper::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase", rename = "Rock")]
pub struct Shower2 {
    #[serde(skip)]
    vampire1: Pepper,
    woman: Vec<u8>,
}

#[allow(unused)]
impl Random for Shower2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Shower2 {
        Shower2::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Shoes")]
pub struct Spoon1 {
    #[serde(skip)]
    highway: i64,
    hose: Shower2,
}

#[allow(unused)]
impl Random for Spoon1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Spoon1 {
        Spoon1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum IceCream {
    A(bool),
    B(Album),
    C(Spoon1),
}

impl Random for IceCream {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> IceCream {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Album = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Spoon1 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Parachute")]
pub struct Signature1 {}

#[allow(unused)]
impl Random for Signature1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Signature1 {
        Signature1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Thermometer {
    A(bool),
    B(i64),
    C(f64),
}

impl Random for Thermometer {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Thermometer {
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
                let v: f64 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Treadmill {
    A(i64),
    B(Vec<u8>),
    C(Option<String>),
}

impl Random for Treadmill {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Treadmill {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Option<String> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Sandwich {
    A(i64),
    B(Vec<HashMap<String, Thermometer>>),
    C(Option<Treadmill>),
}

impl Random for Sandwich {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sandwich {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<HashMap<String, Thermometer>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Option<Treadmill> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", default)]
pub struct Circle {}

#[allow(unused)]
impl Random for Circle {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Circle {
        Circle::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Typewriter {
    A(f64),
    B(String),
}

impl Random for Typewriter {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Typewriter {
        let v: usize = rng.gen_range(0, 2);
        match v {
            0 => Self::A({
                let v: f64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: String = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Diamond1 {
    A(i64),
    B(()),
    C(Typewriter),
}

impl Random for Diamond1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Diamond1 {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: () = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Typewriter = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Boss {
    A(i64),
    B(String),
    C(HashSet<bool>),
    D(Option<Diamond1>),
}

impl Random for Boss {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Boss {
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
                let v: HashSet<bool> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<Diamond1> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Window {
    icecream: HashMap<String, Boss>,
}

#[allow(unused)]
impl Random for Window {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Window {
        Window::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Potato1")]
pub struct Bird {
    flower1: f64,
}

#[allow(unused)]
impl Random for Bird {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bird {
        Bird::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Staircase {
    A(bool),
    B(HashMap<String, i64>),
    C(HashSet<String>),
    D(Bird),
}

impl Random for Staircase {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Staircase {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashMap<String, i64> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<String> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Bird = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case", default)]
pub struct Meteor {
    #[serde(rename = "diamond")]
    tunnel: HashSet<String>,
    #[serde(skip)]
    highway1: i64,
}

#[allow(unused)]
impl Random for Meteor {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Meteor {
        Meteor::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Girl {
    A(f64),
    B(Vec<u8>),
    C(Meteor),
}

impl Random for Girl {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Girl {
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
                let v: Meteor = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Bank1 {
    A(i64),
    B(Vec<u8>),
    C(()),
}

impl Random for Bank1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Bank1 {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: () = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Man {}

#[allow(unused)]
impl Random for Man {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Man {
        Man::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub struct Sword {}

#[allow(unused)]
impl Random for Sword {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Sword {
        Sword::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Church3 {
    A(bool),
    B(i64),
    C(Sword),
    D(Option<Vec<String>>),
}

impl Random for Church3 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Church3 {
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
                let v: Sword = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<Vec<String>> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub struct Feather1 {
    #[serde(skip)]
    tunnel1: bool,
    #[serde(rename = "signature")]
    planet2: Church3,
}

#[allow(unused)]
impl Random for Feather1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Feather1 {
        Feather1::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Pendulum {
    A(bool),
    B(i64),
    C(Vec<u8>),
    D(Option<Vec<bool>>),
}

impl Random for Pendulum {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pendulum {
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
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Option<Vec<bool>> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Mist {
    A(Vec<u8>),
    B(Vec<Vec<u8>>),
    C((i64, Vec<u8>)),
}

impl Random for Mist {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Mist {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<Vec<u8>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: (i64, Vec<u8>) = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase", rename = "Printer")]
pub struct Pillow {
    #[serde(skip_deserializing)]
    electricity: i64,
    #[serde(skip_deserializing)]
    aeroplane: Vec<u8>,
}

#[allow(unused)]
impl Random for Pillow {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pillow {
        Pillow::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Dung {
    A(bool),
    B(f64),
    C(HashSet<String>),
    D(Option<String>),
}

impl Random for Dung {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Dung {
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
                let v: Option<String> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase", rename = "Flower1", default)]
pub struct Airforce {
    #[serde(rename = "map1")]
    car1: HashSet<bool>,
}

#[allow(unused)]
impl Random for Airforce {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Airforce {
        Airforce::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(
    rename_all = "lowercase",
    rename = "Electricity1",
    deny_unknown_fields,
    default
)]
pub struct Library {
    rifle: bool,
}

#[allow(unused)]
impl Random for Library {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Library {
        Library::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Garden {
    A(i64),
    B((f64, (bool,))),
    C(Airforce),
    D(Library),
}

impl Random for Garden {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Garden {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: (f64, (bool,)) = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Airforce = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Library = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "lowercase", rename = "Aeroplane")]
pub struct Milk {
    #[serde(rename = "pillow")]
    vulture1: Garden,
    coffee: Vec<u8>,
}

#[allow(unused)]
impl Random for Milk {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Milk {
        Milk::new(rng.gen_ext(), rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct Horse1 {
    #[serde(flatten)]
    spectrum: Milk,
}

#[allow(unused)]
impl Random for Horse1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Horse1 {
        Horse1::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase", rename = "Computer")]
pub struct Button {
    floodlight: bool,
}

#[allow(unused)]
impl Random for Button {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Button {
        Button::new(rng.gen_ext())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Hat {
    A(bool),
    B(HashMap<String, Option<Option<String>>>),
    C(Horse1),
    D(Button),
}

impl Random for Hat {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Hat {
        let v: usize = rng.gen_range(0, 4);
        match v {
            0 => Self::A({
                let v: bool = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: HashMap<String, Option<Option<String>>> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Horse1 = rng.gen_ext();
                v
            }),
            3 => Self::D({
                let v: Button = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "UPPERCASE", rename = "Vacuum1")]
pub struct Pendulum1 {}

#[allow(unused)]
impl Random for Pendulum1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pendulum1 {
        Pendulum1::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(untagged)]
pub enum Pants2 {
    A(bool),
    B(f64),
    C(Vec<u8>),
    D(HashSet<i64>),
}

impl Random for Pants2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Pants2 {
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
                let v: Vec<u8> = rng.gen_ext();
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
pub enum Clock {
    A(i64),
    B(Vec<u8>),
    C(Pants2),
}

impl Random for Clock {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Clock {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: Pants2 = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq)]
#[serde(untagged)]
pub enum SpotLight1 {
    A(i64),
    B(Vec<u8>),
    C(HashSet<bool>),
}

impl Random for SpotLight1 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> SpotLight1 {
        let v: usize = rng.gen_range(0, 3);
        match v {
            0 => Self::A({
                let v: i64 = rng.gen_ext();
                v
            }),
            1 => Self::B({
                let v: Vec<u8> = rng.gen_ext();
                v
            }),
            2 => Self::C({
                let v: HashSet<bool> = rng.gen_ext();
                v
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, new, PartialEq, Eq, Hash, Default)]
pub struct Typewriter2 {}

#[allow(unused)]
impl Random for Typewriter2 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Typewriter2 {
        Typewriter2::new()
    }
}

macro_rules! gen {
    ($encoder:path) => {{
        let mut rng = rand::thread_rng();
        let mut ret = Vec::<Vec<u8>>::new();

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Shower = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Rifle, i64) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Leg = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Banana = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vulture = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Table = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, HashSet<String>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Vec<bool>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Baby3> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Square = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<HashMap<String, bool>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Explosive> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Bridge1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Eraser = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Vec<bool>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: IceCream = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Signature1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Sandwich> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Circle = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<i64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Window = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Staircase = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<Vec<u8>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<i64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Option<()>, f64) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Girl = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Bank1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Man = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Feather1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: () = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: () = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Pendulum = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Mist> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Vec<u8>, HashMap<String, bool>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<u8> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Option<Option<Vec<u8>>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Pillow = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Dung = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Hat> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, f64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<i64> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Pendulum1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: i64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: bool = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashMap<String, Clock> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: SpotLight1 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: HashSet<bool> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<String> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: Vec<Vec<String>> = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: String = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: (Vec<Vec<String>>, Vec<Vec<((), Typewriter2)>>) = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        let v: f64 = rng.gen_ext();
        ret.push($encoder(&v).unwrap());

        ret
    }};
}
