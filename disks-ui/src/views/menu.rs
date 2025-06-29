use std::collections::HashMap;

use cosmic::{widget::menu, Element};
use crate::fl;

use crate::app::{ContextPage, Message};

pub fn menu_view(key_binds: &HashMap<menu::KeyBind, MenuAction>,) -> Vec<Element<Message>> {
    let menu_bar = menu::bar(vec![
        menu::Tree::with_children(
            menu::root("Image"),
            menu::items(
                key_binds,
                vec![
                    menu::Item::Button("New Disk Image", None, MenuAction::About),
                    menu::Item::Button("Attach Disk Image", None, MenuAction::AttachDisk),
                    menu::Item::Button("Create Disk From Drive", None, MenuAction::CreateDiskFrom),
                    menu::Item::Button("Restore Image to Drive", None, MenuAction::RestoreImageTo),
                ],
            ),
        ),
        menu::Tree::with_children(
            menu::root("Disk"),
            menu::items(
                key_binds,
                vec![
                    menu::Item::Button("Eject", None, MenuAction::Eject),
                    menu::Item::Button("Power Off", None, MenuAction::PowerOff),
                    menu::Item::Button("Format Disk", None, MenuAction::Format),
                    menu::Item::Button("Benchmark Disk", None, MenuAction::Benchmark),
                    menu::Item::Button("SMART Data & Self-Tests", None, MenuAction::SmartData),
                    menu::Item::Button("Drive Settings", None, MenuAction::DriveSettings),
                    menu::Item::Button("Standby Now", None, MenuAction::StandbyNow),
                    menu::Item::Button("Wake-up From Standby", None, MenuAction::Wakeup),
                ],
            ),
        ),
        menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &key_binds,
                vec![menu::Item::Button(fl!("about"), None, MenuAction::About)],
            ),
        ),
    ]);

    vec![menu_bar.into()] //, horizontal_space().into(), end_bar.into()]
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
    Eject,
    PowerOff,
    Format, 
    Benchmark,
    SmartData,
    DriveSettings,
    StandbyNow,
    Wakeup,
    NewDiskImage,
    AttachDisk,
    CreateDiskFrom,
    RestoreImageTo,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
            MenuAction::Eject => Message::Eject,
            MenuAction::PowerOff => Message::PowerOff,
            MenuAction::Format => Message::Format,
            MenuAction::Benchmark => Message::Benchmark,
            MenuAction::SmartData => Message::SmartData,
            MenuAction::DriveSettings => Message::DriveSettings,
            MenuAction::StandbyNow => Message::StandbyNow,
            MenuAction::Wakeup =>Message::Wakeup,
            MenuAction::NewDiskImage => Message::NewDiskImage,
            MenuAction::AttachDisk =>Message::AttachDisk,
            MenuAction::CreateDiskFrom =>Message::CreateDiskFrom,
            MenuAction::RestoreImageTo =>Message::RestoreImageTo,
            
        }
    }
}
