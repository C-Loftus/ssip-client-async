// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::fmt;
use std::io;
use std::str::FromStr;
use thiserror::Error as ThisError;

use strum_macros::Display as StrumDisplay;

/// Return code of SSIP commands
pub type ReturnCode = u16;

/// Message identifier
pub type MessageId = String;

/// Client identifier
pub type ClientId = String;

/// Message identifiers
#[derive(Debug, Clone)]
pub enum MessageScope {
    /// Last message from current client
    Last,
    /// Messages from all clients
    All,
    /// Specific message
    Message(MessageId),
}

impl fmt::Display for MessageScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageScope::Last => write!(f, "self"),
            MessageScope::All => write!(f, "all"),
            MessageScope::Message(id) => write!(f, "{}", id),
        }
    }
}

/// Client identifiers
#[derive(Debug, Clone)]
pub enum ClientScope {
    /// Current client
    Current,
    /// All clients
    All,
    /// Specific client
    Client(ClientId),
}

impl fmt::Display for ClientScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientScope::Current => write!(f, "self"),
            ClientScope::All => write!(f, "all"),
            ClientScope::Client(id) => write!(f, "{}", id),
        }
    }
}

/// Priority
#[derive(StrumDisplay, Debug, Clone)]
pub enum Priority {
    #[strum(serialize = "progress")]
    Progress,
    #[strum(serialize = "notification")]
    Notification,
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "important")]
    Important,
}

/// Punctuation mode.
#[derive(StrumDisplay, Debug, Clone)]
pub enum PunctuationMode {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "some")]
    Some,
    #[strum(serialize = "most")]
    Most,
    #[strum(serialize = "all")]
    All,
}

/// Capital letters recognition mode.
#[derive(StrumDisplay, Debug, Clone)]
pub enum CapitalLettersRecognitionMode {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "spell")]
    Spell,
    #[strum(serialize = "icon")]
    Icon,
}

/// Symbolic key names
#[derive(StrumDisplay, Debug, Clone)]
pub enum KeyName {
    #[strum(serialize = "space")]
    Space,
    #[strum(serialize = "underscore")]
    Underscore,
    #[strum(serialize = "double-quote")]
    DoubleQuote,
    #[strum(serialize = "alt")]
    Alt,
    #[strum(serialize = "control")]
    Control,
    #[strum(serialize = "hyper")]
    Hyper,
    #[strum(serialize = "meta")]
    Meta,
    #[strum(serialize = "shift")]
    Shift,
    #[strum(serialize = "super")]
    Super,
    #[strum(serialize = "backspace")]
    Backspace,
    #[strum(serialize = "break")]
    Break,
    #[strum(serialize = "delete")]
    Delete,
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "end")]
    End,
    #[strum(serialize = "enter")]
    Enter,
    #[strum(serialize = "escape")]
    Escape,
    #[strum(serialize = "f1")]
    F1,
    #[strum(serialize = "f2")]
    F2,
    #[strum(serialize = "f3")]
    F3,
    #[strum(serialize = "f4")]
    F4,
    #[strum(serialize = "f5")]
    F5,
    #[strum(serialize = "f6")]
    F6,
    #[strum(serialize = "f7")]
    F7,
    #[strum(serialize = "f8")]
    F8,
    #[strum(serialize = "f9")]
    F9,
    #[strum(serialize = "f10")]
    F10,
    #[strum(serialize = "f11")]
    F11,
    #[strum(serialize = "f12")]
    F12,
    #[strum(serialize = "f13")]
    F13,
    #[strum(serialize = "f14")]
    F14,
    #[strum(serialize = "f15")]
    F15,
    #[strum(serialize = "f16")]
    F16,
    #[strum(serialize = "f17")]
    F17,
    #[strum(serialize = "f18")]
    F18,
    #[strum(serialize = "f19")]
    F19,
    #[strum(serialize = "f20")]
    F20,
    #[strum(serialize = "f21")]
    F21,
    #[strum(serialize = "f22")]
    F22,
    #[strum(serialize = "f23")]
    F23,
    #[strum(serialize = "f24")]
    F24,
    #[strum(serialize = "home")]
    Home,
    #[strum(serialize = "insert")]
    Insert,
    #[strum(serialize = "kp-*")]
    KpMultiply,
    #[strum(serialize = "kp-+")]
    KpPlus,
    #[strum(serialize = "kp--")]
    KpMinus,
    #[strum(serialize = "kp-.")]
    KpDot,
    #[strum(serialize = "kp-/")]
    KpDivide,
    #[strum(serialize = "kp-0")]
    Kp0,
    #[strum(serialize = "kp-1")]
    Kp1,
    #[strum(serialize = "kp-2")]
    Kp2,
    #[strum(serialize = "kp-3")]
    Kp3,
    #[strum(serialize = "kp-4")]
    Kp4,
    #[strum(serialize = "kp-5")]
    Kp5,
    #[strum(serialize = "kp-6")]
    Kp6,
    #[strum(serialize = "kp-7")]
    Kp7,
    #[strum(serialize = "kp-8")]
    Kp8,
    #[strum(serialize = "kp-9")]
    Kp9,
    #[strum(serialize = "kp-enter")]
    KpEnter,
    #[strum(serialize = "left")]
    Left,
    #[strum(serialize = "menu")]
    Menu,
    #[strum(serialize = "next")]
    Next,
    #[strum(serialize = "num-lock")]
    NumLock,
    #[strum(serialize = "pause")]
    Pause,
    #[strum(serialize = "print")]
    Print,
    #[strum(serialize = "prior")]
    Prior,
    #[strum(serialize = "return")]
    Return,
    #[strum(serialize = "right")]
    Right,
    #[strum(serialize = "scroll-lock")]
    ScrollLock,
    #[strum(serialize = "tab")]
    Tab,
    #[strum(serialize = "up")]
    Up,
    #[strum(serialize = "window")]
    Window,
}

/// Notification type
#[derive(StrumDisplay, Debug, Clone)]
pub enum NotificationType {
    #[strum(serialize = "begin")]
    Begin,
    #[strum(serialize = "end")]
    End,
    #[strum(serialize = "cancel")]
    Cancel,
    #[strum(serialize = "pause")]
    Pause,
    #[strum(serialize = "resume")]
    Resume,
    #[strum(serialize = "index_mark")]
    IndexMark,
    #[strum(serialize = "all")]
    All,
}

/// Notification event type (returned by server)
#[derive(StrumDisplay, Debug, Clone)]
pub enum EventType {
    Begin,
    End,
    Cancel,
    Pause,
    Resume,
    IndexMark(String),
}

/// Event identifier
#[derive(Debug, Clone)]
pub struct EventId {
    // Message id
    pub message: String,
    // Client id
    pub client: String,
}

impl EventId {
    // New event identifier
    pub fn new(message: &str, client: &str) -> Self {
        Self {
            message: message.to_string(),
            client: client.to_string(),
        }
    }
}

/// Notification event
#[derive(Debug, Clone)]
pub struct Event {
    pub ntype: EventType,
    pub id: EventId,
}

impl Event {
    pub fn new(ntype: EventType, message: &str, client: &str) -> Event {
        Event {
            ntype,
            id: EventId::new(message, client),
        }
    }

    pub fn begin(message: &str, client: &str) -> Event {
        Event::new(EventType::Begin, message, client)
    }

    pub fn end(message: &str, client: &str) -> Event {
        Event::new(EventType::End, message, client)
    }

    pub fn index_mark(mark: String, message: &str, client: &str) -> Event {
        Event::new(EventType::IndexMark(mark), message, client)
    }

    pub fn cancel(message: &str, client: &str) -> Event {
        Event::new(EventType::Cancel, message, client)
    }

    pub fn pause(message: &str, client: &str) -> Event {
        Event::new(EventType::Pause, message, client)
    }

    pub fn resume(message: &str, client: &str) -> Event {
        Event::new(EventType::Resume, message, client)
    }
}

/// Synthesis voice
#[derive(Debug, PartialEq)]
pub struct SynthesisVoice {
    pub name: String,
    pub language: Option<String>,
    pub dialect: Option<String>,
}

impl SynthesisVoice {
    pub fn new(name: &str, language: Option<&str>, dialect: Option<&str>) -> SynthesisVoice {
        SynthesisVoice {
            name: name.to_string(),
            language: language.map(|s| s.to_string()),
            dialect: dialect.map(|s| s.to_string()),
        }
    }
    /// Parse Option::None or string "none" into Option::None
    fn parse_none(token: Option<&str>) -> Option<String> {
        match token {
            Some(s) => match s {
                "none" => None,
                s => Some(s.to_string()),
            },
            None => None,
        }
    }
}

impl FromStr for SynthesisVoice {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('\t');
        Ok(SynthesisVoice {
            name: String::from(iter.next().unwrap()),
            language: SynthesisVoice::parse_none(iter.next()),
            dialect: SynthesisVoice::parse_none(iter.next()),
        })
    }
}

/// Command status line
///
/// Consists in a 3-digits code and a message. It can be a success or a failure.
///
/// Examples:
/// - 216 OK OUTPUT MODULE SET
/// - 409 ERR RATE TOO HIGH
#[derive(Debug, PartialEq)]
pub struct StatusLine {
    pub code: ReturnCode,
    pub message: String,
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}
/// Client error, either I/O error or SSIP error.
#[derive(ThisError, Debug)]
pub enum ClientError {
    #[error("Invalid type")]
    InvalidType,
    #[error("I/O: {0}")]
    Io(io::Error),
    #[error("Not ready")]
    NotReady,
    #[error("SSIP: {0}")]
    Ssip(StatusLine),
    #[error("Too few lines")]
    TooFewLines,
    #[error("Too many lines")]
    TooManyLines,
    #[error("Truncated message")]
    TruncatedMessage,
    #[error("Unexpected status: {0}")]
    UnexpectedStatus(ReturnCode),
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> Self {
        if err.kind() == io::ErrorKind::WouldBlock {
            ClientError::NotReady
        } else {
            ClientError::Io(err)
        }
    }
}

/// Client result.
pub type ClientResult<T> = Result<T, ClientError>;

/// Client result consisting in a single status line
pub type ClientStatus = ClientResult<StatusLine>;

/// Client name
#[derive(Debug, Clone)]
pub struct ClientName {
    pub user: String,
    pub application: String,
    pub component: String,
}

impl ClientName {
    pub fn new(user: &str, application: &str) -> Self {
        ClientName::with_component(user, application, "main")
    }

    pub fn with_component(user: &str, application: &str, component: &str) -> Self {
        ClientName {
            user: user.to_string(),
            application: application.to_string(),
            component: component.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::{MessageScope, SynthesisVoice};

    #[test]
    fn parse_synthesis_voice() {
        // Voice with dialect
        let v1 =
            SynthesisVoice::from_str("Portuguese (Portugal)+Kaukovalta\tpt\tKaukovalta").unwrap();
        assert_eq!("Portuguese (Portugal)+Kaukovalta", v1.name);
        assert_eq!("pt", v1.language.unwrap());
        assert_eq!("Kaukovalta", v1.dialect.unwrap());

        // Voice without dialect
        let v2 = SynthesisVoice::from_str("Esperanto\teo\tnone").unwrap();
        assert_eq!("Esperanto", v2.name);
        assert_eq!("eo", v2.language.unwrap());
        assert!(matches!(v2.dialect, None));
    }

    #[test]
    fn format_message_scope() {
        assert_eq!("self", format!("{}", MessageScope::Last).as_str());
        assert_eq!("all", format!("{}", MessageScope::All).as_str());
        assert_eq!(
            "123",
            format!("{}", MessageScope::Message("123".to_string())).as_str()
        );
    }
}
