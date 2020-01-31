extern crate reqwest;
extern crate serde;

use reqwest::Error;
use std::time::Duration;

pub trait RestPath<U> {
    fn get_path(params: U) -> Result<String, Error>;
}

pub struct RestClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl RestClient {
    /// Create a new `RestClient`.
    ///
    /// ### Params
    ///
    /// `base_url`: The base URL for the given Rest API
    pub fn new(base_url: String) -> Result<RestClient, Error> {
        let client = RestClient {
            base_url: base_url,
            client: reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()?,
        };

        Ok(client)
    }

    /// Make a sychronous GET request against a given URL.
    ///
    /// The calling type must implement the `RestPath` and
    /// `serde::de::Deserialize` Traits.
    ///
    /// ### Params
    /// `params`: parameter object for the given call
    pub fn get_sync<U, T>(&mut self, params: U) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + RestPath<U>,
    {
        let path = T::get_path(params)?;
        let res = self
            .client
            .get((self.base_url.clone() + &path).as_str())
            .send()?;
        let json = res.json()?;

        Ok(json)
    }
}
