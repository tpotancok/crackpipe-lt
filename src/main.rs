use std::{thread::sleep, time::Duration};

use libtorrent_sys::ffi::{self, TorrentStatus};

const MAGNET_LINK: &'static str = "magnet:?xt=urn:btih:78efb442414c713dbab287487855af9b68b7b282&xt=urn:btmh:12208cf2021f0caee2a3dcb336fe5e95227b1e38f74cc5698c4f702f99dfaf1f3c17&dn=calories.ts";

pub fn main() {
    let mut session = ffi::create_session_with_alerts();
    let _ = ffi::add_torrent(
        session.pin_mut(),
        &*ffi::parse_magnet_link(MAGNET_LINK, "./output/"),
    );
    let mut open_torrents = 1u16;
    'outer: loop {
        let alerts = ffi::handle_alerts(session.pin_mut(), &mut open_torrents, "./output/resume/");
        for alert in alerts {
            if alert.status == TorrentStatus::Finished {
                println!("Torrent finished");
                break 'outer;
            } else if alert.status == TorrentStatus::Error {
                println!("Torrent failed");
                break 'outer;
            }
        }
        sleep(Duration::from_millis(500))
    }
    println!("All torrents finished");
}
