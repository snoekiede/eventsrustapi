use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::event::{Event,NewEvent};
use crate::models::schema::events::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub pool: DBPool,
}



impl Database {
    

    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        
        Database { pool: result }
    }

    pub fn get_events(&self) -> Vec<Event> {
        
        events
            .load::<Event>(&mut self.pool.get().unwrap())
            .expect("Failed to get events.")
    }

    pub fn get_event(&self, find_id:i32) -> Option<Event> {
        events
            .find(find_id)
            .first::<Event>(&mut self.pool.get().unwrap())
            .ok()
    }

    pub fn create_event(&self,event:NewEvent)->Result<Event,diesel::result::Error>{

        diesel::insert_into(events).values(&event).get_result(&mut self.pool.get().unwrap())
    }

    pub fn delete_event(&self,find_id:i32)->Result<usize,diesel::result::Error>{
        diesel::delete(events.filter(id.eq(find_id))).execute(&mut self.pool.get().unwrap())
    }

    pub fn update_event(&self,event:Event)->Result<Event,diesel::result::Error>{
        diesel::update(events.filter(id.eq(event.id))).set(&event).get_result(&mut self.pool.get().unwrap())
    }
}

