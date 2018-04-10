// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod command;
mod config;
mod contact;
mod event;
mod extension;
mod kademlia;
mod message;
mod node_id;
mod routing_table;


pub use self::config::Config;
pub use self::extension::Extension;
use self::node_id::NodeId;


const ALPHA: u8 = 3;
const B: usize = 64 * 8;
const K: u8 = 16;
const T_REFRESH: u32 = 60_000;