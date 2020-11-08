use rand::{
    distributions::{Alphanumeric, Distribution, Standard},
    Rng,
};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    pub static NAMES: RefCell<HashMap<String, usize>> = RefCell::new(HashMap::new());
}

macro_rules! dedup {
    ($name:expr) => {
        NAMES.with(|map| {
            let mut map = map.borrow_mut();
            let entry = map.entry($name.clone()).or_insert(0);
            let name = format!(
                "{}{}",
                $name,
                if *entry == 0 {
                    "".into()
                } else {
                    entry.to_string()
                }
            );
            *entry += 1;
            name
        })
    };
}

pub fn random_type_name<R: Rng + ?Sized>(rng: &mut R) -> String {
    let name = KEYWORDS[rng.gen_range(0, KEYWORDS.len())].to_string();
    dedup!(name)
}

pub fn random_field_name<R: Rng + ?Sized>(rng: &mut R) -> String {
    let name = KEYWORDS[rng.gen_range(0, KEYWORDS.len())].to_lowercase();
    dedup!(name)
}

const KEYWORDS: &'static [&'static str] = &[
    "Adult",
    "Aeroplane",
    "Air",
    "Aircraft",
    "Carrier",
    "Airforce",
    "Airport",
    "Album",
    "Alphabet",
    "Apple",
    "Arm",
    "Army",
    "Baby",
    "Baby",
    "Backpack",
    "Balloon",
    "Banana",
    "Bank",
    "Barbecue",
    "Bathroom",
    "Bathtub",
    "Bed",
    "Bed",
    "Bee",
    "Bible",
    "Bible",
    "Bird",
    "Bomb",
    "Book",
    "Boss",
    "Bottle",
    "Bowl",
    "Boy",
    "Brain",
    "Bridge",
    "Butterfly",
    "Button",
    "Cappuccino",
    "Car",
    "Car",
    "Carpet",
    "Carrot",
    "Cave",
    "Chair",
    "Chief",
    "Child",
    "Chisel",
    "Chocolates",
    "Church",
    "Church",
    "Circle",
    "Circus",
    "Circus",
    "Clock",
    "Clown",
    "Coffee",
    "CoffeeShop",
    "Comet",
    "CompactDisc",
    "Compass",
    "Computer",
    "Crystal",
    "Cup",
    "Cycle",
    "DataBase",
    "Desk",
    "Diamond",
    "Dress",
    "Drill",
    "Drink",
    "Drum",
    "Dung",
    "Ears",
    "Earth",
    "Egg",
    "Electricity",
    "Elephant",
    "Eraser",
    "Explosive",
    "Eyes",
    "Family",
    "Fan",
    "Feather",
    "Festival",
    "Film",
    "Finger",
    "Fire",
    "Floodlight",
    "Flower",
    "Foot",
    "Fork",
    "Freeway",
    "Fruit",
    "Fungus",
    "Game",
    "Garden",
    "Gas",
    "Gate",
    "Gemstone",
    "Girl",
    "Gloves",
    "God",
    "Grapes",
    "Guitar",
    "Hammer",
    "Hat",
    "Hieroglyph",
    "Highway",
    "Horoscope",
    "Horse",
    "Hose",
    "Ice",
    "IceCream",
    "Insect",
    "JetFighter",
    "Junk",
    "Kaleidoscope",
    "Kitchen",
    "Knife",
    "LeatherJacket",
    "Leg",
    "Library",
    "Liquid",
    "Magnet",
    "Man",
    "Map",
    "Maze",
    "Meat",
    "Meteor",
    "Microscope",
    "Milk",
    "Milkshake",
    "Mist",
    "Money",
    "Monster",
    "Mosquito",
    "Mouth",
    "Nail",
    "Navy",
    "Necklace",
    "Needle",
    "Onion",
    "PaintBrush",
    "Pants",
    "Parachute",
    "Passport",
    "Pebble",
    "Pendulum",
    "Pepper",
    "Perfume",
    "Pillow",
    "Plane",
    "Planet",
    "Pocket",
    "PostOffice",
    "Potato",
    "Printer",
    "Prison",
    "Pyramid",
    "Radar",
    "Rainbow",
    "Record",
    "Restaurant",
    "Rifle",
    "Ring",
    "Robot",
    "Rock",
    "Rocket",
    "Roof",
    "Room",
    "Rope",
    "Saddle",
    "Salt",
    "Sandpaper",
    "Sandwich",
    "Satellite",
    "School",
    "Sex",
    "Ship",
    "Shoes",
    "Shop",
    "Shower",
    "Signature",
    "Skeleton",
    "Slave",
    "Snail",
    "Software",
    "Solid",
    "SpaceShuttle",
    "Spectrum",
    "Sphere",
    "Spice",
    "Spiral",
    "Spoon",
    "SportsCar",
    "SpotLight",
    "Square",
    "Staircase",
    "Star",
    "Stomach",
    "Sun",
    "Sunglasses",
    "Surveyor",
    "SwimmingPool",
    "Sword",
    "Table",
    "Tapestry",
    "Teeth",
    "Telescope",
    "Television",
    "TennisRacquet",
    "Thermometer",
    "Tiger",
    "Toilet",
    "Tongue",
    "Torch",
    "Torpedo",
    "Train",
    "Treadmill",
    "Triangle",
    "Tunnel",
    "Typewriter",
    "Umbrella",
    "Vacuum",
    "Vampire",
    "Videotape",
    "Vulture",
    "Water",
    "Weapon",
    "Web",
    "Wheelchair",
    "Window",
    "Woman",
    "Worm",
    "XRay",
];
