use std::{fs, thread::sleep, time::Duration};

use crackpipe_lt as lt;
use lt::DownloadStatus;

// Wired album (CC licensed)
const MAGNET_LINK: &'static str = "magnet:?xt=urn:btih:a88fda5954e89178c372716a6a78b8180ed4dad3&dn=The+WIRED+CD+-+Rip.+Sample.+Mash.+Share&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fwired-cd.torrent";
// Big Buck Bunny
const MAGNET_LINK_2: &'static str = "magnet:?xt=urn:btih:dd8255ecdc7ca55fb0bbf81323d87062db1f6d1c&dn=Big+Buck+Bunny&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fbig-buck-bunny.torrent" ;

pub fn main() {
    let _ = fs::remove_dir_all("./output/");

    let mut session = lt::Session::new();

    let torrent = session.add_torrent(MAGNET_LINK, "./output/");
    let torrent2 = session.add_torrent(MAGNET_LINK_2, "./output/2");

    let mut open_torrents = 2u16;

    'outer: loop {
        let alerts = session.handle_alerts(&mut open_torrents, "./output/resume/");
        for alert in alerts {
            if alert.status == DownloadStatus::Finished {
                println!("Torrent finished");
            } else if alert.status == DownloadStatus::Error {
                println!("Torrent failed");
            }
        }

        let progress = torrent.get_status().get_progress();
        let progress2 = torrent2.get_status().get_progress();

        println!("Torrent 1 progress: {}%", progress * 100.);
        println!("Torrent 2 progress: {}%", progress2 * 100.);

        if progress == 1. {
            torrent.save_progress();
        }
        if progress2 == 1. {
            torrent2.save_progress();
        }

        if open_torrents == 0 {
            break 'outer;
        }

        sleep(Duration::from_millis(500))
    }
    println!("All torrents finished, checking...");

    torrent.force_recheck();
    torrent2.force_recheck();

    loop {
        let _ = session.handle_alerts(&mut open_torrents, "./output/resume/");

        let progress = torrent.get_status().get_progress();
        let progress2 = torrent2.get_status().get_progress();

        println!("Torrent 1 check progress: {}%", progress * 100.);
        println!("Torrent 2 check progress: {}%", progress2 * 100.);

        if progress == 1. && progress2 == 1. {
            println!("All torrents checked");
            break;
        }

        sleep(Duration::from_millis(500))
    }
}
