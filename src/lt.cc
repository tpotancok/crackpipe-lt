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

#include "src/lt.h"
#include "libtorrent-sys/src/lib.rs.h"
#include <libtorrent/alert_types.hpp>

namespace libtorrent
{
	std::unique_ptr<lt::session> create_session_with_alerts()
	{
		lt::settings_pack p;
		p.set_int(lt::settings_pack::alert_mask, lt::alert_category::status | lt::alert_category::error);
		lt::session ses(p);
		return std::make_unique<lt::session>(std::move(ses));
	}

	rust::Vec<GetStatusResult> get_status_updates(lt::session &ses)
	{
		std::vector<lt::alert *> alerts;
		rust::Vec<GetStatusResult> results;
		ses.pop_alerts(&alerts);

		for (lt::alert const *a : alerts)
		{
			if (lt::alert_cast<lt::torrent_finished_alert>(a))
			{
				GetStatusResult entry = {
					TorrentStatus::Finished,
					lt::alert_cast<lt::torrent_finished_alert>(a)->handle};
				results.push_back(entry);
			}
			if (lt::alert_cast<lt::torrent_error_alert>(a))
			{
				GetStatusResult entry = {
					TorrentStatus::Error,
					lt::alert_cast<lt::torrent_error_alert>(a)->handle};
				results.push_back(entry);
			}
		}
		return results;
	}

	bool handle_eq(const lt::torrent_handle &lhs, const lt::torrent_handle &rhs)
	{
		return lhs == rhs;
	}

	std::unique_ptr<lt::add_torrent_params> parse_magnet_link(rust::Str link, rust::Str save_path)
	{
		lt::add_torrent_params atp = lt::parse_magnet_uri(std::string(link));
		atp.save_path = std::string(save_path);
		return std::make_unique<lt::add_torrent_params>(std::move(atp));
	}

	std::unique_ptr<lt::torrent_handle> add_torrent(lt::session &ses, const lt::add_torrent_params &params)
	{
		return std::make_unique<lt::torrent_handle>(std::move(ses.add_torrent(params)));
	}
} // namespace libtorrent
