pub use std::net::Ipv4Addr;

use crate::error;

// Tokens generated by Anondns are random 32-character string hashes
type Token = String;

#[derive(serde_derive::Deserialize, Debug)]
struct RegisterResponse {
    code: i32,
    data: String,
    #[serde(default, rename = "name")]
    _name: Option<String>,
    #[serde(default, rename = "status")]
    _status: Option<i32>,
    #[serde(default, rename = "type")]
    _ftype: Option<String>,
    #[serde(default)]
    token: Option<String>,
    #[serde(default, rename = "updated")]
    _updated: Option<String>
}

pub struct Service {
    client: reqwest::blocking::Client,
}

impl Service {
    /// Creates a new instance of the API service and initializes a reqwest blocking client
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut service = api::Service::new();
    /// // Do stuff...
    /// ```
    pub fn new() -> Self {
        Service {
            client: reqwest::blocking::Client::new()
        }
    }
 
    /// Registers a new DNS subdomain and returns its associated `Token` upon success
    ///
    /// # Arguments
    ///
    /// * `spoint toubdomain` - A string slice containing the name that will be used for the subdomain
    /// * `target` - The Ipv4Addr the subdomain will redirect to 
    ///
    /// # Errors
    /// 
    /// This function may return one of the following error variants:
    /// - `DnsApiError::BadRequest((i32, String))`
    /// - `DnsApiError::UnknownErrorCode((i32, String))`
    /// - `DnsApiError::Reqwest(reqwest::Error)
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut service = anondns_api::api::Service::new();
    /// let token = service.register("example_subdomain", std::net::Ipv4Addr::new(127, 0, 0, 1)).unwrap();
    /// ```
    pub fn register(&mut self, subdomain: &str, target: Ipv4Addr) -> Result<Token, error::DnsApiError> {
        let url = format!("https://anondns.net/api/register/{}.anondns.net/a/{}", subdomain, target.to_string());
        let json: RegisterResponse = self.client.get(url)
            .send()?
            .json()?;

        return match json.code {
            0 => Ok(json.token.unwrap()),
            1 => Err(error::DnsApiError::BadRequest((1, json.data))),
            v => Err(error::DnsApiError::UnknownErrorCode((v, json.data)))
        };
    }

    /// Updates the redirect target of the specified subdomain and returns the new target `Ipv4Addr` upon success
    ///
    /// # Arguments
    ///
    /// * `subdomain` - A string slice that holds the DNS subdomain to update
    /// * `target` - The new target Ipv4Addr the subdomain will redirect to
    ///
    /// # Errors
    /// 
    /// This function may return one of the following error variants:
    /// - `DnsApiError::BadRequest((i32, String))`
    /// - `DnsApiError::UnknownErrorCode((i32, String))`
    /// - `DnsApiError::Reqwest(reqwest::Error)
    /// - `DnsApiError::AddressParse(std::net::AddrParseError)`
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut service = anondns_api::api::Service::new();
    /// let token = service.register("example_subdomain", std::net::Ipv4Addr::new(127, 0, 0, 1));
    /// let result = service.update("example_subdomain", std::net::Ipv4Addr::new(255, 255, 255, 255), String::from("example_token"));
    /// ```
    pub fn update(&mut self, subdomain: &str, target: Ipv4Addr, token: Token) -> Result<Ipv4Addr, error::DnsApiError> {
        let url = format!("https://anondns.net/api/set/{}.anondns.net/{}/a/{}", subdomain, token, target.to_string());
        let json: RegisterResponse = self.client.get(url)
            .send()?
            .json()?;

        return match json.code {
            0 => Ok(json.data.parse()?),
            1 => Err(error::DnsApiError::BadRequest((1, json.data))),
            v => Err(error::DnsApiError::UnknownErrorCode((v, json.data)))
        };
    }
}