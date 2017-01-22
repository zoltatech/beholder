//! Copyright 2017 Robert L Snyder, Ithaca, NY <zoltatech@gmail.com> <robscary@gmail.com>
//! 
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//! 
//!        http://www.apache.org/licenses/LICENSE-2.0

//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

extern crate uuid;

use std::string;

pub struct Endpoint {
	pub endpoint_id:		uuid::Uuid,
	pub service_name:		string::String
}

pub struct Message {
	pub message_id: 		uuid::Uuid,
	pub sender: 			Endpoint,
	pub recipient_list:		Vec<Endpoint>
}

impl Endpoint {
	pub fn new(service_name: string::String) -> Endpoint {
		Endpoint {
			endpoint_id: 	uuid::Uuid::new_v4(),
			service_name: 	service_name
		}
	}
}

impl<'a> Message {
	pub fn new(sender: Endpoint, recipients: Vec<Endpoint>) -> Message {
		Message {
			message_id: 		uuid::Uuid::new_v4(),
			sender:				sender,
			recipient_list:		recipients
		}
	}
}