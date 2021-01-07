use js_sys::{Object, Reflect};
use num_traits::FromPrimitive;
pub use screeps;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NotOwner,
    NoPath,
    NameExists,
    Busy,
    NotFound,
    NotEnough,
    InvalidTarget,
    Full,
    NotInRange,
    InvalidArgs,
    Tired,
    NoBodypart,
    RclNotEnough,
    GclNotEnough,
    UnknownError,
}

fn map_c(return_code: i8) -> Result<(), Error> {
    map_e(screeps::ReturnCode::from_i8(return_code).ok_or(Error::UnknownError)?)
}

fn map_e(return_code: screeps::constants::ReturnCode) -> Result<(), Error> {
    match return_code {
        screeps::ReturnCode::Ok => Ok(()),
        screeps::ReturnCode::NotOwner => Err(Error::NotOwner),
        screeps::ReturnCode::NoPath => Err(Error::NoPath),
        screeps::ReturnCode::NameExists => Err(Error::NameExists),
        screeps::ReturnCode::Busy => Err(Error::Busy),
        screeps::ReturnCode::NotFound => Err(Error::NotFound),
        screeps::ReturnCode::NotEnough => Err(Error::NotEnough),
        screeps::ReturnCode::InvalidTarget => Err(Error::InvalidTarget),
        screeps::ReturnCode::Full => Err(Error::Full),
        screeps::ReturnCode::NotInRange => Err(Error::NotInRange),
        screeps::ReturnCode::InvalidArgs => Err(Error::InvalidArgs),
        screeps::ReturnCode::Tired => Err(Error::Tired),
        screeps::ReturnCode::NoBodypart => Err(Error::NoBodypart),
        screeps::ReturnCode::RclNotEnough => Err(Error::RclNotEnough),
        screeps::ReturnCode::GclNotEnough => Err(Error::GclNotEnough),
    }
}

pub struct Game;

impl Game {
    pub fn spawns() -> Spawns {
        Spawns(screeps::Game::spawns())
    }

    pub fn time() -> u32 {
        screeps::Game::time()
    }
}

pub struct Spawns(Object);

impl Spawns {
    pub fn get(&self, name: &str) -> Option<StructureSpawn> {
        Some(StructureSpawn(
            Reflect::get(&self.0, &name.into()).ok()?.into(),
        ))
    }
}

pub struct StructureSpawn(screeps::objects::StructureSpawn);

impl StructureSpawn {
    pub fn spawn_creep(
        &self,
        body: &[screeps::constants::creep::Part],
        name: &str,
        options: Option<Object>,
    ) -> Result<(), Error> {
        let result = self.0.spawn_creep(
            &body.iter().map(|&v| JsValue::from(v)).collect(),
            &name.into(),
            options,
        );
        map_c(result)
    }
}
