use std::sync::mpsc::Sender;
use std::sync::mpsc::SendError;

/// Adapters will receive on a port that accepts a stream of `AdapterMsg`. The
/// Outgoing variant contains a message to return. All other variants are
/// control messages which a well behaved adapter should handle.
///
/// The Shutdown variant indicates that the bot wishes to shutdown.
pub enum AdapterMsg {
    Outgoing(OutgoingMessage),
    Shutdown
}

/// An OutgoingMessage is a response to some IncomingMessage. It contains a
/// String and a copy of the IncomingMessage that it is in reply to.
pub struct OutgoingMessage {
    response: String,
    incoming: IncomingMessage
}

impl OutgoingMessage {
    pub fn new(response: String, incoming: IncomingMessage) -> OutgoingMessage {
        OutgoingMessage {
            response: response,
            incoming: incoming
        }
    }

    /// Return a reference to the IncomingMessage that this message is in
    /// response to.
    pub fn get_incoming<'a>(&'a self) -> &'a IncomingMessage {
        &self.incoming
    }

    /// Get ref to response bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.response.as_bytes()
    }
}

/// adapters convert strings they receive into an IncomingMessage. The
/// properties on this struct exist to help adapters route any `OutgoingMessage`
/// back to where the IncomingMessage originated.
///
/// Types implementing the MessageHandler trait should use IncomingMessage.reply
/// to respond. It is up to the ChatAdapter to decide what `reply` means in the
/// context of the service it provides.
#[derive(Clone)]
pub struct IncomingMessage {
    message: String,
    from_adapter: String,
    server: Option<String>,
    channel: Option<String>,
    user: Option<String>,
    tx: Sender<AdapterMsg>
}

impl IncomingMessage {
    pub fn new(from_adapter: String, server: Option<String>, channel: Option<String>,
               user: Option<String>, message: String,
               sender: Sender<AdapterMsg>) -> IncomingMessage {
        IncomingMessage {
            from_adapter: from_adapter,
            server: server,
            channel: channel,
            user: user,
            message: message,
            tx: sender
        }
    }

    /// from_str exists for testing purposes only. This function will likely
    /// disappear in the future as providing a broken std::sync::mpsc::Sender
    /// isn't particularly helpful.
    #[cfg(test)]
    pub fn from_str(message: &str) -> IncomingMessage {
        use std::sync::mpsc::channel;
        let (tx, _) = channel();
        IncomingMessage::new("test".to_owned(), None, None, None, message.to_owned(), tx)
    }

    /// Get the message
    pub fn get_contents(&self) -> &str {
        self.message.as_ref()
    }

    /// Reply to the message.
    pub fn reply(&self, msg: String) -> Result<(), SendError<AdapterMsg>> {
        let outgoing = OutgoingMessage::new(msg, self.to_owned());
        self.tx.send(AdapterMsg::Outgoing(outgoing))
    }
}
