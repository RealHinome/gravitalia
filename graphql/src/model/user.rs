/// A struct representing a user profile.
#[derive(Clone, Debug)]
pub struct User {
    /// The user's vanity used as identification.
    pub vanity: String,
    /// The username of the user.
    pub username: String,
    /// The number of followers the user has.
    pub followers: i32,
    /// The number of accounts the user is following.
    pub following: i32,
    /// An optional hash of the user's avatar image.
    pub avatar: Option<String>,
    /// An optional biography or description of the user.
    pub bio: Option<String>,
    /// The total count of posts made by the user.
    pub posts_count: i32,
    /// A bitmask representing various user flags.
    pub flags: i32,
    /// A vector containing the content of posts made by the user.
    pub posts: Vec<String>,
}
