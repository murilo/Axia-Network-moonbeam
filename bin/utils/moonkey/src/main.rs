// Copyright 2019-2021 AxiaStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Generate an Ethereum account.

use cli_opt::account_key::GenerateAccountKey;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author = "AxiaStake")]
struct Opt {
	#[structopt(flatten)]
	cmd: GenerateAccountKey,
}

impl Opt {
	fn run(&self) {
		self.cmd.run()
	}
}

fn main() {
	// Parses the options
	let cmd = Opt::from_args();
	cmd.run();
}