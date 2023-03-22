## Nmap Scan Information

* **Command:** {{ command }}
* **Start:** {{ start_time }}
* **End:** {{ finish_time }}
* **Status:** {{ status }}
* **Hosts Scanned:** {{ runstats.hosts_total }}
* **Hosts Online:** {{ runstats.hosts_online }}
* **Hosts Offline:** {{ runstats.hosts_offline }}


{%- for host in hosts %}

----

## Host: {{ host.ipv4 }}

{%- if host.hostnames.hostname|length() > 0 %}

### Hostnames

{% for name in host.hostnames.hostname -%}
* {{ name.name }} {%- if name.host_type|length() > 0 %} ({{ name.host_type }}){% endif %}
{%- endfor %}
{%- endif %}

{%- if host.address|length() > 0 %}

### Addresses

{% for addr in host.address -%}
* **{{ addr.address_type }}:** {{ addr.addr }} {%- if addr.vendor|length() > 0 %} ({{ addr.vendor }}){% endif %}
{% endfor %}
{%- endif %}

{%- if host.os.matches|length() > 0 %}

### Operating System: {{ host.os.vendor }} {{ host.os.purpose }}

{% for os in host.os.matches -%}
* **{{ os.name }}:** {{ os.accuracy }}% {% if os.classes|length() > 0 %}
  {%- for class in os.classes %}
  * {{ class.vendor }}{%- if class.generation|length() > 0 %} {{ class.generation }}{% endif %}: {{ class.accuracy }}% *({{ class.type }})*
  {%- endfor %}
{% endif %}
{%- endfor %}
{%- endif %}

{%- if host.ports.port|length() > 0 %}

### Ports

| Port | Service | Product | State | CVEs |
|----|----|----|----|----|

{%- for port in host.ports.port %}
| {{ port.port }}/{{ port.protocol }} | {{ port.service.service }} | {{ port.service.product }} {%- if port.service.version|length() > 0 %} ({{ port.service.version }}){% endif %} | {{ port.state.state }} | {% for script in port.script -%}
  {%- if script.id == "vulners" %}
    {%- for cve in script.items %}[{{ cve.id }} (CVSS: {{ cve.cvss }})](https://vulners.com/{{ cve.type }}/{{ cve.id }}); {% endfor -%}
  {%- endif -%}
{%- endfor %} |
{%- endfor %}
{%- endif %}
{% endfor %}
