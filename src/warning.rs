use cosmic::{
    widget::{warning, Column, Container},
    Element,
};

use crate::gui::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum WarnMessages {
    Info,
    AppName,
    AppUrl,
    AppIcon,
    AppBrowser,
}

impl std::fmt::Display for WarnMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            WarnMessages::Info => write!(f, "You don't meet requirements"),
            WarnMessages::AppName => write!(f, "  - App name must be longer than 3 characters"),
            WarnMessages::AppUrl => write!(
                f,
                "  - You must provide valid URL starting with http:// or https://"
            ),
            WarnMessages::AppIcon => write!(f, "  - You must select an Icon for your launcher"),
            WarnMessages::AppBrowser => {
                write!(f, "  - Please select a browser. Make sure at least one is installed system-wide or via flatpak.")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Warning {
    pub messages: Vec<WarnMessages>,
    pub show: bool,
}

impl Default for Warning {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            show: false,
        }
    }
}

impl Warning {
    pub fn new(messages: Vec<WarnMessages>, show: bool) -> Self {
        Self { messages, show }
    }

    pub fn push_warn(&mut self, message: WarnMessages) -> &mut Self {
        if !self.messages.contains(&message) {
            self.messages.push(message);
        }
        self
    }

    pub fn remove_warn(&mut self, message: WarnMessages) -> &mut Self {
        self.messages.retain(|m| *m != message);
        self
    }

    pub fn view(&self) -> Element<Message> {
        let mut content = String::new();

        for line in &self.messages {
            content.push_str(&format!("{}\n", line));
        }

        let warn = warning(content);

        if self.show {
            Container::new(warn).into()
        } else {
            Container::new(Column::new()).into()
        }
    }
}
