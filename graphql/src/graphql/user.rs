use crate::{
    database::{
        bolt::Bolt,
        mem::{MemPool, MemcacheManager},
    },
    model::user::User,
};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

/// Define the context for your GraphQL schema.
#[derive(Clone)]
pub struct Context {
    /// Pool of connections to the Memcached database.
    pub memcached: MemPool,
    /// Pool of connections to the Neo4j or Memgraph databases using Bolt protocol.
    pub bolt: Bolt,
}
impl juniper::Context for Context {}

/// Implement the GraphQL object for the User struct.
#[graphql_object(context = Context, description = "Information about a user.")]
impl User {
    /// Returns the unique user identifier.
    fn vanity(&self) -> &str {
        &self.vanity
    }

    /// Returns the user's pseudonym, which could also be their first name and last name.
    fn username(&self) -> &str {
        &self.username
    }

    /// Returns the count of other users that follow the user.
    fn followers(&self) -> i32 {
        self.followers
    }

    /// Returns the count of accounts that the user is following.
    fn following(&self) -> i32 {
        self.following
    }

    /// Returns a hash representing the user's avatar image.
    fn avatar(&self) -> Option<String> {
        self.avatar.clone()
    }

    /// Returns the biography or description of the user.
    fn bio(&self) -> Option<String> {
        self.bio.clone()
    }

    /// Returns the total count of posts made by the user.
    fn posts_count(&self) -> i32 {
        self.posts_count
    }

    /// Returns a bitmask representing various user flags.
    fn flags(&self) -> i32 {
        self.flags
    }

    /// Returns a list of post content created by the user.
    fn posts(&self) -> Vec<String> {
        vec![]
    }
}

/// Define the root query object
#[derive(Clone, Copy, Debug)]
pub struct Query;

/// Implement the GraphQL object for the root query
#[graphql_object(context = Context)]
impl Query {
    /// Define an asynchronous method to retrieve a user by vanity
    async fn get_user(context: &Context, vanity: String) -> User {
        println!("{:?}", context.memcached.get("test").unwrap());

        User {
            vanity,
            username: "Test".to_string(),
            followers: 1_281_181,
            following: 0,
            avatar: None,
            bio: Some("Pierre's bf".to_string()),
            posts_count: 0,
            flags: 2,
            posts: vec![],
        }
    }
}

/// Define the schema using RootNode
type Schema = RootNode<
    'static,
    Query,
    EmptyMutation<Context>,
    EmptySubscription<Context>,
>;

/// Create the schema instance
pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}
