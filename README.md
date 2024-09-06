# `teletapo`

[Telegraf](https://github.com/influxdata/telegraf) input plugin for monitoring
[Tapo](https://www.tapo.com/product/smart-plug/tapo-p110/) smart plug power
usage.

# Usage

Write a configuration file in `toml` next to the `teletapo` binary:

```toml
username = "name@example.net"
password = "hunter2"
devices = [
  { name = "spline reticuliter", ip = "192.168.1.21", location = "penthouse" },
  { name = "high velocity fourier transformer", ip = "192.168.1.22", location = "dressing" },
  { name = "lost pointer recycler", ip = "192.168.1.47", location = "pantry" }
]
```

`username` and `password` are the same one used in the mobile app.

Add something like this to the `telegraf` configuration (e.g. `/etc/telegraf/telegraf.conf`)

```
[[inputs.exec]]
  commands = [ "bash -c 'cd /some/path && ./teletapo'" ]
  timeout = "2s"
  interval = "2s"
  data_format = "influx"
```

# Metrics

Measurement name is `tapo_power`. Tags and fields are broken down
below:

## Tags:

|Name|Description|
|---|---|
|name|Name of the device|
|location|Location of the device|
|ip|IP of the device|

> Note: the API doesn't seem to surface the `location`, and so it has been
> preferred to use the values provided in a configuration file.

## Measurements:

|Name|Unit|Description|
|---|---|---|
|current\_power|Milliwatts|Current power draw|
|today\_energy|W·h<sup>-1</sup>|Integrated energy usage, resets at midnight|
|month\_energy|W·h<sup>-1</sup>|Integrated energy usage, resets each month|
|today\_runtime|s|Total run time for the day|
|today\_runtime|s|Total run time for the month|

# Device scanner

If the username and password have been provided in a config file
`teletapo --scan 192.168.1.0` will scan `192.168.1.0/24` for tapo
plugs and will output a config file.
