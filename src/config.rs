use std::{fs, io::Write};
use serde_json::{Map, Value};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = {
        match Config::load() {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);

                let cfg = Config {
                    ip: (127,0,0,1),
                    port: 8000,
                    ssl_enabled: false,
                    ssl_key: "".to_owned(),
                    ssl_cert: "".to_owned(),
                    image_path: "./files".to_owned(),
                    jpg_quality: 80,
                    base_address: "http://localhost".to_owned()
                };

                match create_new_config_file(&cfg) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{:?}", e);
                        panic!()
                    }
                }

                cfg
            }
        }
    };
}

pub fn ip() -> (u8, u8, u8, u8) {
    CONFIG.ip()
}
pub fn port() -> u64 {
    CONFIG.port()
}
// pub fn ssl_enabled() -> bool {
//     CONFIG.ssl_enabled()
// }
// pub fn ssl_key() -> String {
//     CONFIG.ssl_key()
// }
// pub fn ssl_cert() -> String {
//     CONFIG.ssl_cert()
// }
pub fn image_path() -> String {
    CONFIG.image_path()
}
pub fn jpg_quality() -> u8 {
    CONFIG.jpg_quality()
}
// pub fn address() -> SocketAddrV4 {
//     SocketAddrV4::new(Ipv4Addr::new(ip().0, ip().1, ip().2, ip().3), port())
// }
pub fn base_address() -> String {
    CONFIG.base_address()
}
pub fn address_as_str() -> String {
    format!("{}.{}.{}.{}:{}", ip().0, ip().1, ip().2, ip().3, port())
}

fn create_new_config_file(config: &Config) -> Result<(), String> {
    let mut file = match fs::File::create("./config.json") {
        Ok(v) => v,
        Err(e) => return Err(format!("{:?}", e))
    };

    let cfg_str = config.serialize();
    match file.write_all(&cfg_str.into_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e))
    }
}


#[derive(Debug)]
pub struct Config {
    ip: (u8, u8, u8, u8),
    port: u64,
    ssl_enabled: bool,
    ssl_key: String,
    ssl_cert: String,
    image_path: String,
    jpg_quality: u8,
    base_address: String
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let raw_str = match fs::read_to_string("./config.json") {
            Ok(s) => s,
            Err(e) => return Err(format!("Cannot read config file --- {:?}", e))
        };

        let config: Map<String, Value> = match serde_json::from_str(&raw_str) {
            Ok(m) => m,
            Err(e) => return Err(format!("Cannot parse config file --- {:?}", e))
        };

        let port = Self::parse_port(&config)?;
        let ip = Self::parse_ip(&config)?;
        let ssl_enabled = Self::parse_ssl_enabled(&config)?;
        let ssl_key = Self::parse_ssl_key(&config)?;
        let ssl_cert = Self::parse_ssl_cert(&config)?;
        let image_path = Self::parse_img_path(&config)?;
        let jpg_quality = Self::parse_jpg_quality(&config)?;
        let base_address = Self::parse_base_address(&config)?;

        Ok(Config {
            ip,
            port,
            ssl_enabled,
            ssl_key,
            ssl_cert,
            image_path,
            jpg_quality,
            base_address
        })
    }


    // DESERIALIZE
    fn parse_ip(e: &Map<String, Value>) -> Result<(u8, u8, u8, u8), String> {
        if let Some(v) = e.get("ip") {
            if let Some(arr) = v.as_array() {
                let a;
                let b;
                let c;
                let d;
                if let Some(w) = arr.get(0) {
                    match w.as_u64() {
                        Some(u) => a = u as u8,
                        None => return Err("Cannot parse IP from configuration file.".to_owned())
                    }
                } else {return Err("Cannot parse IP from configuration file.".to_owned())}
                if let Some(w) = arr.get(1) {
                    match w.as_u64() {
                        Some(u) => b = u as u8,
                        None => return Err("Cannot parse IP from configuration file.".to_owned())
                    }
                } else {return Err("Cannot parse IP from configuration file.".to_owned())}
                if let Some(w) = arr.get(2) {
                    match w.as_u64() {
                        Some(u) => c = u as u8,
                        None => return Err("Cannot parse IP from configuration file.".to_owned())
                    }
                } else {return Err("Cannot parse IP from configuration file.".to_owned())}
                if let Some(w) = arr.get(3) {
                    match w.as_u64() {
                        Some(u) => d = u as u8,
                        None => return Err("Cannot parse IP from configuration file.".to_owned())
                    }
                } else {return Err("Cannot parse IP from configuration file.".to_owned())}

                return Ok((a, b, c, d))
            }
        }
        Err("Cannot parse IP from Configuration file.".to_owned())
    }

    fn parse_port(e: &Map<String, Value>) -> Result<u64, String> {
        if let Some(v) = e.get("port") {
            if let Some(p) = v.as_u64() {
                return Ok(p)
            }
        }
        Err("Cannot parse Port in Configuration file.".to_owned())
    }

    fn parse_ssl_enabled(e: &Map<String, Value>) -> Result<bool, String> {
        if let Some(v) = e.get("ssl") {
            if let Some(o) = v.as_object() {
                if let Some(enabled) = o.get("enabled") {
                    if let Some(b) = enabled.as_bool() {
                        return Ok(b)
                    }
                } 
            }
        }

        Err("Cannot parse SSL enabled in Config file.".to_owned())
    }

    fn parse_ssl_key(e: &Map<String, Value>) -> Result<String, String> {
        if let Some(v) = e.get("ssl") {
            if let Some(o) = v.as_object() {
                if let Some(key) = o.get("key") {
                    if let Some(b) = key.as_str() {
                        return Ok(b.to_owned())
                    }
                } 
            }
        }

        Err("Cannot parse path to SSL Key in Config File.".to_owned())
    }

    fn parse_ssl_cert(e: &Map<String, Value>) -> Result<String, String> {
        if let Some(v) = e.get("ssl") {
            if let Some(o) = v.as_object() {
                if let Some(cert) = o.get("cert") {
                    if let Some(b) = cert.as_str() {
                        return Ok(b.to_owned())
                    }
                } 
            }
        }

        Err("Cannot parse path to SSL Certificate in Config File.".to_owned())
    }

    fn parse_img_path(e: &Map<String, Value>) -> Result<String, String> {
        if let Some(v) = e.get("image_path") {
            if let Some(p) = v.as_str() {
                return Ok(p.to_owned())
            }
        }

        Err("Cannot parse path to Image folder in Configuration file.".to_owned())
    }

    fn parse_jpg_quality(e: &Map<String, Value>) -> Result<u8, String> {
        if let Some(v) = e.get("jpg_quality") {
            if let Some(p) = v.as_u64() {
                return Ok(p as u8)
            }
        }

        Err("Cannot parse JPG Quality in Configuration file.".to_owned())
    }

    fn parse_base_address(e: &Map<String, Value>) -> Result<String, String> {
        if let Some(v) = e.get("base_address") {
            if let Some(s) = v.as_str() {
                return Ok(s.to_owned())
            }
        }

        Err("Cannot parse base_address in Configuration file.".to_owned())
    }


    // GETTERS
    pub fn ip(&self) -> (u8, u8, u8, u8) {
        self.ip
    }
    pub fn port(&self) -> u64 {
        self.port
    }
    // pub fn ssl_enabled(&self) -> bool {
    //     self.ssl_enabled
    // }
    // pub fn ssl_key(&self) -> String {
    //     self.ssl_key.clone()
    // }
    // pub fn ssl_cert(&self) -> String {
    //     self.ssl_cert.clone()
    // }
    pub fn image_path(&self) -> String {
        self.image_path.clone()
    }
    pub fn jpg_quality(&self) -> u8 {
        self.jpg_quality
    }
    pub fn base_address(&self) -> String {
        self.base_address.clone()
    }


    // SERIALIZE
    pub fn serialize(&self) -> String {
        let ip_str = format!("[
        {},
        {},
        {},
        {}
    ]", self.ip.0, self.ip.1, self.ip.2, self.ip.3);

        format!("{{
    \"ip\": {},
    \"port\": {},
    \"ssl\": {{
        \"enabled\": {},
        \"key\": \"{}\",
        \"cert\": \"{}\"
    }},
    \"image_path\": \"{}\",
    \"jpg_quality\": {}
}}", ip_str, self.port, self.ssl_enabled, self.ssl_key, self.ssl_cert, self.image_path, self.jpg_quality)
    }
}
