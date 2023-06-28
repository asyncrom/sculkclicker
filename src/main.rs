use mouce::common::MouseEvent::Press;
use mouce::error::Error;
use mouce::{Mouse, MouseActions};
use mouce::common::MouseButton::Left;
use iced::widget::{button, column, row, text};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
    mouce_manager: Box<dyn MouseActions>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    StartClicker,
    StopClicker,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0 ,
            mouce_manager: Mouse::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::StartClicker => {
                start_clicker(&mut self.mouce_manager);
            }
            Message::StopClicker => {
                stop_clicker(&mut self.mouce_manager)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            // button("Increment").on_press(Message::IncrementPressed),
            // text(self.value).size(50),
            // button("Decrement").on_press(Message::DecrementPressed),
            button("Start").on_press(Message::StartClicker),
            button("Stop").on_press(Message::StopClicker),
        ]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}



fn mouce_example() {
    let mut manager: Box<dyn MouseActions> = Mouse::new();
    let hook_result = manager.hook(Box::new(|e| {
        match e {
            &Press(Left) => println!("tooo"),
            _ => {}
        }
    }));
    match hook_result {
        Ok(id) => {
            println!("Ok !!");
            //assert_eq!(manager.unhook(id), Ok(()));
        }
        // Hooking may require user privileges on some systems
        // e.g. requires super user for Linux
        Err(err) => assert_eq!(Error::PermissionDenied, err),
    }

    loop {}
}

fn start_clicker(manager: &mut Box<dyn MouseActions>) {
    let hook_result = manager.hook(Box::new(|e| {
        match e {
            &Press(Left) => println!("tooo"),
            _ => {}
        }
    }));
    match hook_result {
        Ok(id) => {
            println!("Ok !!");
            //assert_eq!(manager.unhook(id), Ok(()));
        }
        // Hooking may require user privileges on some systems
        // e.g. requires super user for Linux
        Err(err) => assert_eq!(Error::PermissionDenied, err),
    }
}

fn stop_clicker(manager: &mut Box<dyn MouseActions>) {
    manager.unhook_all().expect("Failed to unhook");
}
