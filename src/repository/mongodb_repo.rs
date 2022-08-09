use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

use crate::models::user_models::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading ENV variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");

        MongoRepo { col: col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc: User = User {
            id: None,
            name: new_user.name,
            title: new_user.title,
            location: new_user.location,
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursor = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");

        let users = cursor.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's details");

        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self, id: &String, user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let new_doc = doc! {
            "$set": {
                "id": user.id,
                "name": user.name,
                "title": user.title,
                "location": user.location,
            }
        };

        let update_doc = self
            .col
            .update_many(filter, new_doc, None)
            .ok()
            .expect("Error updating user");

        Ok(update_doc)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }
}
