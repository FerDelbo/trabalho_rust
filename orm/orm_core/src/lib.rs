pub use orm_derive::Model;

pub trait Model {
    fn name_table() -> &'static str;
    fn create(&self); //create
    fn find(&self);   //retrive 
    fn update(&self); //update 
    fn delete(&self); //delete
    fn insert (&self);
}
