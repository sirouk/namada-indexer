use diesel::allow_columns_to_appear_in_same_group_by_clause;
use diesel::expression::{SqlLiteral, ValidGrouping};

use crate::schema::{bonds, unbonds, validators};

use crate::schema::{bonds, validators};

allow_columns_to_appear_in_same_group_by_clause!(
    bonds::address,
    validators::id,
    validators::namada_address,
    validators::voting_power,
    validators::max_commission,
    validators::commission,
    validators::name,
    validators::email,
    validators::website,
    validators::description,
    validators::discord_handle,
    validators::avatar,
    validators::state,
);

macro_rules! impl_valid_grouping {
    ($valid_grouping_type:ty, $column:path) => {
        impl ValidGrouping<$valid_grouping_type> for $column {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
    };

    ($valid_grouping_type:ty, $column:path, $($columns:path),+) => {
        impl ValidGrouping<$valid_grouping_type> for $column {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }

        impl_valid_grouping!($valid_grouping_type, $($columns),+);
    };
}

impl_valid_grouping!(
    (
        unbonds::address,
        validators::id,
        SqlLiteral<diesel::sql_types::Integer>
    ),
    unbonds::address,
    validators::id,
    validators::namada_address,
    validators::voting_power,
    validators::max_commission,
    validators::commission,
    validators::name,
    validators::email,
    validators::website,
    validators::description,
    validators::discord_handle,
    validators::avatar,
    validators::state
);
