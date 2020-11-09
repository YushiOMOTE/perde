
mod gen;

use derive_new::new;
use serde::{Serialize, Deserialize};
use rand::Rng;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use crate::gen::{Random, GenExt};

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct Arm {
  bowl: Vec<u8>,
}

impl Random for Arm {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Arm {
    Arm::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Freeway {
  A(bool),
  B(f64),
  C(Vec<u8>),
  D(Arm),
}

impl Random for Freeway {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Freeway {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: f64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<u8> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Arm = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Roof")]
pub struct Record {
}

impl Random for Record {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Record {
    Record::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "UPPERCASE", rename = "Worm")]
pub struct Sandwich {
  #[serde(rename = "bed")]
  navy: HashMap<String, Record>,
  #[serde(rename = "junk")]
  rock: bool,
}

impl Random for Sandwich {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Sandwich {
    Sandwich::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "kebab-case")]
pub struct Compass {
}

impl Random for Compass {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Compass {
    Compass::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Coffee")]
pub struct Pendulum {
  #[serde(flatten)]
  train: HashMap<String, Vec<u8>>,
}

impl Random for Pendulum {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Pendulum {
    Pendulum::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "PascalCase", rename = "Ship", deny_unknown_fields)]
pub struct Pebble {
}

impl Random for Pebble {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Pebble {
    Pebble::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "camelCase")]
pub struct Telescope {
  #[serde(flatten)]
  elephant: Pebble,
  #[serde(skip)]
  woman: String,
}

impl Random for Telescope {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Telescope {
    Telescope::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct Rainbow {
}

impl Random for Rainbow {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Rainbow {
    Rainbow::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct Sword {
  adult: Vec<u8>,
  #[serde(rename = "ears")]
  pocket: (),
}

impl Random for Sword {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Sword {
    Sword::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Satellite {
  A(f64),
  B(String),
  C(Vec<u8>),
}

impl Random for Satellite {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Satellite {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: f64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: String = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<u8> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "camelCase", rename = "Bank", deny_unknown_fields)]
pub struct Passport {
  #[serde(rename = "vampire")]
  printer: Option<String>,
  icecream: (Satellite, Vec<HashSet<Vec<u8>>>, ),
}

impl Random for Passport {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Passport {
    Passport::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Web")]
pub struct Torch {
  #[serde(rename = "grapes")]
  nail: f64,
}

impl Random for Torch {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Torch {
    Torch::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct DataBase {
  grapes1: String,
  #[serde(skip)]
  bee: String,
}

impl Random for DataBase {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> DataBase {
    DataBase::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Baby {
  A(f64),
  B(Vec<f64>),
  C(DataBase),
}

impl Random for Baby {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Baby {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: f64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<f64> = rng.gen_ext(); v }),
      2 => Self::C({ let v: DataBase = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Clock {
  A(Vec<u8>),
  B(Torch),
  C(Baby),
}

impl Random for Clock {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Clock {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: Vec<u8> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Torch = rng.gen_ext(); v }),
      2 => Self::C({ let v: Baby = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Car {
  A(Vec<u8>),
  B(Passport),
  C(Option<String>),
  D(Clock),
}

impl Random for Car {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Car {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: Vec<u8> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Passport = rng.gen_ext(); v }),
      2 => Self::C({ let v: Option<String> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Clock = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct Hose {
  #[serde(rename = "magnet")]
  fruit: Car,
  #[serde(skip)]
  bowl1: Vec<HashMap<String, ()>>,
}

impl Random for Hose {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Hose {
    Hose::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct Telescope1 {
  boss: bool,
  umbrella: Hose,
}

impl Random for Telescope1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Telescope1 {
    Telescope1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Carrier {
  A(i64),
  B(String),
  C(Vec<u8>),
  D(HashSet<String>),
}

impl Random for Carrier {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Carrier {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: String = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<u8> = rng.gen_ext(); v }),
      3 => Self::D({ let v: HashSet<String> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "UPPERCASE", rename = "Money", deny_unknown_fields)]
pub struct PostOffice {
  #[serde(rename = "festival")]
  circus: HashSet<i64>,
  toilet: Carrier,
}

impl Random for PostOffice {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> PostOffice {
    PostOffice::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "camelCase", rename = "Parachute")]
pub struct Solid {
  icecream1: bool,
  #[serde(flatten)]
  ship: HashMap<String, HashMap<String, PostOffice>>,
}

impl Random for Solid {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Solid {
    Solid::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct CompactDisc {
}

impl Random for CompactDisc {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> CompactDisc {
    CompactDisc::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Kaleidoscope", deny_unknown_fields)]
pub struct Potato1 {
  #[serde(skip)]
  child: i64,
}

impl Random for Potato1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Potato1 {
    Potato1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct SpaceShuttle {
  #[serde(rename = "xray", skip_deserializing)]
  salt: Potato1,
  #[serde(rename = "web")]
  bed1: Vec<Vec<(bool, )>>,
}

impl Random for SpaceShuttle {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> SpaceShuttle {
    SpaceShuttle::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Army {
  A(Vec<u8>),
  B(HashMap<String, i64>),
  C(()),
  D(SpaceShuttle),
}

impl Random for Army {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Army {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: Vec<u8> = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashMap<String, i64> = rng.gen_ext(); v }),
      2 => Self::C({ let v: () = rng.gen_ext(); v }),
      3 => Self::D({ let v: SpaceShuttle = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "PascalCase")]
pub struct Bee {
}

impl Random for Bee {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bee {
    Bee::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Pillow {
  A(f64),
  B(Vec<u8>),
  C(Option<String>),
}

impl Random for Pillow {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Pillow {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: f64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Option<String> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Gemstone {
  A(Vec<f64>),
  B(HashSet<i64>),
  C(Option<Bee>),
  D(Pillow),
}

impl Random for Gemstone {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Gemstone {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: Vec<f64> = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<i64> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Option<Bee> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Pillow = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Elephant {
  A(bool),
  B(String),
  C(HashMap<String, i64>),
  D(Gemstone),
}

impl Random for Elephant {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Elephant {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: String = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashMap<String, i64> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Gemstone = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct Carrier1 {
  #[serde(skip_deserializing)]
  bathroom: String,
  #[serde(rename = "surveyor")]
  computer: (f64, Elephant, ),
}

impl Random for Carrier1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Carrier1 {
    Carrier1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct SpotLight {
  #[serde(flatten)]
  coffeeshop: HashMap<String, bool>,
}

impl Random for SpotLight {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> SpotLight {
    SpotLight::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "PascalCase")]
pub struct Chocolates {
  fruit1: f64,
}

impl Random for Chocolates {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Chocolates {
    Chocolates::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Salt {
  A(i64),
  B(f64),
  C(Vec<u8>),
  D(HashSet<Vec<u8>>),
}

impl Random for Salt {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Salt {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: f64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<u8> = rng.gen_ext(); v }),
      3 => Self::D({ let v: HashSet<Vec<u8>> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Rifle", deny_unknown_fields)]
pub struct Mist {
  square: i64,
}

impl Random for Mist {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Mist {
    Mist::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Car1 {
  A(HashMap<String, Vec<String>>),
  B(Vec<bool>),
  C(HashSet<Salt>),
  D(Mist),
}

impl Random for Car1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Car1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: HashMap<String, Vec<String>> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<bool> = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashSet<Salt> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Mist = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Pants {
  A(i64),
  B(f64),
  C(String),
}

impl Random for Pants {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Pants {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: f64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: String = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Tunnel", deny_unknown_fields)]
pub struct Desk {
  sword: bool,
}

impl Random for Desk {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Desk {
    Desk::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Torpedo")]
pub struct Family {
  #[serde(rename = "leatherjacket")]
  aeroplane: (f64, ),
}

impl Random for Family {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Family {
    Family::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Spice1 {
  A(i64),
  B(Vec<Desk>),
  C(Family),
}

impl Random for Spice1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Spice1 {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<Desk> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Family = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Liquid {
  #[serde(skip_deserializing)]
  kitchen: Vec<Option<Pants>>,
  #[serde(rename = "shop")]
  aircraft: HashMap<String, Spice1>,
}

impl Random for Liquid {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Liquid {
    Liquid::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case")]
pub struct Pebble1 {
  #[serde(flatten)]
  spectrum: Liquid,
  rock1: HashSet<bool>,
}

impl Random for Pebble1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Pebble1 {
    Pebble1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Mouth")]
pub struct Junk {
  #[serde(rename = "bowl2")]
  jetfighter: (Vec<u8>, String, ),
  #[serde(rename = "sandpaper")]
  tennisracquet: i64,
}

impl Random for Junk {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Junk {
    Junk::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Spectrum {
  A(bool),
  B(HashSet<String>),
  C(Junk),
}

impl Random for Spectrum {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Spectrum {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<String> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Junk = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Meteor")]
pub struct Train {
  explosive: Vec<Vec<f64>>,
}

impl Random for Train {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Train {
    Train::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Bed {
  A(i64),
  B(HashSet<i64>),
  C(Train),
  D(Option<f64>),
}

impl Random for Bed {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bed {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<i64> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Train = rng.gen_ext(); v }),
      3 => Self::D({ let v: Option<f64> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename = "Garden")]
pub struct Desk1 {
  #[serde(rename = "compactdisc", skip_deserializing)]
  elephant1: Bed,
}

impl Random for Desk1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Desk1 {
    Desk1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Telescope2 {
}

impl Random for Telescope2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Telescope2 {
    Telescope2::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct Garden1 {
}

impl Random for Garden1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Garden1 {
    Garden1::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Backpack")]
pub struct Telescope3 {
  #[serde(skip)]
  desk: f64,
  #[serde(rename = "fork", skip_deserializing)]
  tongue: Vec<Vec<u8>>,
}

impl Random for Telescope3 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Telescope3 {
    Telescope3::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Family2 {
  A(i64),
  B((f64, )),
  C(Garden1),
  D(Option<Telescope3>),
}

impl Random for Family2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Family2 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: (f64, ) = rng.gen_ext(); v }),
      2 => Self::C({ let v: Garden1 = rng.gen_ext(); v }),
      3 => Self::D({ let v: Option<Telescope3> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Cup {
  A(bool),
  B(f64),
}

impl Random for Cup {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Cup {
    let v: usize = rng.gen_range(0, 2);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: f64 = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Maze {
  A(HashMap<String, Vec<f64>>),
  B(Vec<Family2>),
  C(Cup),
}

impl Random for Maze {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Maze {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: HashMap<String, Vec<f64>> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<Family2> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Cup = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct Passport2 {
  #[serde(rename = "church")]
  satellite1: String,
}

impl Random for Passport2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Passport2 {
    Passport2::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename = "Man", deny_unknown_fields)]
pub struct Horse {
  #[serde(skip)]
  barbecue: f64,
  #[serde(skip)]
  sunglasses: Passport2,
}

impl Random for Horse {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Horse {
    Horse::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Clown {
  A(i64),
  B(String),
  C((HashSet<i64>, Vec<(Horse, )>, )),
}

impl Random for Clown {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Clown {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: String = rng.gen_ext(); v }),
      2 => Self::C({ let v: (HashSet<i64>, Vec<(Horse, )>, ) = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "kebab-case")]
pub struct Kaleidoscope1 {
  #[serde(rename = "toilet1")]
  explosive1: (Vec<u8>, ),
}

impl Random for Kaleidoscope1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Kaleidoscope1 {
    Kaleidoscope1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Surveyor {
  A(bool),
  B(i64),
  C(f64),
  D(HashSet<Vec<u8>>),
}

impl Random for Surveyor {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Surveyor {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: i64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: f64 = rng.gen_ext(); v }),
      3 => Self::D({ let v: HashSet<Vec<u8>> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Aircraft1 {
  #[serde(skip)]
  paintbrush: f64,
}

impl Random for Aircraft1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Aircraft1 {
    Aircraft1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case")]
pub struct Carrier2 {
  #[serde(rename = "bathtub")]
  gate: Vec<u8>,
}

impl Random for Carrier2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Carrier2 {
    Carrier2::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Perfume {
  A(HashMap<String, bool>),
  B(Vec<Option<f64>>),
  C(Carrier2),
}

impl Random for Perfume {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Perfume {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: HashMap<String, bool> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<Option<f64>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Carrier2 = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Wheelchair")]
pub struct Nail {
  freeway: bool,
  compactdisc1: HashSet<bool>,
}

impl Random for Nail {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Nail {
    Nail::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Pillow1 {
  A(Vec<u8>),
  B(HashMap<String, Vec<u8>>),
  C(Vec<Vec<u8>>),
  D((bool, )),
}

impl Random for Pillow1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Pillow1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: Vec<u8> = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashMap<String, Vec<u8>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<Vec<u8>> = rng.gen_ext(); v }),
      3 => Self::D({ let v: (bool, ) = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "camelCase", rename = "PostOffice1", deny_unknown_fields)]
pub struct Album {
  #[serde(rename = "maze")]
  umbrella1: Pillow1,
  #[serde(rename = "tapestry", skip_deserializing)]
  pepper: HashSet<String>,
}

impl Random for Album {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Album {
    Album::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Game {
  cycle: HashSet<bool>,
}

impl Random for Game {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Game {
    Game::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct Onion {
  school: bool,
}

impl Random for Onion {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Onion {
    Onion::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CompactDisc1 {
  A(Vec<u8>),
  B(Game),
  C(Onion),
}

impl Random for CompactDisc1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> CompactDisc1 {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: Vec<u8> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Game = rng.gen_ext(); v }),
      2 => Self::C({ let v: Onion = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct LeatherJacket1 {
  icecream2: bool,
}

impl Random for LeatherJacket1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> LeatherJacket1 {
    LeatherJacket1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct Bible1 {
  #[serde(rename = "car")]
  maze1: LeatherJacket1,
}

impl Random for Bible1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bible1 {
    Bible1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "kebab-case", rename = "Tunnel1", deny_unknown_fields)]
pub struct Planet {
  #[serde(skip)]
  solid: Bible1,
  fungus: f64,
}

impl Random for Planet {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Planet {
    Planet::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Dress {
  A(bool),
  B(i64),
  C(HashMap<String, HashMap<String, Option<bool>>>),
  D(Option<Planet>),
}

impl Random for Dress {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Dress {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: i64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashMap<String, HashMap<String, Option<bool>>> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Option<Planet> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Circle {
  A(i64),
  B(Vec<bool>),
  C(HashSet<CompactDisc1>),
  D(Option<Dress>),
}

impl Random for Circle {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Circle {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<bool> = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashSet<CompactDisc1> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Option<Dress> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Worm2", deny_unknown_fields)]
pub struct Family3 {
  fan: Album,
  #[serde(flatten)]
  sportscar: HashMap<String, Vec<Circle>>,
}

impl Random for Family3 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Family3 {
    Family3::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Sex {
  A(Vec<HashMap<String, HashMap<String, i64>>>),
  B(HashSet<Vec<u8>>),
  C(()),
  D(Family3),
}

impl Random for Sex {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Sex {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: Vec<HashMap<String, HashMap<String, i64>>> = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<Vec<u8>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: () = rng.gen_ext(); v }),
      3 => Self::D({ let v: Family3 = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Liquid1 {
  A(HashMap<String, Vec<Vec<Vec<u8>>>>),
  B(Nail),
  C(Option<()>),
  D(Sex),
}

impl Random for Liquid1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Liquid1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: HashMap<String, Vec<Vec<Vec<u8>>>> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Nail = rng.gen_ext(); v }),
      2 => Self::C({ let v: Option<()> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Sex = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "lowercase", rename = "Baby1", deny_unknown_fields)]
pub struct Insect {
  grapes2: Perfume,
  #[serde(rename = "water")]
  xray1: Liquid1,
}

impl Random for Insect {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Insect {
    Insect::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Triangle", deny_unknown_fields)]
pub struct Table {
  arm: Vec<HashMap<String, HashMap<String, String>>>,
  #[serde(rename = "brain1")]
  fan1: String,
}

impl Random for Table {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Table {
    Table::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "kebab-case", rename = "Balloon", deny_unknown_fields)]
pub struct Ring {
  #[serde(rename = "baby")]
  arm1: Vec<u8>,
}

impl Random for Ring {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Ring {
    Ring::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "CoffeeShop", deny_unknown_fields)]
pub struct Drum {
  #[serde(flatten)]
  circus1: HashMap<String, i64>,
}

impl Random for Drum {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Drum {
    Drum::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Eraser {
  A((Ring, HashSet<i64>, )),
  B(Drum),
}

impl Random for Eraser {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Eraser {
    let v: usize = rng.gen_range(0, 2);
    match v {
      0 => Self::A({ let v: (Ring, HashSet<i64>, ) = rng.gen_ext(); v }),
      1 => Self::B({ let v: Drum = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "lowercase", rename = "Pillow2")]
pub struct Triangle1 {
}

impl Random for Triangle1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Triangle1 {
    Triangle1::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Car3 {
  water1: Vec<u8>,
  #[serde(skip)]
  bottle: (bool, ),
}

impl Random for Car3 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Car3 {
    Car3::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct Telescope4 {
}

impl Random for Telescope4 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Telescope4 {
    Telescope4::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Snail1 {
  A(f64),
  B(Vec<u8>),
  C(HashMap<String, Car3>),
  D(Option<Telescope4>),
}

impl Random for Snail1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Snail1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: f64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashMap<String, Car3> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Option<Telescope4> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum God {
  A(Vec<String>),
  B(HashSet<bool>),
  C(Snail1),
}

impl Random for God {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> God {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: Vec<String> = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<bool> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Snail1 = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Rocket {
  A(i64),
  B(HashSet<Vec<u8>>),
  C(Triangle1),
  D(God),
}

impl Random for Rocket {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Rocket {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<Vec<u8>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Triangle1 = rng.gen_ext(); v }),
      3 => Self::D({ let v: God = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case")]
pub struct Electricity3 {
  #[serde(rename = "circus2")]
  map: HashMap<String, f64>,
}

impl Random for Electricity3 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Electricity3 {
    Electricity3::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Treadmill", deny_unknown_fields)]
pub struct Signature {
}

impl Random for Signature {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Signature {
    Signature::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Ears {
  A(String),
  B(Vec<u8>),
  C(HashSet<i64>),
  D(Signature),
}

impl Random for Ears {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Ears {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: String = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashSet<i64> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Signature = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Typewriter {
  #[serde(rename = "umbrella2")]
  diamond1: i64,
  #[serde(skip)]
  leatherjacket1: String,
}

impl Random for Typewriter {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Typewriter {
    Typewriter::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct Microscope {
  #[serde(flatten)]
  carrot: Typewriter,
  #[serde(rename = "salt1")]
  car1: f64,
}

impl Random for Microscope {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Microscope {
    Microscope::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PostOffice2 {
  A(bool),
  B(i64),
  C(Vec<u8>),
}

impl Random for PostOffice2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> PostOffice2 {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: i64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<u8> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "kebab-case", rename = "Plane", deny_unknown_fields)]
pub struct Airport {
  #[serde(rename = "desk1")]
  ring: Vec<Vec<HashSet<PostOffice2>>>,
  #[serde(skip_deserializing)]
  electricity: bool,
}

impl Random for Airport {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Airport {
    Airport::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "snake_case", rename = "Junk1", deny_unknown_fields)]
pub struct School {
}

impl Random for School {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> School {
    School::new(
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Triangle2 {
  A(bool),
  B(f64),
  C(Vec<Vec<u8>>),
  D(HashSet<Vec<u8>>),
}

impl Random for Triangle2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Triangle2 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: f64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<Vec<u8>> = rng.gen_ext(); v }),
      3 => Self::D({ let v: HashSet<Vec<u8>> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "kebab-case", rename = "Bowl", deny_unknown_fields)]
pub struct Air1 {
  #[serde(skip)]
  shower: f64,
}

impl Random for Air1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Air1 {
    Air1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "lowercase", rename = "PostOffice3")]
pub struct Air {
  #[serde(skip)]
  dress: Air1,
}

impl Random for Air {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Air {
    Air::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", deny_unknown_fields)]
pub struct Ears1 {
  #[serde(rename = "insect")]
  airport1: bool,
  #[serde(rename = "pyramid")]
  navy1: String,
}

impl Random for Ears1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Ears1 {
    Ears1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename = "Solid1", deny_unknown_fields)]
pub struct Vampire {
  mouth: bool,
  #[serde(flatten)]
  coffeeshop1: Ears1,
}

impl Random for Vampire {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Vampire {
    Vampire::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Train2 {
  A(i64),
  B(HashSet<String>),
  C(Air),
  D(Option<Option<Vampire>>),
}

impl Random for Train2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Train2 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<String> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Air = rng.gen_ext(); v }),
      3 => Self::D({ let v: Option<Option<Vampire>> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Eraser1 {
  A(f64),
  B(Vec<u8>),
  C(HashMap<String, Train2>),
  D(Vec<bool>),
}

impl Random for Eraser1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Eraser1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: f64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashMap<String, Train2> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Vec<bool> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Banana")]
pub struct Bible2 {
  #[serde(rename = "torpedo")]
  tongue2: i64,
}

impl Random for Bible2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bible2 {
    Bible2::new(
      rng.gen_ext(),
    )
  }
}



fn main() {
let mut rng = rand::thread_rng();

  let v: Option<(bool, )> = rng.gen_ext();
  let f = File::create("json/Option__bool___.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: i64 = rng.gen_ext();
  let f = File::create("json/i64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<u8> = rng.gen_ext();
  let f = File::create("json/Vec_u8_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Option<String> = rng.gen_ext();
  let f = File::create("json/Option_String_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashMap<String, Freeway> = rng.gen_ext();
  let f = File::create("json/HashMap_String_Freeway_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<u8> = rng.gen_ext();
  let f = File::create("json/Vec_u8_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: () = rng.gen_ext();
  let f = File::create("json/__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Option<HashMap<String, Sandwich>> = rng.gen_ext();
  let f = File::create("json/Option_HashMap_String_Sandwich__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashMap<String, Option<Compass>> = rng.gen_ext();
  let f = File::create("json/HashMap_String_Option_Compass__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashSet<f64> = rng.gen_ext();
  let f = File::create("json/HashSet_f64_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Pendulum = rng.gen_ext();
  let f = File::create("json/Pendulum.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (Telescope, ) = rng.gen_ext();
  let f = File::create("json/_Telescope__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<u8> = rng.gen_ext();
  let f = File::create("json/Vec_u8_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Option<Vec<u8>> = rng.gen_ext();
  let f = File::create("json/Option_Vec_u8__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: bool = rng.gen_ext();
  let f = File::create("json/bool.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: bool = rng.gen_ext();
  let f = File::create("json/bool.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<u8> = rng.gen_ext();
  let f = File::create("json/Vec_u8_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Rainbow = rng.gen_ext();
  let f = File::create("json/Rainbow.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashSet<String> = rng.gen_ext();
  let f = File::create("json/HashSet_String_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (Sword, ) = rng.gen_ext();
  let f = File::create("json/_Sword__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (Vec<u8>, HashSet<i64>, ) = rng.gen_ext();
  let f = File::create("json/_Vec_u8__HashSet_i64___.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: bool = rng.gen_ext();
  let f = File::create("json/bool.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Option<HashMap<String, f64>> = rng.gen_ext();
  let f = File::create("json/Option_HashMap_String_f64__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Telescope1 = rng.gen_ext();
  let f = File::create("json/Telescope1.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: f64 = rng.gen_ext();
  let f = File::create("json/f64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Solid = rng.gen_ext();
  let f = File::create("json/Solid.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: i64 = rng.gen_ext();
  let f = File::create("json/i64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashSet<f64> = rng.gen_ext();
  let f = File::create("json/HashSet_f64_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<u8> = rng.gen_ext();
  let f = File::create("json/Vec_u8_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: CompactDisc = rng.gen_ext();
  let f = File::create("json/CompactDisc.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Option<Vec<String>> = rng.gen_ext();
  let f = File::create("json/Option_Vec_String__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<u8> = rng.gen_ext();
  let f = File::create("json/Vec_u8_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Army = rng.gen_ext();
  let f = File::create("json/Army.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Carrier1 = rng.gen_ext();
  let f = File::create("json/Carrier1.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: SpotLight = rng.gen_ext();
  let f = File::create("json/SpotLight.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Chocolates = rng.gen_ext();
  let f = File::create("json/Chocolates.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: i64 = rng.gen_ext();
  let f = File::create("json/i64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<HashSet<Car1>> = rng.gen_ext();
  let f = File::create("json/Vec_HashSet_Car1__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: f64 = rng.gen_ext();
  let f = File::create("json/f64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Pebble1 = rng.gen_ext();
  let f = File::create("json/Pebble1.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: f64 = rng.gen_ext();
  let f = File::create("json/f64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Spectrum = rng.gen_ext();
  let f = File::create("json/Spectrum.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: f64 = rng.gen_ext();
  let f = File::create("json/f64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<f64> = rng.gen_ext();
  let f = File::create("json/Vec_f64_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: i64 = rng.gen_ext();
  let f = File::create("json/i64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (i64, ) = rng.gen_ext();
  let f = File::create("json/_i64__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<u8> = rng.gen_ext();
  let f = File::create("json/Vec_u8_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (Desk1, ) = rng.gen_ext();
  let f = File::create("json/_Desk1__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Telescope2 = rng.gen_ext();
  let f = File::create("json/Telescope2.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: f64 = rng.gen_ext();
  let f = File::create("json/f64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashMap<String, Maze> = rng.gen_ext();
  let f = File::create("json/HashMap_String_Maze_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<bool> = rng.gen_ext();
  let f = File::create("json/Vec_bool_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Clown = rng.gen_ext();
  let f = File::create("json/Clown.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: f64 = rng.gen_ext();
  let f = File::create("json/f64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashMap<String, HashSet<bool>> = rng.gen_ext();
  let f = File::create("json/HashMap_String_HashSet_bool__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (i64, ) = rng.gen_ext();
  let f = File::create("json/_i64__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: () = rng.gen_ext();
  let f = File::create("json/__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: () = rng.gen_ext();
  let f = File::create("json/__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashMap<String, Kaleidoscope1> = rng.gen_ext();
  let f = File::create("json/HashMap_String_Kaleidoscope1_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Surveyor = rng.gen_ext();
  let f = File::create("json/Surveyor.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: f64 = rng.gen_ext();
  let f = File::create("json/f64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Option<bool> = rng.gen_ext();
  let f = File::create("json/Option_bool_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<Aircraft1> = rng.gen_ext();
  let f = File::create("json/Vec_Aircraft1_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: i64 = rng.gen_ext();
  let f = File::create("json/i64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: i64 = rng.gen_ext();
  let f = File::create("json/i64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Insect = rng.gen_ext();
  let f = File::create("json/Insect.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<Vec<HashMap<String, ()>>> = rng.gen_ext();
  let f = File::create("json/Vec_Vec_HashMap_String______.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: () = rng.gen_ext();
  let f = File::create("json/__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (String, ) = rng.gen_ext();
  let f = File::create("json/_String__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<HashMap<String, HashSet<bool>>> = rng.gen_ext();
  let f = File::create("json/Vec_HashMap_String_HashSet_bool___.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<u8> = rng.gen_ext();
  let f = File::create("json/Vec_u8_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashMap<String, Table> = rng.gen_ext();
  let f = File::create("json/HashMap_String_Table_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Eraser = rng.gen_ext();
  let f = File::create("json/Eraser.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<HashSet<String>> = rng.gen_ext();
  let f = File::create("json/Vec_HashSet_String__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<HashMap<String, String>> = rng.gen_ext();
  let f = File::create("json/Vec_HashMap_String_String__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: HashMap<String, Rocket> = rng.gen_ext();
  let f = File::create("json/HashMap_String_Rocket_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Electricity3 = rng.gen_ext();
  let f = File::create("json/Electricity3.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: () = rng.gen_ext();
  let f = File::create("json/__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: f64 = rng.gen_ext();
  let f = File::create("json/f64.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Ears = rng.gen_ext();
  let f = File::create("json/Ears.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Microscope = rng.gen_ext();
  let f = File::create("json/Microscope.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Airport = rng.gen_ext();
  let f = File::create("json/Airport.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (f64, HashMap<String, String>, ) = rng.gen_ext();
  let f = File::create("json/_f64_HashMap_String_String___.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: bool = rng.gen_ext();
  let f = File::create("json/bool.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Option<Vec<u8>> = rng.gen_ext();
  let f = File::create("json/Option_Vec_u8__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<School> = rng.gen_ext();
  let f = File::create("json/Vec_School_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Vec<Triangle2> = rng.gen_ext();
  let f = File::create("json/Vec_Triangle2_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: Option<Eraser1> = rng.gen_ext();
  let f = File::create("json/Option_Eraser1_.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: bool = rng.gen_ext();
  let f = File::create("json/bool.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: (Bible2, ) = rng.gen_ext();
  let f = File::create("json/_Bible2__.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

  let v: String = rng.gen_ext();
  let f = File::create("json/String.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();

}

