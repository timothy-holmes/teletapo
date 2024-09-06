use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tapo::ApiClient;
use toml;

use std::{net::Ipv4Addr, time::Duration};
#[derive(Debug, Deserialize, Serialize)]
struct Device {
    name: String,
    location: String,
    ip: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    username: String,
    password: String,
    devices: Vec<Device>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Scanning mode, /24 as an argument
    #[arg(long)]
    scan: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = std::fs::read_to_string("./config.toml")?;
    let config: Config = toml::from_str(&config)?;
    let tapoclient = ApiClient::new(&config.username, &config.password);

    if cli.scan.is_some() {
        let address: Ipv4Addr = cli.scan.unwrap().parse()?;
        println!("Scanning mode... {:?}", address);
        let mut i = 0;
        let mut conf = Config {
            username: config.username,
            password: config.password,
            devices: Vec::<Device>::new(),
        };

        let client = Client::new();

        while i != 254 {
            i += 1;
            let addr: Ipv4Addr = Ipv4Addr::from_bits(address.to_bits() + i);
            let ipstring: String = addr.to_string();
            let url = format!("http://{}/app", ipstring);

            let timeout_duration = Duration::from_millis(30);
            print!("\rScanning {}...", addr);
            match client.get(url).timeout(timeout_duration).send().await {
                Ok(res) => match res.headers().get("server") {
                    Some(v) => {
                        if v.eq("SHIP 2.0") {
                            let dev = tapoclient.clone().p110(&ipstring).await?;
                            match dev.get_device_info().await {
                                Ok(info) => {
                                    let device = Device {
                                        name: info.nickname,
                                        location: "nowhere".into(),
                                        ip: info.ip,
                                    };
                                    conf.devices.push(device);
                                }
                                Err(_) => {
                                    // probably a tapo device, but not a plug
                                }
                            }
                        }
                    }
                    _ => {}
                },
                Err(_) => {}
            }
        }
        print!("\r                          \r");
        println!("Config file: \n");
        println!(r#"username = "{}""#, conf.username);
        println!(r#"password = "{}""#, conf.password);
        println!("devices = {{");
        for d in conf.devices {
            println!(
                r#"  {{ name = "{}", location = "{}", ip = "{}" }},"#,
                d.name, d.location, d.ip
            );
        }
        println!("}}");
        return Ok(());
    }

    for device in config.devices {
        let dev = tapoclient.clone().p110(&device.ip).await?;
        let energy_usage = dev.get_energy_usage().await?;

        // p110_energy_consumption,name=moustique,room=chambre,ip=192.168.1.21 current_power=1130i,month_energy=282i,month_runtime=1501i,today_energy=29i,today_runtime=397i
        println!(
            r#"tapo_power,name={},location={},ip={} current_power={}i,today_energy={}i,month_energy={}i,today_runtime={}i,month_runtime={}i"#,
            device.name,
            device.location,
            device.ip,
            energy_usage.current_power,
            energy_usage.today_energy,
            energy_usage.month_energy,
            energy_usage.today_runtime,
            energy_usage.month_runtime
        );
    }

    Ok(())
}
