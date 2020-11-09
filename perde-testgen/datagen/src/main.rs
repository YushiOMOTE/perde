
// Generated 2020-11-10T05:20:47.119165300+09:00
mod gen;

use derive_new::new;
use serde::{Serialize, Deserialize};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use crate::gen::{Random, GenExt};

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "PascalCase", rename = "Foot")]
pub struct Bed {
}

#[allow(unused)]impl Random for Bed {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bed {
    Bed::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "lowercase")]
pub struct Clock {
  #[serde(skip_deserializing)]
  spaceshuttle: i64,
}

#[allow(unused)]impl Random for Clock {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Clock {
    Clock::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum Torch {
  A(f64),
  B(HashSet<Vec<u8>>),
  C(Bed),
  D(Clock),
}

impl Random for Torch {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Torch {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: f64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<Vec<u8>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Bed = rng.gen_ext(); v }),
      3 => Self::D({ let v: Clock = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Festival")]
pub struct Vacuum {
  #[serde(skip)]
  library: String,
  bird: Vec<u8>,
}

#[allow(unused)]impl Random for Vacuum {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Vacuum {
    Vacuum::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CoffeeShop {
}

#[allow(unused)]impl Random for CoffeeShop {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> CoffeeShop {
    CoffeeShop::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Default)]
#[serde(rename_all = "PascalCase", rename = "Book", default)]
pub struct Elephant {
  #[serde(skip_deserializing)]
  snail: Vec<u8>,
  #[serde(skip_deserializing)]
  rock: HashMap<String, i64>,
}

#[allow(unused)]impl Random for Elephant {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Elephant {
    Elephant::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "snake_case", default)]
pub struct PostOffice {
}

#[allow(unused)]impl Random for PostOffice {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> PostOffice {
    PostOffice::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Potato", default)]
pub struct Spiral {
}

#[allow(unused)]impl Random for Spiral {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Spiral {
    Spiral::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "snake_case")]
pub struct SpaceShuttle {
}

#[allow(unused)]impl Random for SpaceShuttle {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> SpaceShuttle {
    SpaceShuttle::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash)]
#[serde(untagged)]
pub enum Spice {
  A(bool),
  B(Vec<u8>),
  C(SpaceShuttle),
}

impl Random for Spice {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Spice {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      2 => Self::C({ let v: SpaceShuttle = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash)]
#[serde(rename = "SpotLight", deny_unknown_fields)]
pub struct Videotape {
  navy: Spice,
}

#[allow(unused)]impl Random for Videotape {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Videotape {
    Videotape::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(untagged)]
pub enum Church {
  A(Vec<u8>),
  B(HashMap<String, Vec<Spiral>>),
  C(()),
  D(Videotape),
}

impl Random for Church {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Church {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: Vec<u8> = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashMap<String, Vec<Spiral>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: () = rng.gen_ext(); v }),
      3 => Self::D({ let v: Videotape = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(untagged)]
pub enum Necklace {
  A(Vec<Vec<u8>>),
  B(Elephant),
  C(Option<Vec<PostOffice>>),
  D(Church),
}

impl Random for Necklace {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Necklace {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: Vec<Vec<u8>> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Elephant = rng.gen_ext(); v }),
      2 => Self::C({ let v: Option<Vec<PostOffice>> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Church = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash)]
#[serde(untagged)]
pub enum Drink {
  A(String),
  B(Vec<u8>),
}

impl Random for Drink {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Drink {
    let v: usize = rng.gen_range(0, 2);
    match v {
      0 => Self::A({ let v: String = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "PascalCase", rename = "Table", default)]
pub struct Car {
  #[serde(skip)]
  map: Vec<u8>,
  cappuccino: String,
}

#[allow(unused)]impl Random for Car {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Car {
    Car::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(untagged)]
pub enum DataBase {
  A(String),
  B(HashSet<Drink>),
  C(Car),
}

impl Random for DataBase {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> DataBase {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: String = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashSet<Drink> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Car = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(rename_all = "lowercase", rename = "Passport", deny_unknown_fields)]
pub struct Spoon {
  teeth: DataBase,
  #[serde(skip)]
  car: Option<bool>,
}

#[allow(unused)]impl Random for Spoon {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Spoon {
    Spoon::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "snake_case", rename = "SportsCar", default)]
pub struct Hieroglyph {
}

#[allow(unused)]impl Random for Hieroglyph {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Hieroglyph {
    Hieroglyph::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum Water {
  A(f64),
  B(Spoon),
  C(Hieroglyph),
}

impl Random for Water {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Water {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: f64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Spoon = rng.gen_ext(); v }),
      2 => Self::C({ let v: Hieroglyph = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "PascalCase")]
pub struct Aircraft {
  family: Vec<u8>,
}

#[allow(unused)]impl Random for Aircraft {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Aircraft {
    Aircraft::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(untagged)]
pub enum Horse {
  A(bool),
  B(HashMap<String, HashMap<String, Vec<u8>>>),
  C((HashSet<String>, )),
}

impl Random for Horse {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Horse {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashMap<String, HashMap<String, Vec<u8>>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: (HashSet<String>, ) = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Default)]
#[serde(rename_all = "snake_case", rename = "Star", default)]
pub struct Necklace1 {
  #[serde(skip_deserializing)]
  kitchen: Vec<u8>,
  milk: HashMap<String, Option<Horse>>,
}

#[allow(unused)]impl Random for Necklace1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Necklace1 {
    Necklace1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Baby")]
pub struct Girl {
  carrier: HashMap<String, HashSet<String>>,
}

#[allow(unused)]impl Random for Girl {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Girl {
    Girl::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum Grapes {
  A(f64),
  B((Option<Vec<u8>>, Vec<u8>, )),
  C(Girl),
}

impl Random for Grapes {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Grapes {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: f64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: (Option<Vec<u8>>, Vec<u8>, ) = rng.gen_ext(); v }),
      2 => Self::C({ let v: Girl = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(rename_all = "camelCase", rename = "Shoes", deny_unknown_fields)]
pub struct Bed1 {
  #[serde(rename = "gas")]
  roof: Grapes,
}

#[allow(unused)]impl Random for Bed1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bed1 {
    Bed1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(rename_all = "camelCase", rename = "Telescope", deny_unknown_fields)]
pub struct Explosive {
  #[serde(rename = "church")]
  robot: Bed1,
}

#[allow(unused)]impl Random for Explosive {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Explosive {
    Explosive::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "UPPERCASE", rename = "Child", default)]
pub struct Meat {
}

#[allow(unused)]impl Random for Meat {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Meat {
    Meat::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Bowl")]
pub struct Drum {
  #[serde(skip_deserializing)]
  church1: (Option<Meat>, f64, ),
}

#[allow(unused)]impl Random for Drum {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Drum {
    Drum::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "snake_case", default)]
pub struct Film {
}

#[allow(unused)]impl Random for Film {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Film {
    Film::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(untagged)]
pub enum Staircase {
  A(bool),
  B(Vec<u8>),
  C((bool, (String, HashSet<i64>, ), )),
  D(Film),
}

impl Random for Staircase {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Staircase {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      2 => Self::C({ let v: (bool, (String, HashSet<i64>, ), ) = rng.gen_ext(); v }),
      3 => Self::D({ let v: Film = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Album {
  #[serde(skip)]
  rifle: f64,
  coffee: Staircase,
}

#[allow(unused)]impl Random for Album {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Album {
    Album::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "PascalCase", rename = "Printer", default)]
pub struct Cave {
  #[serde(rename = "tunnel")]
  gloves: HashMap<String, bool>,
  flower: f64,
}

#[allow(unused)]impl Random for Cave {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Cave {
    Cave::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum Fan {
  A(i64),
  B(Vec<u8>),
  C(Album),
  D(Cave),
}

impl Random for Fan {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Fan {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Album = rng.gen_ext(); v }),
      3 => Self::D({ let v: Cave = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum Typewriter {
  A(Vec<u8>),
  B(Fan),
}

impl Random for Typewriter {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Typewriter {
    let v: usize = rng.gen_range(0, 2);
    match v {
      0 => Self::A({ let v: Vec<u8> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Fan = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash)]
#[serde(untagged)]
pub enum Carpet1 {
  A(bool),
  B(i64),
  C(Vec<u8>),
}

impl Random for Carpet1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Carpet1 {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: i64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<u8> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Default)]
#[serde(rename_all = "snake_case", rename = "Torch1")]
pub struct Air {
  #[serde(skip)]
  hose: Vec<u8>,
  #[serde(flatten)]
  videotape: HashMap<String, HashMap<String, ((bool, ), HashMap<String, Carpet1>, )>>,
}

#[allow(unused)]impl Random for Air {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Air {
    Air::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "kebab-case")]
pub struct Record {
}

#[allow(unused)]impl Random for Record {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Record {
    Record::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "UPPERCASE", rename = "Coffee1")]
pub struct Ship2 {
}

#[allow(unused)]impl Random for Ship2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Ship2 {
    Ship2::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "snake_case", rename = "Meat2")]
pub struct Egg {
  bathtub: bool,
  #[serde(rename = "leg1")]
  vampire: ((f64, i64, ), ),
}

#[allow(unused)]impl Random for Egg {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Egg {
    Egg::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash)]
#[serde(untagged)]
pub enum Roof1 {
  A(bool),
  B(String),
  C(Vec<u8>),
  D(()),
}

impl Random for Roof1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Roof1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: String = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<u8> = rng.gen_ext(); v }),
      3 => Self::D({ let v: () = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum TennisRacquet3 {
  A(i64),
  B(f64),
  C(String),
  D(Vec<u8>),
}

impl Random for TennisRacquet3 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> TennisRacquet3 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: f64 = rng.gen_ext(); v }),
      2 => Self::C({ let v: String = rng.gen_ext(); v }),
      3 => Self::D({ let v: Vec<u8> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(untagged)]
pub enum Surveyor {
  A(i64),
  B(HashMap<String, HashSet<String>>),
  C(Vec<i64>),
  D(Option<HashMap<String, i64>>),
}

impl Random for Surveyor {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Surveyor {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashMap<String, HashSet<String>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: Vec<i64> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Option<HashMap<String, i64>> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum Vulture1 {
  A(i64),
  B(HashMap<String, TennisRacquet3>),
  C(HashSet<Vec<u8>>),
  D(Surveyor),
}

impl Random for Vulture1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Vulture1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashMap<String, TennisRacquet3> = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashSet<Vec<u8>> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Surveyor = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(rename = "Thermometer", deny_unknown_fields)]
pub struct Barbecue {
  carpet1: Vulture1,
}

#[allow(unused)]impl Random for Barbecue {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Barbecue {
    Barbecue::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Bible {
}

#[allow(unused)]impl Random for Bible {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bible {
    Bible::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct Air1 {
  #[serde(skip)]
  boss: String,
  #[serde(rename = "slave")]
  circle: f64,
}

#[allow(unused)]impl Random for Air1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Air1 {
    Air1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "PascalCase", rename = "CompactDisc", deny_unknown_fields)]
pub struct Rainbow1 {
  #[serde(skip)]
  vacuum: i64,
  arm: Air1,
}

#[allow(unused)]impl Random for Rainbow1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Rainbow1 {
    Rainbow1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields, default)]
pub struct Kaleidoscope1 {
  saddle: bool,
}

#[allow(unused)]impl Random for Kaleidoscope1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Kaleidoscope1 {
    Kaleidoscope1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
pub struct Kaleidoscope {
  #[serde(flatten)]
  pocket1: Kaleidoscope1,
}

#[allow(unused)]impl Random for Kaleidoscope {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Kaleidoscope {
    Kaleidoscope::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(default)]
pub struct Wheelchair {
  crystal3: String,
}

#[allow(unused)]impl Random for Wheelchair {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Wheelchair {
    Wheelchair::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum Cappuccino {
  A(Vec<u8>),
  B(HashMap<String, f64>),
  C(HashSet<Vec<u8>>),
  D(Wheelchair),
}

impl Random for Cappuccino {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Cappuccino {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: Vec<u8> = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashMap<String, f64> = rng.gen_ext(); v }),
      2 => Self::C({ let v: HashSet<Vec<u8>> = rng.gen_ext(); v }),
      3 => Self::D({ let v: Wheelchair = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "snake_case", default)]
pub struct Gemstone1 {
  #[serde(rename = "planet1")]
  drink1: bool,
}

#[allow(unused)]impl Random for Gemstone1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Gemstone1 {
    Gemstone1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "camelCase", rename = "Egg1")]
pub struct Baby2 {
}

#[allow(unused)]impl Random for Baby2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Baby2 {
    Baby2::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", default)]
pub struct PostOffice1 {
  #[serde(skip)]
  ship: ((Baby2, Vec<bool>, ), HashSet<String>, ),
  satellite2: bool,
}

#[allow(unused)]impl Random for PostOffice1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> PostOffice1 {
    PostOffice1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Brain2", default)]
pub struct Star2 {
}

#[allow(unused)]impl Random for Star2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Star2 {
    Star2::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Sex1 {
  #[serde(skip_deserializing)]
  torpedo: f64,
  child: Vec<u8>,
}

#[allow(unused)]impl Random for Sex1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Sex1 {
    Sex1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename = "Foot1", deny_unknown_fields, default)]
pub struct Pyramid1 {
  car2: (f64, ),
}

#[allow(unused)]impl Random for Pyramid1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Pyramid1 {
    Pyramid1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename = "Fruit2")]
pub struct Window5 {
  film: f64,
}

#[allow(unused)]impl Random for Window5 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Window5 {
    Window5::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Girl4", default)]
pub struct Drill3 {
}

#[allow(unused)]impl Random for Drill3 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Drill3 {
    Drill3::new(
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Circle {
  floodlight: bool,
  #[serde(skip)]
  butterfly: String,
}

#[allow(unused)]impl Random for Circle {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Circle {
    Circle::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash)]
#[serde(untagged)]
pub enum Shop1 {
  A(i64),
  B(Vec<u8>),
  C((bool, )),
  D(Circle),
}

impl Random for Shop1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Shop1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: i64 = rng.gen_ext(); v }),
      1 => Self::B({ let v: Vec<u8> = rng.gen_ext(); v }),
      2 => Self::C({ let v: (bool, ) = rng.gen_ext(); v }),
      3 => Self::D({ let v: Circle = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Knife2 {
  #[serde(rename = "needle1")]
  pepper1: Shop1,
  #[serde(rename = "bathtub1")]
  room1: String,
}

#[allow(unused)]impl Random for Knife2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Knife2 {
    Knife2::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "snake_case", default)]
pub struct Circus2 {
  #[serde(rename = "microscope1")]
  map1: String,
}

#[allow(unused)]impl Random for Circus2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Circus2 {
    Circus2::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Tapestry2 {
  shop: (Circus2, HashSet<i64>, ),
}

#[allow(unused)]impl Random for Tapestry2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Tapestry2 {
    Tapestry2::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(rename_all = "lowercase", rename = "Bathtub")]
pub struct Bank1 {
  #[serde(flatten)]
  satellite3: Knife2,
  sportscar1: (Tapestry2, ),
}

#[allow(unused)]impl Random for Bank1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bank1 {
    Bank1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq)]
#[serde(untagged)]
pub enum Sandwich {
  A(HashSet<String>),
  B(Bank1),
  C(Option<HashMap<String, HashSet<String>>>),
}

impl Random for Sandwich {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Sandwich {
    let v: usize = rng.gen_range(0, 3);
    match v {
      0 => Self::A({ let v: HashSet<String> = rng.gen_ext(); v }),
      1 => Self::B({ let v: Bank1 = rng.gen_ext(); v }),
      2 => Self::C({ let v: Option<HashMap<String, HashSet<String>>> = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Sphere1 {
  #[serde(skip)]
  nail1: Vec<HashSet<String>>,
  insect: f64,
}

#[allow(unused)]impl Random for Sphere1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Sphere1 {
    Sphere1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Default)]
#[serde(rename_all = "lowercase")]
pub struct Chisel {
  #[serde(skip)]
  cappuccino1: HashSet<Vec<u8>>,
  chief2: Vec<u8>,
}

#[allow(unused)]impl Random for Chisel {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Chisel {
    Chisel::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Hash,Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields, default)]
pub struct Planet1 {
  #[serde(skip)]
  god1: Vec<bool>,
  #[serde(rename = "telescope1")]
  man1: String,
}

#[allow(unused)]impl Random for Planet1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Planet1 {
    Planet1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,PartialEq,Eq,Default)]
#[serde(rename_all = "snake_case", default)]
pub struct Bomb1 {
  #[serde(flatten)]
  carpet2: HashMap<String, i64>,
}

#[allow(unused)]impl Random for Bomb1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Bomb1 {
    Bomb1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "PascalCase", rename = "Navy1")]
pub struct Shop2 {
  #[serde(skip_deserializing)]
  foot2: Bomb1,
  hieroglyph1: f64,
}

#[allow(unused)]impl Random for Shop2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Shop2 {
    Shop2::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
pub struct Air2 {
  #[serde(skip)]
  skeleton1: f64,
  baby3: Vec<HashSet<String>>,
}

#[allow(unused)]impl Random for Air2 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Air2 {
    Air2::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Knife3")]
pub struct DataBase1 {
  #[serde(flatten)]
  milk1: HashMap<String, Shop2>,
  train1: Air2,
}

#[allow(unused)]impl Random for DataBase1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> DataBase1 {
    DataBase1::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "lowercase", rename = "Sex2", deny_unknown_fields, default)]
pub struct Chocolates1 {
  airforce2: f64,
}

#[allow(unused)]impl Random for Chocolates1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Chocolates1 {
    Chocolates1::new(
      rng.gen_ext(),
    )
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new)]
#[serde(untagged)]
pub enum SportsCar1 {
  A(bool),
  B(HashMap<String, Vec<u8>>),
  C((Option<String>, Option<HashMap<String, f64>>, )),
  D(Chocolates1),
}

impl Random for SportsCar1 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> SportsCar1 {
    let v: usize = rng.gen_range(0, 4);
    match v {
      0 => Self::A({ let v: bool = rng.gen_ext(); v }),
      1 => Self::B({ let v: HashMap<String, Vec<u8>> = rng.gen_ext(); v }),
      2 => Self::C({ let v: (Option<String>, Option<HashMap<String, f64>>, ) = rng.gen_ext(); v }),
      3 => Self::D({ let v: Chocolates1 = rng.gen_ext(); v }),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone,new,Default)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", rename = "Signature1", default)]
pub struct Circus3 {
  woman: f64,
  #[serde(skip_deserializing)]
  freeway2: Vec<u8>,
}

#[allow(unused)]impl Random for Circus3 {
  fn random<R: Rng + ?Sized>(rng: &mut R) -> Circus3 {
    Circus3::new(
      rng.gen_ext(),
      rng.gen_ext(),
    )
  }
}



fn main() {
let mut rng = rand::thread_rng();

  let v: HashMap<String, f64> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<Option<HashSet<bool>>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Option<String> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashSet<Vec<u8>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Torch = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<String> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: (Vec<u8>, ) = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: i64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Option<(HashMap<String, HashMap<String, Vacuum>>, bool, )> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashMap<String, CoffeeShop> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Necklace = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: (HashSet<bool>, ) = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Water = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Aircraft = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Option<Vec<u8>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Necklace1 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Option<Vec<u8>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Explosive = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: i64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<Option<i64>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashSet<i64> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: i64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Drum = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Typewriter = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: i64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: i64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Air = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Record = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: () = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Ship2 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Egg = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<HashSet<i64>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashSet<Roof1> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashSet<i64> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashMap<String, Barbecue> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Bible = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Rainbow1 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Kaleidoscope = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<String> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashMap<String, f64> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashSet<bool> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<i64> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Cappuccino = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Option<(String, )> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Gemstone1 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: PostOffice1 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Star2 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: (Sex1, Pyramid1, ) = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashSet<Vec<u8>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: bool = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: (Window5, HashMap<String, Vec<Option<Drill3>>>, ) = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: i64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Sandwich = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Sphere1 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Chisel = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Planet1 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<Option<bool>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: DataBase1 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: SportsCar1 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: f64 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Option<Vec<u8>> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: HashMap<String, f64> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Vec<u8> = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: Circus3 = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

  let v: String = rng.gen_ext();
  println!("======================\n");
  println!("{}", serde_json::to_string(&v).unwrap());

}

