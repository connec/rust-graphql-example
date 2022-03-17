use juniper::FieldResult;
use tokio::sync::{Mutex, MutexGuard};
use uuid::Uuid;

use crate::{
    db::Db,
    model::{Human, NewHuman},
};

pub(crate) struct Context {
    db: Mutex<Db>,
}

impl Context {
    pub(crate) fn new(db: Db) -> Self {
        Self { db: Mutex::new(db) }
    }

    pub(super) async fn db(&self) -> MutexGuard<'_, Db> {
        self.db.lock().await
    }
}

impl juniper::Context for Context {}

pub(crate) struct Query;

/// The root query structure.
#[juniper::graphql_object(Context = Context)]
impl Query {
    /// The API version.
    fn api_version() -> &str {
        "v1"
    }

    /// All the humanoid creatures in the Star Wars universe that we know about.
    async fn humans(context: &Context) -> FieldResult<Vec<Human>> {
        Ok(context.db().await.list_humans().await?)
    }

    /// A humanoid creature in the Star Wars universe.
    async fn human(context: &Context, id: Uuid) -> FieldResult<Human> {
        let human = context.db().await.get_human(&id).await?;
        Ok(human)
    }
}

pub(crate) struct Mutation;

/// The root mutation structure.
#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_human(context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        let human = context.db().await.insert_human(new_human).await?;
        Ok(human)
    }
}
