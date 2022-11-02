use ns3_parallel::{executor::ConfigFormat, BuildCmd, BuildParam, Executor, ExecutorBuilder};
use serde::{Deserialize, Serialize};

// This is what you want to read from your configuration file.
// Each part of the configuration file will be formed into a struct of this.
// Fields not specified in your config file will use the default value defined in the Default trait.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MyConfig {
    pub sim_time: u32,
    pub app_name: String,
    pub policy: Vec<u32>,
}

// A set of parameters you need to execute ns3 program.
// One param struct means one ns3 task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyParam {
    pub sim_time: u32,
    pub app_name: String,
    pub policy: u32,
}

// This trait impl will give you the default value of your config struct.
// When a field is not specified in the config file, the default value will be used.
impl Default for MyConfig {
    fn default() -> Self {
        MyConfig {
            sim_time: 100,
            app_name: "ns3-tcp-bbr".to_string(),
            policy: vec![1, 2, 3],
        }
    }
}

// From each of your config struct, you have to generate a vector of param struct.
impl BuildParam<MyParam> for MyConfig {
    fn build_param(&self) -> Vec<MyParam> {
        let mut params: Vec<MyParam> = Vec::new();
        for policy in &self.policy {
            let param = MyParam {
                sim_time: self.sim_time,
                app_name: self.app_name.clone(),
                policy: *policy,
            };
            params.push(param);
        }
        params
    }
}

// From each of your param struct, you have to generate the command line passed to ns3 program.
// The output of method build_cmd will be passed as the argument of "waf --run" in the command line.
impl BuildCmd for MyParam {
    fn build_cmd(&self) -> String {
        format!(
            "simple-ns3 --app-name={} --sim-time={} --policy={}",
            self.app_name, self.sim_time, self.policy
        )
    }
}

#[tokio::main]
async fn main() {
    // ========== Use toml format as the config file ==========
    // Use ExecutorBuilder to build your executor.
    let mut exe: Executor<MyConfig, MyParam> = ExecutorBuilder::new()
        .config_path("config.toml")
        .config_format(ConfigFormat::Toml)
        .ns3_path("ns-allinone-3.33/ns-3.33/")
        .build()
        .unwrap();

    // Run your executor.
    let _ = exe.execute().await.unwrap();

    // Collect your results.
    let outputs = exe.get_outputs().to_owned();

    // Here I just print all the results, you can do whatever you want with them here.
    for (_, output) in outputs {
        for task in output {
            println!("{}", task.stderr);
        }
    }

    // ========== Use ron format as the config file ==========
    // Use ExecutorBuilder to build your executor.
    let mut exe: Executor<MyConfig, MyParam> = ExecutorBuilder::new()
        .config_path("config.ron")
        .config_format(ConfigFormat::Ron)
        .ns3_path("ns-allinone-3.33/ns-3.33/")
        .task_concurrent(4)
        .retry_limit(2)
        .build()
        .unwrap();

    // Run your executor.
    let _ = exe.execute().await.unwrap();

    // Collect your results.
    let outputs = exe.get_outputs().to_owned();

    // Here I just print all the results, you can do whatever you want with them here.
    for (_, output) in outputs {
        for task in output {
            println!("{}", task.stderr);
        }
    }
}
