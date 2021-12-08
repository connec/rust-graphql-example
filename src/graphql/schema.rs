use juniper::FieldResult;
use uuid::Uuid;

use crate::Context;

pub(crate) struct Query;

/// The root query structure.
#[juniper::graphql_object(Context = Context)]
impl Query {
    /// The API version.
    fn api_version() -> &str {
        "v1"
    }

    /// A humanoid creature in the Star Wars universe.
    fn human(context: &Context, id: Uuid) -> FieldResult<Human> {
        let human = context.find_human(&id)?;
        Ok(human)
    }
}

pub(crate) struct Mutation;

/// The root mutation structure.
#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn create_human(context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        let human = context.insert_human(new_human)?;
        Ok(human)
    }
}

/// Episodes in the original (and best) Star Wars trilogy.
#[derive(Clone, Copy, juniper::GraphQLEnum)]
pub(crate) enum Episode {
    NewHope,
    Empire,
    Jedi,
}

/// A humanoid creature in the Star Wars universe.
#[derive(Clone, juniper::GraphQLObject)]
pub(crate) struct Human {
    /// Their unique identifier, assigned by us.
    id: Uuid,

    /// Their name.
    name: String,

    /// The episodes in which they appeared.
    appears_in: Vec<Episode>,

    /// Their home planet.
    home_planet: String,
}

impl Human {
    pub(crate) fn new(new_human: NewHuman) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        }
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

/// A new humanoid creature in the Star Wars universe.
///
/// `id` is assigned by the server upon creation.
#[derive(juniper::GraphQLInputObject)]
pub(crate) struct NewHuman {
    /// Their name.
    name: String,

    /// The episodes in which they appeared.
    appears_in: Vec<Episode>,

    /// Their home planet.
    home_planet: String,
}

impl NewHuman {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
