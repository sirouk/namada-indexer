use std::str::FromStr;

use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::Serialize;
use shared::validator::Validator;

use crate::schema::validators;

#[derive(Serialize, Queryable, Selectable, Clone)]
#[diesel(table_name = validators)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ValidatorDb {
    pub id: i32,
    pub namada_address: String,
    pub voting_power: i32,
    pub max_commission: String,
    pub commission: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub discord_handle: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Insertable, Clone)]
#[diesel(table_name = validators)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ValidatorInsertDb {
    pub namada_address: String,
    pub voting_power: i32,
    pub max_commission: String,
    pub commission: String,
}

#[derive(Serialize, AsChangeset, Clone)]
#[diesel(table_name = validators)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ValidatorUpdateMetadataDb {
    pub commission: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub discord_handle: Option<String>,
    pub avatar: Option<String>,
}

impl ValidatorInsertDb {
    pub fn from_validator(validator: Validator) -> Self {
        Self {
            namada_address: validator.address.to_string(),
            voting_power: f32::from_str(&validator.voting_power).unwrap()
                as i32,
            max_commission: validator.max_commission.clone(),
            commission: validator.commission.clone(),
        }
    }
}
