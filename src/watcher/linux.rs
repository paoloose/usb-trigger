use crate::watcher::{Watcher, EventsIter};
#[cfg(target_os = "linux")]
extern crate udev;

use crate::usb_event::USBEvent;

pub struct LinuxEventsIter {
    socket: udev::MonitorSocket,
    poll: mio::Poll,
    events: mio::Events,
}

impl EventsIter for LinuxEventsIter { }

pub struct LinuxWatcher {
    events_iter: LinuxEventsIter,
}
const UDEV_USB_SUBSYSTEM: &str = "usb";
const UDEV_USB_DEVTYPE:   &str = "usb_device";

impl Watcher for LinuxWatcher {
    fn new() -> Result<Self, std::io::Error> {
        // Initialize the udev monitor and get the socket
        let socket = udev::MonitorBuilder::new()?
            .match_subsystem_devtype(UDEV_USB_SUBSYSTEM, UDEV_USB_DEVTYPE)?
            .listen()?;

        let events_iter = LinuxEventsIter::new(
            socket,
            mio::Poll::new()?,
            mio::Events::with_capacity(1024)
        )?;

        Ok(LinuxWatcher { events_iter })
    }

    fn upcoming(self: Box<Self>) -> Box<dyn EventsIter> {
        Box::new(self.events_iter)
    }
}

impl LinuxEventsIter {
    pub fn new(
        socket: udev::MonitorSocket,
        poll: mio::Poll,
        events: mio::Events
    ) -> Result<Self, std::io::Error> {
        let mut events_iter = LinuxEventsIter {
            socket,
            poll,
            events,
        };
        events_iter.register_socket()?;

        Ok(events_iter)
    }

    fn register_socket(&mut self) -> Result<(), std::io::Error> {
        self.poll.registry().register(
            &mut self.socket,
            mio::Token(0),
            mio::Interest::READABLE
        )?;

        Ok(())
    }
}

// Iterator implementation for LinuxEventsIter
// It polls the udev socket for events and returns them as USBEvent
impl Iterator for LinuxEventsIter {
    type Item = USBEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.poll.poll(&mut self.events, None).unwrap();

        for event in self.events.iter() {
            if event.token() == mio::Token(0) && event.is_readable() {
                // Socket is ready to be read
                match self.socket.iter().next() {
                    Some(uevent) => {
                        return Some(USBEvent::from_uevent(uevent));
                    },
                    None => {
                        return None;
                    }
                }
            }
        };

        return None;
    }
}
