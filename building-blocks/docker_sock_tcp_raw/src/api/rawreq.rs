use crate::api::api_utils::get_formatted_api_request;
use crate::api::DockerApiClient;
use crate::utils::Response;

pub trait Rawreq: DockerApiClient {
    /// Get version info for Docker
    /// Returns a JSON serialized string containing this information
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate docker_sock_tcp_raw;
    ///
    /// use rust_docker::api::containers::Containers;
    /// use rust_docker::api::version::Version;
    /// use rust_docker::client::DockerClient;
    ///
    /// let client = match DockerClient::new("unix:///var/run/docker.sock") {
    ///     Ok(a) => a,
    ///     Err(err) => {
    ///         println!("{}", err);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// match client.get_version_info() {
    ///     Ok(info) => println!("Version Info : {}", info),
    ///     Err(err) => println!("An error occured : {}", err),
    /// }
    /// ```
    fn get_raw_req(&self,req: &str) -> Result<String, String> {

        let resp = match self.request(&req) {
            Some(resp) => String::from_utf8(resp).unwrap(),
            None => return Err("Got no response from docker host.".to_string()),
        };

        Ok(resp)
    }
}
