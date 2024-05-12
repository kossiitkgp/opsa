pub struct Message {
    pub id: i32,
    pub text: String,
    pub user: User,
}

pub struct User {
    pub id: i32,
    pub name: String,
    pub avatar_url: String,
}

pub struct Channel {
    pub name: String,
}
