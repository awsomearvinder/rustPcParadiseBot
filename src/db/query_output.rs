/// The crate level Query Output which represents everything the database can return.
#[allow(dead_code)]
pub enum QueryOutput {
    DiscordUser(UserInfo),
    DiscordMessages(Vec<MessageInfo>)
}

pub struct UserInfo;
pub struct MessageInfo;
