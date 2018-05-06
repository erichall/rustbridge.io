pub mod invite;
pub mod salt;
pub mod user;
pub mod workshop;

use failure::{Error, ResultExt};

pub trait Sanitize {
    fn sanitize(&self) -> Result<(), Error>;
}

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

pub trait Resource: Validate + Sanitize {
    type Model;

    fn create(&self) -> Result<(), Error>;
    fn read_all() -> Result<Vec<Self::Model>, Error>;
    fn read_one(id: usize) -> Result<Self::Model, Error>;
    fn update(&self, model_id: usize) -> Result<(), Error>;
    fn delete(&self, model_id: usize) -> Result<(), Error>;
}
