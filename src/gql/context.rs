use crate::{db::Db};

pub(crate) struct Context {
    db: Db,
}

impl Context {
    pub(crate) fn new(db: Db) -> Self {
        Self { db }
    }

    pub(super) fn db(&self) -> &Db {
        &self.db
    }
}

impl juniper::Context for Context {}
