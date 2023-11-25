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

#[cxx::bridge(namespace = "libtorrent")]
pub mod ffi {
    pub enum DownloadStatus {
        Running,
        Finished,
        Error,
    }

    pub struct StatusAlert<'a> {
        status: DownloadStatus,
        torrent: &'a TorrentHandle,
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

        pub fn handle_alerts<'a>(
            ses: Pin<&'a mut Session>,
            open_torrents: &mut u16,
            save_data_path: &str,
        ) -> Vec<StatusAlert<'a>>;

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
