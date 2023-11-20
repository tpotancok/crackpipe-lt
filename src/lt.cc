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

	TorrentStatus get_status(lt::session &ses)
	{
		std::vector<lt::alert *> alerts;
		ses.pop_alerts(&alerts);

		for (lt::alert const *a : alerts)
		{
			if (lt::alert_cast<lt::torrent_finished_alert>(a))
			{
				return TorrentStatus::Finished;
			}
			if (lt::alert_cast<lt::torrent_error_alert>(a))
			{
				return TorrentStatus::Error;
			}
		}
		return TorrentStatus::Running;
	}
} // namespace libtorrent
