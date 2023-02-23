use serde::de::{self, Visitor, MapAccess};
use void::Void;

use serde::{Deserialize, Deserializer};

use std::string::String;
use std::fmt;
use std::net::IpAddr;
use std::marker::PhantomData;

use std::str::FromStr;
use minijinja::value::Object;

#[derive(Debug, Deserialize)]
pub(crate) struct NampRun {
	pub scaninfo: ScanInfo,
	#[serde(rename = "host", default = "Vec::new")]
	pub hosts: Vec<Host>,
}
impl fmt::Display for NampRun { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for NampRun {}

#[derive(Debug, Deserialize)]
pub(crate) struct ScanInfo {
	#[serde(rename = "type")]
	pub scantype: String,
	pub protocol: Protocol,
	#[serde(rename = "numservices")]
	pub num_services: i32,
	pub services: String,
}
impl fmt::Display for ScanInfo { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for ScanInfo {}

#[derive(Debug, Deserialize)]
pub(crate) struct Host {
	pub status: Option<HostState>,
	pub hostnames: Option<HostNames>,
	pub address: Option<HostAddress>,
	pub ports: Ports,
	pub os: Option<OperatingSystem>,
}
impl fmt::Display for Host { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Host {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum HostStates {
	UP,
	DOWN,
	UNKNOWN,
	SKIPPED,
}
impl fmt::Display for HostStates { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for HostStates {}

#[derive(Debug, Deserialize)]
pub(crate) struct HostState {
	pub state: HostStates,
	pub reason: String,
	#[serde(rename = "reason_ttl")]
	pub ttl: i16,
}
impl fmt::Display for HostState{ fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for HostState {}

#[derive(Debug, Deserialize)]
pub(crate) struct HostNames {
	#[serde(default = "Vec::new")]
	pub hostname: Vec<HostName>,
}
impl fmt::Display for HostNames { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for HostNames {}

#[derive(Debug, Deserialize)]
pub(crate) struct HostName {
	pub name: String,
	#[serde(rename = "type")]
	pub host_type: String,
}
impl fmt::Display for HostName { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for HostName {}

#[derive(Debug, Deserialize)]
pub(crate) struct HostAddress {
	pub addr: IpAddr,
	#[serde(rename = "addrtype")]
	pub address_type: String,
	pub vendor: Option<String>,
}
impl fmt::Display for HostAddress { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for HostAddress {}

#[derive(Debug, Deserialize)]
pub(crate) struct Ports {
	#[serde(default = "Vec::new")]
	pub port: Vec<Port>,
}
impl fmt::Display for Ports { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Ports {}

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
impl fmt::Display for Port { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Port {}

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
impl fmt::Display for Protocol { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Protocol {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum State {
	OPEN,
	FILTERED,
	CLOSED,
}
impl State { fn default() -> State { State::FILTERED } }
impl fmt::Display for State { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for State {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Tunnel {
	NO,
	SSL,
}
impl Tunnel { fn default() -> Tunnel { Tunnel::NO } }
impl fmt::Display for Tunnel { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Tunnel {}

#[derive(Debug, Deserialize)]
pub(crate) struct PortState {
	#[serde(default = "State::default")]
	pub state: State,
	pub reason: String,
	#[serde(rename = "reason_ttl")]
	pub ttl: i16,
}
impl fmt::Display for PortState { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for PortState {}
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
impl fmt::Display for PortService { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for PortService {}

#[derive(Debug, Deserialize)]
pub (crate) struct Cpe {
	#[serde(rename = "$value")]
	pub value: String,
}
impl fmt::Display for Cpe { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Cpe {}

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
impl fmt::Display for Script { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Script {}

#[derive(Debug, Deserialize)]
pub(crate) struct Table {
	pub key: Option<String>,
	#[serde(rename = "table")]
	pub rows: Vec<Row>,
}
impl fmt::Display for Table { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Table {}

#[derive(Debug, Deserialize)]
pub(crate) struct Row {
	pub key: Option<String>,
	#[serde(rename = "$value")]
	pub value: Vec<Col>,
}
impl fmt::Display for Row { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Row {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Col {
	Elem(Element),
	#[serde(rename = "table", deserialize_with = "deserialize_inner_table")]
	Other(String),
}
impl fmt::Display for Col { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Col {}
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
impl fmt::Display for Element { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for Element {}

#[derive(Debug, Deserialize)]
pub(crate) struct OperatingSystem {
	#[serde(rename = "portused", default = "Vec::new")]
	pub ports: Vec<Port>,
	#[serde(rename = "osmatch", default = "Vec::new")]
	pub matches: Vec<OsMatch>,
}
impl fmt::Display for OperatingSystem { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for OperatingSystem {}

#[derive(Debug, Deserialize)]
pub(crate) struct OsMatch {
	pub name: String,
	pub accuracy: i8,
	#[serde(rename = "osclass", default = "Vec::new")]
	pub classes: Vec<OsClass>,
}
impl fmt::Display for OsMatch { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for OsMatch {}

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
impl fmt::Display for OsClass { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Debug::fmt(self, f) } }
impl Object for OsClass {}

