use serde::de::{self, Visitor, MapAccess};
use void::Void;

use serde::{Deserialize, Deserializer};
use std::string::String;
use std::fmt;
use std::net::IpAddr;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub(crate) struct NampRun {
	pub scaninfo: ScanInfo,
	#[serde(rename = "host", default = "Vec::new")]
	pub hosts: Vec<Host>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ScanInfo {
	#[serde(rename = "type")]
	pub scantype: String,
	pub protocol: Protocol,
	#[serde(rename = "numservices")]
	pub num_services: i32,
	pub services: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Host {
	pub status: Option<HostState>,
	pub hostnames: Option<HostNames>,
	pub address: Option<HostAddress>,
	pub ports: Ports,
	pub os: Option<OperatingSystem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum HostStates {
	UP,
	DOWN,
	UNKNOWN,
	SKIPPED,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HostState {
	pub state: HostStates,
	pub reason: String,
	#[serde(rename = "reason_ttl")]
	pub ttl: i16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HostNames {
	#[serde(default = "Vec::new")]
	pub hostname: Vec<HostName>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HostName {
	pub name: String,
	#[serde(rename = "type")]
	pub host_type: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HostAddress {
	pub addr: IpAddr,
	#[serde(rename = "addrtype")]
	pub address_type: String,
	pub vendor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Ports {
	#[serde(default = "Vec::new")]
	pub port: Vec<Port>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Port {
	#[serde(alias = "proto", default = "Protocol::default")]
	pub protocol: Protocol,
	#[serde(rename = "portid")]
	pub port: i32,
	#[serde(deserialize_with = "state_or_struct")]
	pub state: PortState,
	pub service: Option<PortService>,
	#[serde(default = "Vec::new")]
	pub script: Vec<Script>,
}

/// Tag <port> has an element <state state="" ...>
/// Tag <os><portused> has an attribute state=""
/// This function can deserialize both structures into a PortState struct
fn state_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	T: Deserialize<'de> + FromStr<Err = Void>,
	D: Deserializer<'de>,
{
	// The serde-Deserialization uses a Visitor to either use «T» if a FromStr trait is available
	// or the «D» if there is no such trait.
	struct StringOrStruct<T>(PhantomData<fn() -> T>);

	impl<'de, T> Visitor<'de> for StringOrStruct<T>
		where T: Deserialize<'de> + FromStr<Err = Void>,
	{
		type Value = T;
		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("String or PortState")
		}

		fn visit_str<E>(self, value: &str) -> Result<T, E> where E: de::Error {
			Ok(FromStr::from_str(value).unwrap())
		}

		fn visit_map<M>(self, value: M) -> Result<T, M::Error> where M: MapAccess<'de> {
			Deserialize::deserialize(de::value::MapAccessDeserializer::new(value))
		}
	}
	deserializer.deserialize_any(StringOrStruct(PhantomData))
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Protocol {
	IP,
	TCP,
	UDP,
	SCTP,
}
impl Protocol { fn default() -> Protocol { Protocol::TCP } }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum State {
	OPEN,
	FILTERED,
	CLOSED,
}
impl State { fn default() -> State { State::FILTERED } }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Tunnel {
	NO,
	SSL,
}
impl Tunnel { fn default() -> Tunnel { Tunnel::NO } }

#[derive(Debug, Deserialize)]
pub(crate) struct PortState {
	#[serde(default = "State::default")]
	pub state: State,
	pub reason: String,
	#[serde(rename = "reason_ttl")]
	pub ttl: i16,
}
impl FromStr for PortState {
	type Err = Void;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(PortState {
			state: match s {
				"open" => State::OPEN,
				"filtered" => State::FILTERED,
				_ => State::CLOSED
			},
			reason: String::from(""),
			ttl: 0,
		})
	}
}

#[derive(Debug, Deserialize)]
pub(crate) struct PortService {
	#[serde(rename = "name")]
	pub service: String,
	pub product: Option<String>,
	pub version: Option<String>,
	#[serde(rename = "tunnel", default = "Tunnel::default")]
	pub ssl: Tunnel,
	#[serde(rename = "servicefp")]
	pub footprint: Option<String>,
	#[serde(default = "Vec::new")]
	pub cpe: Vec<Cpe>,
}

#[derive(Debug, Deserialize)]
pub (crate) struct Cpe {
	#[serde(rename = "$value")]
	pub value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Script {
	pub id: String,
	#[serde(rename = "output")]
	pub raw: Option<String>,
	#[serde(rename = "elem", default = "Vec::new")]
	pub elements: Vec<Element>,
	#[serde(default = "Vec::new")]
	pub table: Vec<Table>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Table {
	pub key: Option<String>,
	#[serde(rename = "table")]
	pub rows: Vec<Row>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Row {
	pub key: Option<String>,
	#[serde(rename = "$value")]
	pub value: Vec<Col>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Col {
	Elem(Element),
	#[serde(rename = "table", deserialize_with = "deserialize_inner_table")]
	Other(String),
}
fn deserialize_inner_table<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
	de::IgnoredAny::deserialize(deserializer)?;
	Ok(String::from("Tables in tables not yet supported"))
}

#[derive(Debug, Deserialize)]
pub(crate) struct Element {
	pub key: Option<String>,
	#[serde(rename = "$value")]
	pub value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OperatingSystem {
	#[serde(rename = "portused", default = "Vec::new")]
	pub ports: Vec<Port>,
	#[serde(rename = "osmatch", default = "Vec::new")]
	pub matches: Vec<OsMatch>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OsMatch {
	pub name: String,
	pub accuracy: i8,
	#[serde(rename = "osclass", default = "Vec::new")]
	pub classes: Vec<OsClass>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OsClass {
	#[serde(rename = "type")]
	pub os_type: Option<String>,
	pub vendor: String,
	pub accuracy: i8,
	#[serde(rename = "osfamily")]
	pub family: String,
	#[serde(rename = "osgen")]
	pub generation: Option<String>,
	#[serde(default = "Vec::new")]
	pub cpe: Vec<Cpe>,
}





