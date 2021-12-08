mod context;
mod schema;

use self::schema::{Human, NewHuman};
pub(crate) use self::{
    context::Context,
    schema::{Mutation, Query},
};
