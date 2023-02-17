
use serde::Deserialize;
use std::string::String;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub(crate) struct NampRun {
	scaninfo: ScanInfo,
	#[serde(rename = "host")]
	hosts: Vec<Host>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ScanInfo {
	#[serde(rename = "type")]
	scantype: String,
	protocol: Protocol,
	#[serde(rename = "numservices")]
	num_services: i32,
	services: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Host {
	status: Option<HostState>,
	#[serde(rename = "@addr")]
	address: Option<IpAddr>,
	#[serde(from = "PortsList")]
	ports: Vec<Port>,
	//os: Option<OperatingSystem>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HostState {
	state: String,
	reason_ttl: i8,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Port {
	protocol: Protocol,
	#[serde(rename = "portid")]
	port: i32,
	#[serde(rename = "state")]
	state: String,
	product: Option<Product>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Protocol {
	TCP,
	UDP,
	ICMP,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum State {
	UP,
	OPEN,
	FILTERED,
	CLOSED,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Product {
	//#[serde(tag = "service", content = "name")]
	service: Option<String>,
	//#[serde(tag = "service", content = "product")]
	product: Option<String>,
	//#[serde(tag = "service", content = "servicefp")]
	product_footprint: Option<String>,
	scripts: Vec<Script>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Script {
	id: String,
	#[serde(rename = "output")]
	raw_output: Option<String>,
	#[serde(rename = "elem")]
	elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Element {
	key: String,
	value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OperatingSystem {
	//#[serde(rename = "portused")]
	//ports: Vec<Port>,
	#[serde(rename = "osmatch")]
	matches: Vec<OsMatch>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OsMatch {
	name: String,
	accuracy: i8,
	#[serde(rename = "osclass")]
	classes: Vec<OsClass>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OsClass {
	#[serde(rename = "type")]
	os_type: String,
	vendor: String,
	#[serde(rename = "osfamily")]
	family: String,
	#[serde(rename = "osgen")]
	generation: String,
	accuracy: i8,
}





