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

## Examples

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

