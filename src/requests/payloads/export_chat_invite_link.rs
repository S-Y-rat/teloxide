use crate::{
    requests::{dynamic, json, Method},
    types::ChatId,
};

/// Use this method to generate a new invite link for a chat; any previously generated link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the new invite link as String on success.Note: Each administrator in a chat generates their own invite links. Bots can't use invite links generated by other administrators. If you want your bot to work with invite links, it will need to generate its own link using exportChatInviteLink – after this the link will become available to the bot via the getChat method. If your bot needs to generate a new invite link replacing its previous one, use exportChatInviteLink again.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
}

impl Method for ExportChatInviteLink {
    type Output = String;

    const NAME: &'static str = "exportChatInviteLink";
}

impl json::Payload for ExportChatInviteLink {}

impl dynamic::Payload for ExportChatInviteLink {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl ExportChatInviteLink {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
        }
    }
}

impl json::Request<'_, ExportChatInviteLink> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }
}
                 