use serde::de::{self, Visitor, MapAccess};
use void::Void;

use serde::{Deserialize, Deserializer};

use std::string::String;
use std::fmt;
//use std::net::IpAddr;
use std::marker::PhantomData;

use std::str::FromStr;
use minijinja::value::{StructObject, Value};

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct NmapRun {
	pub scaninfo: ScanInfo,
	#[serde(rename = "host", default = "Vec::new")]
	pub hosts: Vec<Host>,
}
impl NmapRun {
	pub fn empty() -> Self {
		let mut run = NmapRun::default();
		run.hosts.push(Host::empty());
		run
	}
}
impl StructObject for NmapRun {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"scaninfo" => Some(Value::from_struct_object(self.scaninfo.clone())),
			"hosts" => Some(Value::from(self.hosts.clone())),
			_ => panic!("Unknown field {} on {}", name, "NmapRun"),
		}
	}
}
impl From<NmapRun> for Value {
	fn from(val: NmapRun) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct ScanInfo {
	#[serde(rename = "type")]
	pub scantype: String,
	pub protocol: Protocol,
	#[serde(rename = "numservices")]
	pub num_services: i32,
	pub services: String,
}
impl StructObject for ScanInfo {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"scantype" => Some(Value::from(self.scantype.clone())),
			"protocol" => Some(Value::from(self.protocol.clone())),
			"num_services" => Some(Value::from(self.num_services.clone())),
			"serices" => Some(Value::from(self.services.clone())),
			_ => panic!("Unknown field {} on {}", name, "ScanInfo"),
		}
	}
}
impl From<ScanInfo> for Value {
	fn from(val: ScanInfo) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct Host {
	pub status: Option<HostState>,
	pub hostnames: Option<HostNames>,
	pub address: Vec<HostAddress>,
	pub ports: Ports,
	pub os: Option<OperatingSystem>,
}
impl Host {
	pub fn empty() -> Self {
		let mut host = Host::default();
		host.status = Some(HostState::empty());
		host.hostnames = Some(HostNames::empty());
		host.address.push(HostAddress::empty());
		host.ports = Ports::empty();
		host.os = Some(OperatingSystem::empty());
		host
	}
}
impl StructObject for Host {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"status" => Some(Value::from(self.status.clone().unwrap_or(Default::default()))),
			"hostnames" => Some(Value::from(self.hostnames.clone().unwrap_or(Default::default()))),
			"address" => Some(Value::from(self.address.clone())),
			"ipv4" => self.address.iter()
				.filter(|addr| addr.address_type == AddressType::IPV4)
				.map(|val| Value::from(val.addr.clone()))
				.next(),
			"ipv6" => self.address.iter()
				.filter(|addr| addr.address_type == AddressType::IPV6)
				.map(|val| Value::from(val.addr.clone()))
				.next(),
			"mac" => self.address.iter()
				.filter(|addr| addr.address_type == AddressType::MAC)
				.map(|val| Value::from(val.addr.clone()))
				.next(),
			"ports" => Some(Value::from(self.ports.clone())),
			"os" => Some(Value::from(self.os.clone().unwrap_or(Default::default()))),
			_ => panic!("Unknown field {} on {}", name, "Host"),
		}
	}
}
impl From<Host> for Value {
	fn from(val: Host) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum HostStates {
	UP,
	DOWN,
	SKIPPED,
	UNKNOWN,
}
impl Default for HostStates { fn default() -> Self { Self::UNKNOWN } }
impl From<HostStates> for Value {
	fn from(val: HostStates) -> Self {
		match val {
			HostStates::UP => Value::from("up"),
			HostStates::DOWN => Value::from("down"),
			HostStates::SKIPPED => Value::from("skipped"),
			_ => Value::from("unknown"),
		}
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct HostState {
	pub state: HostStates,
	pub reason: String,
	#[serde(rename = "reason_ttl")]
	pub ttl: i16,
}
impl HostState {
	pub fn empty() -> Self { HostState::default() }
}
impl StructObject for HostState {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"state" => Some(Value::from(self.state.clone())),
			"reason" => Some(Value::from(self.reason.clone())),
			"ttl" => Some(Value::from(self.ttl)),
			_ => panic!("Unknown field {} on {}", name, "HostState"),
		}
	}
}
impl From<HostState> for Value {
	fn from(val: HostState) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct HostNames {
	#[serde(default = "Vec::new")]
	pub hostname: Vec<HostName>,
}
impl HostNames {
	pub fn empty() -> Self {
		let mut names = HostNames::default();
		names.hostname.push(HostName::empty());
		names.hostname.push(HostName::empty());
		names
	}
}
impl StructObject for HostNames {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"hostname" => Some(Value::from(self.hostname.clone())),
			_ => panic!("Unknown field {} on {}", name, "HostNames"),
		}
	}
}
impl From<HostNames> for Value {
	fn from(val: HostNames) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct HostName {
	pub name: String,
	#[serde(rename = "type")]
	pub host_type: String,
}
impl HostName { pub fn empty() -> Self { HostName::default() } }
impl StructObject for HostName {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"name" => Some(Value::from(self.name.clone())),
			"host_type" => Some(Value::from(self.host_type.clone())),
			_ => panic!("Unknown field {} on {}", name, "HostName"),
		}
	}
}
impl From<HostName> for Value {
	fn from(val: HostName) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct HostAddress {
	pub addr: String,
	#[serde(rename = "addrtype")]
	pub address_type: AddressType,
	pub vendor: Option<String>,
}
impl HostAddress {
	pub fn empty() -> Self {
		let mut addr = HostAddress::default();
		addr.vendor = Some(String::from(""));
		addr
	}
}
impl StructObject for HostAddress {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"addr" => Some(Value::from(self.addr.clone())),
			"address_type" => Some(Value::from(self.address_type.clone())),
			"vendor" => Some(Value::from(self.vendor.clone().unwrap_or(Default::default()))),
			_ => panic!("Unknown field {} on {}", name, "HostAddress"),
		}
	}
}
impl From<HostAddress> for Value {
	fn from(val: HostAddress) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum AddressType {
	IPV4,
	IPV6,
	MAC,
	UNKNOWN,
}
impl Default for AddressType { fn default() -> Self { Self::UNKNOWN } }
impl From<AddressType> for Value {
	fn from(val: AddressType) -> Self {
		match val {
			AddressType::IPV4 => Value::from("ipv4"),
			AddressType::IPV6 => Value::from("ipv6"),
			AddressType::MAC => Value::from("mac"),
			_ => Value::from("unknown"),
		}
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct Ports {
	#[serde(default = "Vec::new")]
	pub port: Vec<Port>,
}
impl Ports {
	pub fn empty() -> Self {
		let mut ports = Ports::default();
		ports.port.push(Port::empty());
		ports
	}
}
impl StructObject for Ports {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"port" => Some(Value::from(self.port.clone())),
			_ => panic!("Unknown field {} on {}", name, "Ports"),
		}
	}
}
impl From<Ports> for Value {
	fn from(val: Ports) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
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
impl Port {
	pub fn empty() -> Self {
		let mut port = Port::default();
		port.state = PortState::empty();
		port.service = Some(PortService::empty());
		port.script.push(Script::empty());
		port
	}
}
impl StructObject for Port {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"protocol" => Some(Value::from(self.protocol.clone())),
			"port" => Some(Value::from(self.port.clone())),
			"state" => Some(Value::from(self.state.clone())),
			"service" => Some(Value::from(self.service.clone().unwrap_or(Default::default()))),
			"script" => Some(Value::from(self.script.clone())),
			_ => panic!("Unknown field {} on {}", name, "Port"),
		}
	}
}
impl From<Port> for Value {
	fn from(val: Port) -> Self {
		Value::from_struct_object(val)
	}
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


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Protocol {
	IP,
	TCP,
	UDP,
	SCTP,
}
impl Default for Protocol { fn default() -> Self { Self::TCP } }
impl From<Protocol> for Value {
	fn from(val: Protocol) -> Self {
		match val {
			Protocol::IP => Value::from("ip"),
			Protocol::TCP => Value::from("tcp"),
			Protocol::UDP => Value::from("udp"),
			Protocol::SCTP => Value::from("sctp"),
		}
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum State {
	OPEN,
	CLOSED,
	FILTERED,
}
impl Default for State { fn default() -> Self { Self::FILTERED } }
impl From<State> for Value {
	fn from(val: State) -> Self {
		match val {
			State::OPEN => Value::from("open"),
			State::CLOSED => Value::from("closed"),
			State::FILTERED => Value::from("filtered"),
		}
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Tunnel {
	NO,
	SSL,
}
impl Default for Tunnel { fn default() -> Self { Self::NO } }
impl From<Tunnel> for Value {
	fn from(val: Tunnel) -> Self {
		match val {
			Tunnel::NO => Value::from("no_ssl"),
			Tunnel::SSL => Value::from("ssl"),
		}
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct PortState {
	#[serde(default = "State::default")]
	pub state: State,
	pub reason: String,
	#[serde(rename = "reason_ttl")]
	pub ttl: i16,
}
impl PortState { pub fn empty() -> Self { PortState::default() } }
impl StructObject for PortState {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"state" => Some(Value::from(self.state.clone())),
			"reason" => Some(Value::from(self.reason.clone())),
			"ttl" => Some(Value::from(self.ttl.clone())),
			_ => panic!("Unknown field {} on {}", name, "PortState"),
		}
	}
}
impl From<PortState> for Value {
	fn from(val: PortState) -> Self {
		Value::from_struct_object(val)
	}
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

#[derive(Debug, Default, Clone, Deserialize)]
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
impl PortService {
	pub fn empty() -> Self {
		let mut service = PortService::default();
		service.product = Some(String::from(""));
		service.version = Some(String::from(""));
		service.footprint = Some(String::from(""));
		service.cpe.push(Cpe::empty());
		service
	}
}
impl StructObject for PortService {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"service" => Some(Value::from(self.service.clone())),
			"product" => Some(Value::from(self.product.clone().unwrap_or(Default::default()))),
			"version" => Some(Value::from(self.version.clone().unwrap_or(Default::default()))),
			"ssl" => Some(Value::from(self.ssl.clone())),
			"footprint" => Some(Value::from(self.footprint.clone().unwrap_or(Default::default()))),
			"cpe" => Some(Value::from(self.cpe.clone())),
			_ => panic!("Unknown field {} on {}", name, "PortService"),
		}
	}
}
impl From<PortService> for Value {
	fn from(val: PortService) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub (crate) struct Cpe {
	#[serde(rename = "$value")]
	pub value: String,
}
impl Cpe { pub fn empty() -> Self { Cpe::default() } }
impl StructObject for Cpe {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"value" => Some(Value::from(self.value.clone())),
			_ => panic!("Unknown field {} on {}", name, "Cpe"),
		}
	}
}
impl From<Cpe> for Value {
	fn from(val: Cpe) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct Script {
	pub id: String,
	#[serde(rename = "output")]
	pub raw: Option<String>,
	#[serde(rename = "elem", default = "Vec::new")]
	pub elements: Vec<Element>,
	#[serde(default = "Vec::new")]
	pub table: Vec<Table>,
}
impl Script {
	pub fn empty() -> Self {
		let mut script = Script::default();
		script.raw = Some(String::from(""));
		script.elements.push(Element::empty());
		script.table.push(Table::empty());
		script
	}
}
impl StructObject for Script {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"id" => Some(Value::from(self.id.clone())),
			"raw" => Some(Value::from(self.raw.clone().unwrap_or(Default::default()))),
			"elements" => Some(Value::from(self.elements.clone())),
			"table" => Some(Value::from(self.table.clone())),
			_ => panic!("Unknown field {} on {}", name, "Script"),
		}
	}
}
impl From<Script> for Value {
	fn from(val: Script) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct Table {
	pub key: Option<String>,
	#[serde(rename = "table")]
	pub rows: Vec<Row>,
}
impl Table {
	pub fn empty() -> Self {
		let mut table = Table::default();
		table.key = Some(String::from(""));
		table.rows.push(Row::empty());
		table
	}
}
impl StructObject for Table {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"key" => Some(Value::from(self.key.clone().unwrap_or(Default::default()))),
			"rows" => Some(Value::from(self.rows.clone())),

			_ => panic!("Unknown field {} on {}", name, "Table"),
		}
	}
}
impl From<Table> for Value {
	fn from(val: Table) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct Row {
	pub key: Option<String>,
	#[serde(rename = "$value")]
	pub value: Vec<Col>,
}
impl Row {
	pub fn empty() -> Self {
		let mut row = Row::default();
		row.key = Some(String::from(""));
		row.value.push(Col::default());
		row
	}
}
impl StructObject for Row {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"key" => Some(Value::from(self.key.clone().unwrap_or(Default::default()))),
			"value" => Some(Value::from(self.value.clone())),
			_ => panic!("Unknown field {} on {}", name, "Row"),
		}
	}
}
impl From<Row> for Value {
	fn from(val: Row) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Col {
	Elem(Element),
	#[serde(rename = "table", deserialize_with = "deserialize_inner_table")]
	Other(String),
}
impl Default for Col { fn default() -> Self { Self::Other(String::from("Unknown")) } }
fn deserialize_inner_table<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
	de::IgnoredAny::deserialize(deserializer)?;
	Ok(String::from("Tables in tables not yet supported"))
}
impl From<Col> for Value {
	fn from(val: Col) -> Self {
		match val {
			Col::Elem(elem) => Value::from_struct_object(elem),
			Col::Other(msg) => Value::from(msg),
		}
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct Element {
	pub key: Option<String>,
	#[serde(rename = "$value")]
	pub value: String,
}
impl Element {
	pub fn empty() -> Self {
		let mut elem = Element::default();
		elem.key = Some(String::from(""));
		elem
	}
}
impl StructObject for Element {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"key" => Some(Value::from(self.key.clone().unwrap_or(Default::default()))),
			"value" => Some(Value::from(self.value.clone())),
			_ => panic!("Unknown field {} on {}", name, "Element"),
		}
	}
}
impl From<Element> for Value {
	fn from(val: Element) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct OperatingSystem {
	#[serde(rename = "portused", default = "Vec::new")]
	pub ports: Vec<Port>,
	#[serde(rename = "osmatch", default = "Vec::new")]
	pub matches: Vec<OsMatch>,
}
impl OperatingSystem {
	pub fn empty() -> Self {
		let mut os = OperatingSystem::default();
		os.ports.push(Port::empty());
		os.matches.push(OsMatch::empty());
		os
	}
}
impl StructObject for OperatingSystem {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"ports" => Some(Value::from(self.ports.clone())),
			"matches" => Some(Value::from(self.matches.clone())),
			_ => panic!("Unknown field {} on {}", name, "OperatingSystem"),
		}
	}
}
impl From<OperatingSystem> for Value {
	fn from(val: OperatingSystem) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct OsMatch {
	pub name: String,
	pub accuracy: i8,
	#[serde(rename = "osclass", default = "Vec::new")]
	pub classes: Vec<OsClass>,
}
impl OsMatch {
	pub fn empty() -> Self {
		let mut osmatch = OsMatch::default();
		osmatch.classes.push(OsClass::empty());
		osmatch
	}
}
impl StructObject for OsMatch {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"name" => Some(Value::from(self.name.clone())),
			"accuracy" => Some(Value::from(self.accuracy.clone())),
			"classes" => Some(Value::from(self.classes.clone())),
			_ => panic!("Unknown field {} on {}", name, "OsMatch"),
		}
	}
}
impl From<OsMatch> for Value {
	fn from(val: OsMatch) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
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
impl OsClass {
	pub fn empty() -> Self {
		let mut osclass = OsClass::default();
		osclass.os_type = Some(String::from(""));
		osclass.generation = Some(String::from(""));
		osclass.cpe.push(Cpe::empty());
		osclass
	}
}
impl StructObject for OsClass {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"type" => Some(Value::from(self.os_type.clone().unwrap_or(Default::default()))),
			"os_type" => Some(Value::from(self.os_type.clone().unwrap_or(Default::default()))),
			"vendor" => Some(Value::from(self.vendor.clone())),
			"accuracy" => Some(Value::from(self.accuracy.clone())),
			"family" => Some(Value::from(self.family.clone())),
			"generation" => Some(Value::from(self.generation.clone().unwrap_or(Default::default()))),
			"cpe" => Some(Value::from(self.cpe.clone())),
			_ => panic!("Unknown field {} on {}", name, "OsClass"),
		}
	}
}
impl From<OsClass> for Value {
	fn from(val: OsClass) -> Self {
		Value::from_struct_object(val)
	}
}
