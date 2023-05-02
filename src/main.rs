mod watcher;
mod usb_event;

fn main() {
    let usb_watcher = watcher::create_watcher();

    for usb in usb_watcher.upcoming() {
        println!("{:?}", usb);
    }
}
