use ns3_parallel::{BuildCmd, BuildParam, Executor, ExecutorBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub sim_time: u32,
    pub app_name: String,
    pub policy: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub sim_time: u32,
    pub app_name: String,
    pub policy: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            sim_time: 100,
            app_name: "ns3-tcp-bbr".to_string(),
            policy: vec![1, 2, 3],
        }
    }
}

impl BuildParam<Param> for Config {
    fn build_param(&self) -> Vec<Param> {
        let mut params: Vec<Param> = Vec::new();
        for policy in &self.policy {
            let param = Param {
                sim_time: self.sim_time,
                app_name: self.app_name.clone(),
                policy: *policy,
            };
            params.push(param);
        }
        params
    }
}
impl BuildCmd for Param {
    fn build_cmd(&self) -> String {
        format!(
            "simple-ns3 --app-name={} --sim-time={} --policy={}",
            self.app_name, self.sim_time, self.policy
        )
    }
}

#[tokio::main]
async fn main() {
    let mut exe: Executor<Config, Param> = ExecutorBuilder::new()
        .config_path("config.toml")
        .ns3_path("ns-allinone-3.33/ns-3.33/")
        .build()
        .unwrap();
    let _ = exe.execute().await.unwrap();
    let outputs = exe.get_outputs().to_owned();
    for (_, output) in outputs {
        for task in output {
            println!("{}", task.stderr);
        }
    }
}
