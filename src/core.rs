//! Core traits for the library

/// # BuildParam
///
/// This trait is used to build the parameters for the NS3 program.
///
/// Implement this trait and also the `Default` trait on your own config struct.
///
/// ## Example
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use ns3_parallel::{BuildParam, BuildCmd};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// #[serde(default)]
/// pub struct Config {
///     pub sim_time: u32,
///     pub app_name: String,
///     pub policy: Vec<u32>,
/// }
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// pub struct Param {
///     pub sim_time: u32,
///     pub app_name: String,
///     pub policy: u32,
/// }
///
/// impl Default for Config {
///     fn default() -> Self {
///         Config {
///             sim_time: 100,
///             app_name: "ns3-tcp-bbr".to_string(),
///             policy: vec![1, 2, 3],
///         }
///     }
/// }
///
/// impl BuildParam<Param> for Config {
///     fn build_param(&self) -> Vec<Param> {
///         let mut params: Vec<Param> = Vec::new();
///         for policy in &self.policy {
///             let param = Param {
///                 sim_time: self.sim_time,
///                 app_name: self.app_name.clone(),
///                 policy: *policy,
///             };
///             params.push(param);
///         }
///         params
///     }      
/// }
/// impl BuildCmd for Param {
///     fn build_cmd(&self) -> String {
///         format!(
///             "xxx --app-name={} --sim-time={} --policy={}",
///             self.app_name, self.sim_time, self.policy
///         )
///     }
/// }
pub trait BuildParam<P: BuildCmd> {
    fn build_param(&self) -> Vec<P>;
}

/// # BuildCmd
///
/// This trait is used to build the command line for the NS3 program.
///
/// Implement this trait on your own param struct.
///
/// ## Example
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use ns3_parallel::{BuildParam, BuildCmd};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// #[serde(default)]
/// pub struct Config {
///     pub sim_time: u32,
///     pub app_name: String,
///     pub policy: Vec<u32>,
/// }
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// pub struct Param {
///     pub sim_time: u32,
///     pub app_name: String,
///     pub policy: u32,
/// }
///
/// impl Default for Config {
///     fn default() -> Self {
///         Config {
///             sim_time: 100,
///             app_name: "ns3-tcp-bbr".to_string(),
///             policy: vec![1, 2, 3],
///         }
///     }
/// }
///
/// impl BuildParam<Param> for Config {
///     fn build_param(&self) -> Vec<Param> {
///         let mut params: Vec<Param> = Vec::new();
///         for policy in &self.policy {
///             let param = Param {
///                 sim_time: self.sim_time,
///                 app_name: self.app_name.clone(),
///                 policy: *policy,
///             };
///             params.push(param);
///         }
///         params
///     }      
/// }
/// impl BuildCmd for Param {
///     fn build_cmd(&self) -> String {
///         format!(
///             "xxx --app-name={} --sim-time={} --policy={}",
///             self.app_name, self.sim_time, self.policy
///         )
///     }
/// }
pub trait BuildCmd {
    fn build_cmd(&self) -> String;
}
