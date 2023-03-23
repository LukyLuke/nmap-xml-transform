
## Nmap Scan Information

* **Command:** {{ command }}
* **Start:** {{ start_time }}
* **End:** {{ finish_time }}
* **Status:** {{ status }}
* **Hosts Scanned:** {{ runstats.hosts_total }}
* **Hosts Online:** {{ runstats.hosts_online }}
* **Hosts Offline:** {{ runstats.hosts_offline }}

## NMAP-Scripts

{%- for host in hosts %}
{%- for port in host.ports %}
{%- for script in port.script -%}

### Script: {{ script.id }}

{%- for item in script.items -%}

{%- if item.key|length() > 0 -%}
**{{ item.key }}:**
{% endif %}

{% for value in item.items -%}
* {% if value.key|length() > 0 %}{{ value.key }}: {% endif %}{{ value.value }}
{% endfor %}
{% endfor %}

{%- endfor %}
{%- endfor %}
{% endfor %}
