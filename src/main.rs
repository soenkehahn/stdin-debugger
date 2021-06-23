use nix::poll::{poll, PollFd, PollFlags};
use std::{
    io::{self, Read},
    thread,
    time::Duration,
};

fn log(message: &str) {
    eprintln!("[stdin-debugger]: {}", message);
}

fn stdin_is_closed() -> bool {
    let mut poll_fds = [PollFd::new(0, PollFlags::all())];
    poll(&mut poll_fds, 0).unwrap();
    if let Some(events) = poll_fds[0].revents() {
        events.contains(PollFlags::POLLHUP)
    } else {
        false
    }
}

fn main() {
    loop {
        let mut buffer = [0; 1024];
        let length = io::stdin().read(&mut buffer).unwrap();
        match length {
            0 => {
                log("received 0 bytes");
            }
            length => {
                let snippet = String::from_utf8_lossy(&buffer[..length]);
                log(&format!("{} bytes read: {:?}", length, snippet));
            }
        }
        if stdin_is_closed() {
            log("stdin is closed");
            break;
        }
        thread::sleep(Duration::from_secs_f32(0.1));
    }
}
