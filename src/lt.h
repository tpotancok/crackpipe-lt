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

#pragma once

#include "rust/cxx.h"

#include <libtorrent/version.hpp>
#include <libtorrent/create_torrent.hpp>
#include <libtorrent/session.hpp>
#include <libtorrent/magnet_uri.hpp>

#include <memory>

namespace libtorrent
{
    enum class TorrentStatus : uint8_t;
    struct GetStatusResult;

    std::unique_ptr<lt::session> create_session_with_alerts();
    rust::Vec<GetStatusResult> get_status_updates(lt::session &ses);
    bool handle_eq(const lt::torrent_handle &lhs, const lt::torrent_handle &rhs);
    std::unique_ptr<lt::add_torrent_params> parse_magnet_link(rust::Str link, rust::Str save_path);
    std::unique_ptr<lt::torrent_handle> add_torrent(lt::session &ses, const lt::add_torrent_params &params);
}
