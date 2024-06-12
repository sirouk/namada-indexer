use diesel::prelude::Insertable;
use diesel::{Queryable, Selectable};
use serde::Serialize;
use shared::parameters::Parameters;

use crate::schema::chain_parameters;

#[derive(Serialize, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = chain_parameters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ParametersInsertDb {
    pub epoch: i32,
    pub unbonding_length: i32,
    pub pipeline_length: i32,
    pub epochs_per_year: i32,
    pub min_num_of_blocks: i32,
    pub min_duration: i32,
    pub apr: String,
    pub native_token_address: String,
}

pub type ParametersDb = ParametersInsertDb;

impl From<Parameters> for ParametersInsertDb {
    fn from(value: Parameters) -> Self {
        Self {
            epoch: value.epoch as i32,
            unbonding_length: value.unbonding_length as i32,
            pipeline_length: value.pipeline_length as i32,
            epochs_per_year: value.epochs_per_year as i32,
            min_num_of_blocks: value.min_num_of_blocks as i32,
            min_duration: value.min_duration as i32,
            apr: value.apr,
            native_token_address: value.native_token_address,
        }
    }
}
