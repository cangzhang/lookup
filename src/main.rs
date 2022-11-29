use mki::{register_hotkey, Keyboard};
use winput::{Action, Input, Vk};

fn main() {
    register_hotkey(&[Keyboard::LeftControl, Keyboard::B], move || {
        if let Ok(prev) = cli_clipboard::get_contents() {
            println!("[before] {prev}");

            let inputs = [
                Input::from_vk(Vk::LeftControl, Action::Press),
                Input::from_vk(Vk::C, Action::Press),
            ];
            let _ = winput::send_inputs(&inputs);
            std::thread::sleep(std::time::Duration::from_secs(1));

            if let Ok(q) = cli_clipboard::get_contents() {
                println!("[current] {:?}", q);
                let _r = cli_clipboard::set_contents(prev.to_owned());
            }

            println!("[final] all done");
        }
    });

    loop {}
}
