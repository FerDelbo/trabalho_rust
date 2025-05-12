pub mod db;
pub mod conf;
pub mod types;

pub use db::{execute};
pub use types::{Boolean, Date, Decimal, Integer, Text};

pub trait Model {
    fn name_table() -> &'static str;
    fn create(&self); //create
    fn find(&self);   //retrive 
    fn update(&self); //update 
    fn delete(&self); //delete
    fn insert (&self);
}
