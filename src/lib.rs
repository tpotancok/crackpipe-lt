// Copyright (c) 2022 Nicolas Chevalier
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::pin::Pin;

#[cxx::bridge(namespace = "libtorrent")]
pub mod ffi {
    pub enum DownloadStatus {
        Running,
        Finished,
        Error,
    }

    pub struct StatusAlert {
        status: DownloadStatus,
        torrent: UniquePtr<TorrentHandle>,
        pub resume_data_saved: bool,
    }

    unsafe extern "C++" {
        include!("src/lt.h");

        #[rust_name = "Session"]
        type session;

        #[rust_name = "TorrentHandle"]
        type torrent_handle;

        #[rust_name = "AddTorrentParams"]
        type add_torrent_params;

        #[rust_name = "TorrentStatus"]
        type torrent_status;

        pub fn create_session_with_alerts() -> UniquePtr<Session>;

        pub fn handle_alerts(ses: Pin<&mut Session>, save_data_path: &str) -> Vec<StatusAlert>;

        pub fn handle_eq(lhs: &TorrentHandle, rhs: &TorrentHandle) -> bool;

        pub fn parse_magnet_link(link: &str, save_path: &str) -> UniquePtr<AddTorrentParams>;

        pub fn add_torrent(
            ses: Pin<&mut Session>,
            params: &AddTorrentParams,
        ) -> UniquePtr<TorrentHandle>;

        pub fn resume_torrent(data: &str) -> UniquePtr<AddTorrentParams>;

        pub fn save_torrent(handle: &TorrentHandle);

        pub fn force_recheck(handle: &TorrentHandle);

        pub fn get_torrent_status(handle: &TorrentHandle) -> UniquePtr<TorrentStatus>;

        pub fn status_get_progress(status: &TorrentStatus) -> f32;
    }
}

/// libtorrent uses mutexes internally to guarantee thread safety
unsafe impl Send for ffi::Session {}
unsafe impl Sync for ffi::Session {}

unsafe impl Send for ffi::TorrentHandle {}
unsafe impl Sync for ffi::TorrentHandle {}

pub use ffi::DownloadStatus;

pub struct TorrentStatus {
    status: cxx::UniquePtr<ffi::TorrentStatus>,
}

impl TorrentStatus {
    pub(crate) fn new(ptr: cxx::UniquePtr<ffi::TorrentStatus>) -> Self {
        TorrentStatus { status: ptr }
    }

    pub fn get_progress(&self) -> f32 {
        ffi::status_get_progress(&self.status)
    }
}

pub struct Torrent {
    torrent: cxx::UniquePtr<ffi::TorrentHandle>,
}

impl Torrent {
    pub(crate) fn new(session: Pin<&mut ffi::Session>, params: &ffi::AddTorrentParams) -> Self {
        Torrent {
            torrent: ffi::add_torrent(session, params),
        }
    }

    pub fn save_progress(&self) {
        ffi::save_torrent(&self.torrent)
    }

    pub fn force_recheck(&self) {
        ffi::force_recheck(&self.torrent)
    }

    pub fn get_status(&self) -> TorrentStatus {
        TorrentStatus::new(ffi::get_torrent_status(&self.torrent))
    }
}

impl From<cxx::UniquePtr<ffi::TorrentHandle>> for Torrent {
    fn from(value: cxx::UniquePtr<ffi::TorrentHandle>) -> Self {
        Self { torrent: value }
    }
}

impl PartialEq for Torrent {
    fn eq(&self, other: &Self) -> bool {
        ffi::handle_eq(&self.torrent, &other.torrent)
    }
}

pub struct Session<'a> {
    session: cxx::UniquePtr<ffi::Session>,
    save_data_path: &'a str,
}

impl Session<'_> {
    pub fn new<'a>(save_data_path: &'a str) -> Session<'a> {
        Session {
            session: ffi::create_session_with_alerts(),
            save_data_path,
        }
    }

    pub fn handle_alerts(&mut self) -> Vec<StatusAlert> {
        ffi::handle_alerts(self.session.pin_mut(), self.save_data_path)
            .into_iter()
            .map(|v| v.into())
            .collect()
    }

    pub fn add_torrent(&mut self, magnet_link: &str, save_path: &str) -> Torrent {
        Torrent::new(
            self.session.pin_mut(),
            &ffi::parse_magnet_link(magnet_link, save_path),
        )
    }
}

pub struct StatusAlert {
    pub status: DownloadStatus,
    pub torrent: Torrent,
    pub resume_data_saved: bool,
}

impl StatusAlert {
    pub fn new(
        torrent: Torrent,
        download_status: DownloadStatus,
        resume_data_saved: bool,
    ) -> StatusAlert {
        Self {
            torrent,
            status: download_status,
            resume_data_saved,
        }
    }

    pub fn apply(&mut self, other: &StatusAlert) {
        if self.torrent != other.torrent {
            return;
        }

        self.status = {
            if self.status == DownloadStatus::Running {
                other.status
            } else {
                self.status
            }
        };

        self.resume_data_saved = self.resume_data_saved || other.resume_data_saved;
    }
}

impl From<ffi::StatusAlert> for StatusAlert {
    fn from(value: ffi::StatusAlert) -> Self {
        Self {
            status: value.status,
            torrent: value.torrent.into(),
            resume_data_saved: value.resume_data_saved,
        }
    }
}
