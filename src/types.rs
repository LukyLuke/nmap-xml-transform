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
	#[serde(rename = "args")]
	pub command: String,
	#[serde(rename = "startstr")]
	pub start_time: String,
	pub runstats: RunStats,
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
			"runstats" => Some(Value::from(self.runstats.clone())),
			"command" => Some(Value::from(self.command.clone())),
			"start_time" => Some(Value::from(self.start_time.clone())),
			"finish_time" => Some(Value::from(self.runstats.finished.finish_time.clone())),
			"elapsed_time" => Some(Value::from(self.runstats.finished.elapsed)),
			"status" => Some(Value::from(self.runstats.finished.status.clone())),
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
pub(crate) struct RunStats {
	pub finished: Finished,
	pub hosts: HostStats,
}
impl StructObject for RunStats {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"finished" => Some(Value::from_struct_object(self.finished.clone())),
			"hosts" => Some(Value::from_struct_object(self.hosts.clone())),
			"hosts_total" => Some(Value::from(self.hosts.total)),
			"hosts_online" => Some(Value::from(self.hosts.up)),
			"hosts_offline" => Some(Value::from(self.hosts.down)),
			_ => panic!("Unknown field {} on {}", name, "RunStats"),
		}
	}
}
impl From<RunStats> for Value {
	fn from(val: RunStats) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct Finished {
	#[serde(rename = "timestr")]
	pub finish_time: String,
	pub elapsed: f64,
	#[serde(rename = "exit")]
	pub status: String,
}
impl StructObject for Finished {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"finish_time" => Some(Value::from(self.finish_time.clone())),
			"elapsed" => Some(Value::from(self.elapsed)),
			"status" => Some(Value::from(self.status.clone())),
			_ => panic!("Unknown field {} on {}", name, "Finished"),
		}
	}
}
impl From<Finished> for Value {
	fn from(val: Finished) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct HostStats {
	pub up: i64,
	pub down: i64,
	pub total: i64,
}
impl StructObject for HostStats {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"up" => Some(Value::from(self.up)),
			"down" => Some(Value::from(self.down)),
			"total" => Some(Value::from(self.total)),
			_ => panic!("Unknown field {} on {}", name, "RunStats"),
		}
	}
}
impl From<HostStats> for Value {
	fn from(val: HostStats) -> Self {
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
			"num_services" => Some(Value::from(self.num_services)),
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
		let mut host = Host {
			status: Some(HostState::default()),
			hostnames: Some(HostNames::empty()),
			ports: Ports::empty(),
			os: Some(OperatingSystem::empty()),
			..Default::default()
		};
		host.address.push(HostAddress::default());
		host
	}
}
impl StructObject for Host {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"status" => Some(Value::from(self.status.clone().unwrap_or_default())),
			"hostnames" => Some(Value::from(self.hostnames.clone().unwrap_or_default().hostname)),
			"address" => Some(Value::from(self.address.clone())),
			"ipv4" => self.address.iter()
				.filter(|addr| addr.address_type == AddressType::Ipv4)
				.map(|val| Value::from(val.addr.clone()))
				.next(),
			"ipv6" => self.address.iter()
				.filter(|addr| addr.address_type == AddressType::Ipv6)
				.map(|val| Value::from(val.addr.clone()))
				.next(),
			"mac" => self.address.iter()
				.filter(|addr| addr.address_type == AddressType::Mac)
				.map(|val| Value::from(val.addr.clone()))
				.next(),
			"ports" => Some(Value::from(self.ports.clone().port)),
			"os" => Some(Value::from(self.os.clone().unwrap_or_default())),
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
	Up,
	Down,
	Skipped,
	Unknown,
}
impl Default for HostStates { fn default() -> Self { Self::Unknown } }
impl From<HostStates> for Value {
	fn from(val: HostStates) -> Self {
		match val {
			HostStates::Up => Value::from("up"),
			HostStates::Down => Value::from("down"),
			HostStates::Skipped => Value::from("skipped"),
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
impl StructObject for HostAddress {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"addr" => Some(Value::from(self.addr.clone())),
			"address_type" => Some(Value::from(self.address_type.clone())),
			"vendor" => Some(Value::from(self.vendor.clone().unwrap_or_default())),
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
	Ipv4,
	Ipv6,
	Mac,
	Unknown,
}
impl Default for AddressType { fn default() -> Self { Self::Unknown } }
impl From<AddressType> for Value {
	fn from(val: AddressType) -> Self {
		match val {
			AddressType::Ipv4 => Value::from("ipv4"),
			AddressType::Ipv6 => Value::from("ipv6"),
			AddressType::Mac => Value::from("mac"),
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
		let mut port = Port {
			service: Some(PortService::empty()),
			..Default::default()
		};
		port.script.push(Script::empty());
		port
	}
}
impl StructObject for Port {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"protocol" => Some(Value::from(self.protocol.clone())),
			"port" => Some(Value::from(self.port)),
			"state" => Some(Value::from(self.state.clone())),
			"service" => Some(Value::from(self.service.clone().unwrap_or_default())),
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
#[allow(clippy::upper_case_acronyms)]
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
	Open,
	Closed,
	Filtered,
}
impl Default for State { fn default() -> Self { Self::Filtered } }
impl From<State> for Value {
	fn from(val: State) -> Self {
		match val {
			State::Open => Value::from("open"),
			State::Closed => Value::from("closed"),
			State::Filtered => Value::from("filtered"),
		}
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(clippy::upper_case_acronyms)]
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
impl StructObject for PortState {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"state" => Some(Value::from(self.state.clone())),
			"reason" => Some(Value::from(self.reason.clone())),
			"ttl" => Some(Value::from(self.ttl)),
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
				"open" => State::Open,
				"filtered" => State::Filtered,
				_ => State::Closed
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
		service.cpe.push(Cpe::default());
		service
	}
}
impl StructObject for PortService {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"service" => Some(Value::from(self.service.clone())),
			"product" => Some(Value::from(self.product.clone().unwrap_or_default())),
			"version" => Some(Value::from(self.version.clone().unwrap_or_default())),
			"ssl" => Some(Value::from(self.ssl.clone())),
			"footprint" => Some(Value::from(self.footprint.clone().unwrap_or_default())),
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
		script.elements.push(Element::default());
		script.table.push(Table::empty());
		script
	}
}
impl StructObject for Script {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"id" => Some(Value::from(self.id.clone())),
			"raw" => Some(Value::from(self.raw.clone().unwrap_or_default())),
			"items" => {
				let mut items: Vec<ScriptItem> = if self.elements.is_empty() { vec![] } else { vec![ScriptItem { key: None, items: self.elements.clone() }]};
				self.table.iter()
					.for_each(|table| {
						table.rows.iter().for_each(|row| {
							items.push(ScriptItem {
								key: table.key.clone(),
								items: row.value.iter().map(|col| match col {
									Col::Elem(elem) => elem.clone(),
									Col::Table(tbl) => tbl.clone(),
								}).collect(),
							});
						});
				});
				Some(Value::from(items))
			},
			_ => panic!("Unknown field {} on {}", name, "Script"),
		}
	}
}
impl From<Script> for Value {
	fn from(val: Script) -> Self {
		Value::from_struct_object(val)
	}
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ScriptItem {
	pub key: Option<String>,
	pub items: Vec<Element>,
}
impl StructObject for ScriptItem {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"key" => Some(Value::from(self.key.clone().unwrap_or_default())),
			"items" => Some(Value::from(self.items.clone())),
			_ => self.items.iter()
				.find(|item| item.key.clone().unwrap_or_default() == name)
				.map(|elem| Value::from(elem.value.clone())),
		}
	}
}
impl From<ScriptItem> for Value {
	fn from(val: ScriptItem) -> Self {
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
		table.rows.push(Row::empty());
		table
	}
}
impl StructObject for Table {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"key" => Some(Value::from(self.key.clone().unwrap_or_default())),
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
		row.value.push(Col::default());
		row
	}
}
impl StructObject for Row {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"key" => Some(Value::from(self.key.clone().unwrap_or_default())),
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


/// Special handling of script table results
/// See InnerTableVisitor for better description
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Col {
	Elem(Element),
	#[serde(deserialize_with = "deserialize_inner_table")]
	Table(Element)
}
impl Default for Col { fn default() -> Self { Self::Elem( Default::default() ) } }
fn deserialize_inner_table<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Element, D::Error> {
	deserializer.deserialize_map(InnerTableVisitor)
}
impl From<Col> for Value {
	fn from(val: Col) -> Self {
		match val {
			Col::Elem(elem) => Value::from_struct_object(elem),
			Col::Table(elem) => Value::from_struct_object(elem)
		}
	}
}

/// Special handling of script table results
///
/// Normal table results are like:
///
/// ```xml
/// <table>
///   <table>
///     <elem key="">value</elem>
///     <elem key="">value</elem>
///   <table>
/// <table>
/// ```
///
/// Although there are scripts producing such an output, which is totally valid due to the schema:
///
/// ```xml
/// <table>
///   <table>
///     <elem key="">value</elem>
///     <table key="">
///       <elem>value</elem>
///       <elem>value</elem>
///     </table>
///     <elem key="">value</elem>
///   <table>
/// <table>
/// ```
/// The enum Col::Elem will parse the first variant,
/// while the Col::Table is for the second nested variant
///
struct InnerTableVisitor;
impl<'de> Visitor<'de> for InnerTableVisitor {
	type Value = Element;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("XML like <table key='key'><elem>value</elem><elem>value</elem></table> expected")
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
	where M: MapAccess<'de> {
		let mut key_attr = String::from("");
		let mut res = String::from("");
		while let Some((key, value)) = access.next_entry::<String, String>()? {
			match key.as_str() {
				"elem" => {
					if !res.is_empty() { res.push('.'); }
					res.push_str(&value)
				},
				_ => key_attr = value,
			}
		}
		Ok(Element { key: Some(key_attr), value: res })
	}
}


#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct Element {
	pub key: Option<String>,
	#[serde(rename = "$value")]
	pub value: String,
}
impl StructObject for Element {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"key" => Some(Value::from(self.key.clone().unwrap_or_default())),
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

	fn get_combined_os_classes(&self) -> Vec<&OsClass> {
		self.matches.iter()
			.filter_map(|m| m.classes.iter().find(|&c| {
					let check = c.os_type.clone().unwrap_or_default();
					!check.contains("general") && !check.contains("specialized")
				})
			)
			.collect::<Vec<&OsClass>>()
	}
}
impl StructObject for OperatingSystem {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"ports" => Some(Value::from(self.ports.clone())),
			"matches" => Some(Value::from(self.matches.clone())),
			"vendor" => {
				let v = self.get_combined_os_classes()
					.iter()
					.map(|c| match c.os_type.clone() {
						Some(t) => t,
						None => self.matches.iter()
							.map(|m| m.name.to_lowercase())
							.map(|name| String::from( if name.contains("windows") { "Windows" } else { "Linux" } ))
							.next()
							.unwrap_or_else(|| String::from("Linux"))
					})
					.next();
					Some(Value::from(v.unwrap_or_else(|| String::from("Linux"))))
				},
			"purpose" => {
				let v = self.get_combined_os_classes()
					.iter()
					.map(|c| match c.os_type.clone() {
						Some(t) => t,
						None => self.matches.iter()
							.map(|m| m.name.to_lowercase())
							.map(|name| String::from( if name.contains("windows") { "Windows" } else { "Linux" } ))
							.next()
							.unwrap_or_else(|| String::from("Computer"))
					})
					.next();
					Some(Value::from(v.unwrap_or_else(|| String::from("Computer"))))
				},
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
			"accuracy" => Some(Value::from(self.accuracy)),
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
		osclass.cpe.push(Cpe::default());
		osclass

	}
}
impl StructObject for OsClass {
	fn get_field(&self, name: &str) -> Option<Value> {
		match name {
			"type" => Some(Value::from(self.os_type.clone().unwrap_or_default())),
			"os_type" => Some(Value::from(self.os_type.clone().unwrap_or_default())),
			"vendor" => Some(Value::from(self.vendor.clone())),
			"accuracy" => Some(Value::from(self.accuracy)),
			"family" => Some(Value::from(self.family.clone())),
			"generation" => Some(Value::from(self.generation.clone().unwrap_or_default())),
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
