// For now, the application is simple enough that our domain types can derive their serialization
// to/from the DB and GraphQL, and the same types can be used for both. In future, we may need
// different types for the DB and for GraphQL, with conversions between them.

use uuid::Uuid;

/// A humanoid creature in the Star Wars universe.
#[derive(Clone, Debug, juniper::GraphQLObject, sqlx::FromRow)]
pub(crate) struct Human {
    /// Their unique identifier, assigned by us.
    pub(crate) id: Uuid,

    /// Their name.
    pub(crate) name: String,

    /// The episodes in which they appeared.
    pub(crate) appears_in: EpisodeVec,

    /// Their home planet.
    pub(crate) home_planet: String,
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

    pub(crate) fn appears_in(&self) -> EpisodeSlice {
        EpisodeSlice(&self.appears_in)
    }

    pub(crate) fn home_planet(&self) -> &str {
        &self.home_planet
    }
}

/// Episodes in the original (and best) Star Wars trilogy.
#[derive(Clone, Copy, Debug, juniper::GraphQLEnum, sqlx::Type)]
#[sqlx(type_name = "episode")]
#[sqlx(rename_all = "snake_case")]
pub(crate) enum Episode {
    /// Star Wars: Episode IV – A New Hope
    NewHope,

    /// Star Wars: Episode V – The Empire Strikes Back
    Empire,

    /// Star Wars: Episode VI – Return of the Jedi
    Jedi,
}

// Workarounds for https://github.com/launchbadge/sqlx/issues/298

#[derive(Clone, Copy, Debug, sqlx::Encode)]
pub(crate) struct EpisodeSlice<'a>(&'a [Episode]);

impl sqlx::Type<sqlx::Postgres> for EpisodeSlice<'_> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_episode")
    }
}

#[derive(Clone, Debug, sqlx::Decode)]
pub(crate) struct EpisodeVec(Vec<Episode>);

impl sqlx::Type<sqlx::Postgres> for EpisodeVec {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_episode")
    }
}

impl From<Vec<Episode>> for EpisodeVec {
    fn from(episodes: Vec<Episode>) -> Self {
        Self(episodes)
    }
}

impl<S> juniper::GraphQLType<S> for EpisodeVec
where
    S: juniper::ScalarValue,
{
    fn name(_: &Self::TypeInfo) -> Option<&'static str> {
        None
    }

    fn meta<'r>(
        info: &Self::TypeInfo,
        registry: &mut juniper::Registry<'r, S>,
    ) -> juniper::meta::MetaType<'r, S>
    where
        S: 'r,
    {
        Vec::<Episode>::meta(info, registry)
    }
}

impl<S> juniper::GraphQLValue<S> for EpisodeVec
where
    S: juniper::ScalarValue,
{
    type Context = <Vec<Episode> as juniper::GraphQLValue<S>>::Context;
    type TypeInfo = <Vec<Episode> as juniper::GraphQLValue<S>>::TypeInfo;

    fn type_name(&self, _: &Self::TypeInfo) -> Option<&'static str> {
        None
    }

    fn resolve(
        &self,
        info: &Self::TypeInfo,
        selection: Option<&[juniper::Selection<S>]>,
        executor: &juniper::Executor<Self::Context, S>,
    ) -> juniper::ExecutionResult<S> {
        self.0.resolve(info, selection, executor)
    }
}

impl<S> juniper::GraphQLValueAsync<S> for EpisodeVec
where
    S: juniper::ScalarValue + Send + Sync,
{
    fn resolve_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        selection: Option<&'a [juniper::Selection<S>]>,
        executor: &'a juniper::Executor<Self::Context, S>,
    ) -> futures::future::BoxFuture<'a, juniper::ExecutionResult<S>> {
        self.0.resolve_async(info, selection, executor)
    }
}

impl<S> juniper::marker::IsOutputType<S> for EpisodeVec where S: juniper::ScalarValue {}
