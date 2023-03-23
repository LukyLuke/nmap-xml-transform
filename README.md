This project was initiated due to many assets and IOCs from network scans and exposure analyses which where documented in [DFIR-IRIS](https://github.com/dfir-iris/iris-web).
To not have to add and link all these assets and IOCs manually, the nmap xml output was perfect to transform into different formats and finally into a bunch of curl commands to manage everything via the API.


# NMAP XML Transform

Transforms an NMAP-XML File to any other file based on a template.

## NMAP-Command

```bash
$ sudo nmap -oX nmap_output_example.xml --top-ports=2000 --script=vulners -O -sV -sC 192.168.88.0/24
Starting Nmap 7.93 ( https://nmap.org ) at 2023-02-24 09:44 CET
Nmap scan report for 192.168.88.1
Host is up (0.00052s latency).
Not shown: 1993 closed tcp ports (reset)
PORT     STATE SERVICE        VERSION
21/tcp   open  ftp            MikroTik router ftpd 6.49.2
22/tcp   open  ssh            MikroTik RouterOS sshd (protocol 2.0)
23/tcp   open  telnet         Linux telnetd
53/tcp   open  domain         (generic dns response: NOTIMP)
80/tcp   open  http           MikroTik router config httpd
2000/tcp open  bandwidth-test MikroTik bandwidth-test server
8291/tcp open  unknown
1 service unrecognized despite returning data. If you know the service/version, please submit the following fingerprint at https://nmap.org/cgi-bin/submit.cgi?new-service :
SF-Port53-TCP:V=7.93%I=7%D=2/24%Time=63F878EB%P=x86_64-pc-linux-gnu%r(DNSV
SF:ersionBindReqTCP,E,"\0\x0c\0\x06\x81\x84\0\0\0\0\0\0\0\0");
MAC Address: E4:8D:8C:E8:AA:D4 (Routerboard.com)
Device type: general purpose
Running: Linux 2.6.X|3.X
OS CPE: cpe:/o:linux:linux_kernel:2.6 cpe:/o:linux:linux_kernel:3
OS details: Linux 2.6.32 - 3.10
Network Distance: 1 hop
Service Info: Host: Mikrotik; OSs: Linux, RouterOS; Device: router; CPE: cpe:/o:mikrotik:routeros, cpe:/o:linux:linux_kernel

...
```

An example output you can see in the example.xml file provided in the templates folder.

## Transform the xml

```bash
$ nmap_xml nmap_output_example.xml templates/output_template.csv > transformed.csv
```

## Templates

Templates are based on JINJA-2, implemented by the project minijinja.

Some examples to transform the xml file you can find in the templates folder.

### Provided Templates

* **hosts_detail.md** - Create a Markdown document for all hosts including ports and found CVEs with links to vulners.com
* **iris_assets_api.curl** - Creates a basch script with curl commands to create all assets, IOCs and link them in DFIR-IRIS
* **iris_cve_ioc_api.curl** - Creates a bash script with curl commands to create all found CVEs as IOCs in DFIR-IRIS
* **script_items.md** - For demonstrational purposes to show how CVEs and other script output can be processed

### Create a Template

Create any jinja-2 template what is needed.
The dataformat looks like the following:

```
{
    finish_time: ""
    elapsed_time: ""
    status: ""
    scaninfo: {
        scantype: "",
        protocol: TCP|UDP|SCTP|IP,
        num_services: 0,
        services: "",
    },
    hosts: [
        {
            ipv4: "",
            ipv6: "",
            mac: "",
            status: {
                state: Up|Down|Skipped|Unknown,
                reason: "",
                ttl: 0,
            }?,
            hostnames: [
                {
                    name: "",
                    host_type: "",
                },
            ],
            address: [
                {
                    addr: "",
                    address_type: Ipv4|Ipv6|Mac|Unknown,
                    vendor: "",
                },
            ],
            ports: [
                {
                    protocol: TCP|UDP|SCTP|IP,
                    port: 0,
                    state: {
                        state: Open|Closed|Filtered,
                        reason: "",
                        ttl: 0,
                    },
                    service: {
                        service: "",
                        product: "",
                        version: "",
                        ssl: SSL|NO,
                        footprint: "",
                        cpe: [
                            {
                                value: "",
                            },
                        ],
                    }?,
                    script: [
                        {
                            id: "",
                            raw: "",
                            items: [
                                {
                                    key: "",
                                    items: [
                                        {
                                            key:"",
                                            value:""
                                        }
                                    ]
                                }
                            ],
                        },
                    ],
                },
            ],
            os: {
                vendor: "",
                purpose: "",
                ports: [
                    {
                        protocol: TCP|UDP|SCTP|IP,
                        port: 0,
                        state: {
                            state: Open|Closed|Filtered,
                            reason: "",
                            ttl: 0,
                        },
                        service: {
                            service: "",
                            product: "",
                            version: "",
                            ssl: SSL|NO,
                            footprint: "",
                            cpe: [
                                {
                                    value: "",
                                },
                            ],
                        },
                        script: [
                            {
                                id: "",
                                raw: "",
                                items: [
                                    {
                                        key: "",
                                        items: [
                                            {
                                                key:"",
                                                value:""
                                            }
                                        ]
                                    }
                                ],
                            },
                        ],
                    },
                ],
                matches: [
                    {
                        name: "",
                        accuracy: 0,
                        classes: [
                            {
                                os_type: "",
                                vendor: "",
                                accuracy: 0,
                                family: "",
                                generation: "",
                                cpe: [
                                    {
                                        value: "",
                                    },
                                ],
                            },
                        ],
                    },
                ],
            }?,
        },
    ],
    command: "",
    start_time: "",
    runstats: {
        hosts_total: 0,
        hosts_online: 0,
        hosts_offline: 0,
        finished: {
            finish_time: "",
            elapsed: 0.0,
            status: "",
        },
        hosts: {
            up: 0,
            down: 0,
            total: 0,
        },
    },
}
```


## Examples

### CSV with all Assets for DFIR-IRIS

Scan the network and transform the output into CSV to import in DFIR-IRIS Assets:

```bash
$ sudo nmap -oX nmap_scan_88.xml --top-ports=2000 --script=vulners -O -sV -sC 192.168.88.0/24
...

$ nmap_xml nmap_scan_88.xml templates/iris_assets.csv | tee iris_assets.csv
asset_name,asset_type_name,asset_description,asset_ip,asset_domain,asset_tags
192.168.88.1,Windows - Computer,,192.168.88.1,,import
192.168.88.2,Windows - Computer,,192.168.88.2,,import
192.168.88.254,Windows - Computer,,192.168.88.254,,import
192.168.88.221,Windows - Computer,,192.168.88.221,,import
```

### CURL Scripts for DFIR-IRIS IOC-API

Scan the network and transform the output into curl calls to import all found CVEs by vulners into IOCs in DFIR-IRIS via the API:

```bash
$ sudo nmap -oX nmap_scan_88.xml --top-ports=2000 --script=vulners -O -sV -sC 192.168.88.0/24
...

$ nmap_xml nmap_scan_88.xml templates/iris_cve_ioc_api.curl | tee iris_ioc.sh
#!/bin/sh

AUTHORIZATION_TOKEN="Profile -> My Settings -> API Key"
IRIS_DOMAIN="iris.example.dom"
CASE_ID=1

curl -k -X POST \
-H "Authorization: Bearer ${AUTHORIZATION_TOKEN}" \
-H 'Content-Type: application/json' \
-d '{ "ioc_type_id": 144, "ioc_tlp_id": 2, "ioc_value": "EXPLOITPACK:98FE96309F9524B8C84C508837551A19", "ioc_description": "CVSS3 Value: 5.8\nSee [Vulners: EXPLOITPACK:98FE96309F9524B8C84C508837551A19](https://vulners.com/exploitpack/EXPLOITPACK:98FE96309F9524B8C84C508837551A19) for more details", "ioc_tags": "import,vulners", "custom_attributes": {}, "cid": '${CASE_ID}' }' \
-s \
https://${IRIS_DOMAIN}/case/ioc/add
...
```
