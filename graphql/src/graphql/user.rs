use crate::{database::mem::MemcacheManager, model::user::User};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

/// Define the context for your GraphQL schema.
#[derive(Clone, Debug)]
pub struct Context {
    pub memcached: crate::database::mem::MemPool,
}
impl juniper::Context for Context {}

/// Implement the GraphQL object for the User struct.
#[graphql_object(context = Context, description = "Information about a user")]
impl User {
    #[graphql(description = "Unique user identifier.")]
    fn vanity(&self) -> &str {
        &self.vanity
    }

    #[graphql(
        description = "User's pseudonym, could be also firstname and lastname."
    )]
    fn username(&self) -> &str {
        &self.username
    }

    #[graphql(description = "Count of other users that follow the user.")]
    fn followers(&self) -> i32 {
        self.followers
    }

    #[graphql(description = "Count of accounts that the user is following.")]
    fn following(&self) -> i32 {
        self.following
    }

    #[graphql(description = "Hash representing the user's avatar image.")]
    fn avatar(&self) -> Option<String> {
        self.avatar.clone()
    }

    #[graphql(description = "Biography or description of the user.")]
    fn bio(&self) -> Option<String> {
        self.bio.clone()
    }

    #[graphql(description = "Total count of posts made by the user.")]
    fn posts_count(&self) -> i32 {
        self.posts_count
    }

    #[graphql(description = "Bitmask representing various user flags.")]
    fn flags(&self) -> i32 {
        self.flags
    }

    #[graphql(description = "List of post content created by the user.")]
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
