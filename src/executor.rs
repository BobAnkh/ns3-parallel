use futures::stream::FuturesUnordered;
use futures::StreamExt;
use pbr::MultiBar;
use std::collections::HashMap;
use std::path::Path;
use std::process::Output;
use tokio::process::Command;
use tokio::task::spawn_blocking;

use crate::core::*;
use crate::error::Error;

const DEFAULT_RETRY_LIMIT: i32 = 5;

#[derive(Debug, Clone)]
pub struct Executor<T: Default + BuildParam<P>, P: BuildCmd> {
    config_path: String,
    ns3_path: String,
    task_concurrent: usize,
    retry_limit: u32,
    pub configs: HashMap<String, T>,
    pub outputs: HashMap<String, Vec<Task<P>>>,
}

#[derive(Debug, Clone)]
pub struct ExecutorBuilder {
    pub config_path: Option<String>,
    pub ns3_path: Option<String>,
    pub task_concurrent: Option<usize>,
    pub retry_limit: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Task<P: BuildCmd> {
    pub param: P,
    pub output: Output,
    pub stdout: String,
    pub stderr: String,
}

impl<T: Default + BuildParam<P>, P: BuildCmd> Executor<T, P> {
    pub fn get_config_path(&self) -> &str {
        &self.config_path
    }

    pub fn get_ns3_path(&self) -> &str {
        &self.ns3_path
    }

    pub fn get_task_concurrent(&self) -> usize {
        self.task_concurrent
    }

    pub fn get_retry_limit(&self) -> u32 {
        self.retry_limit
    }

    pub fn get_configs(&self) -> &HashMap<String, T> {
        &self.configs
    }

    pub fn get_outputs(&self) -> &HashMap<String, Vec<Task<P>>> {
        &self.outputs
    }

    pub async fn execute(&mut self) -> Result<(), Error> {
        let mut tasks = FuturesUnordered::new();
        let mut params_map: HashMap<&String, Vec<P>> = self
            .configs
            .iter()
            .map(|(k, v)| (k, v.build_param()))
            .collect();
        let total_count = params_map.iter().map(|(_, v)| v.len()).sum::<usize>() as u64;
        let mb = MultiBar::new();
        mb.println("Launch NS3 Tasks: ");
        let mut pb1 = mb.create_bar(total_count);
        mb.println("Complete NS3 Tasks: ");
        let mut pb2 = mb.create_bar(total_count);
        let progress = spawn_blocking(move || {
            mb.listen();
        });
        let ns3_dir = Path::new(&self.ns3_path);
        for params in params_map.drain() {
            for param in params.1 {
                pb1.inc();
                tasks.push(execute_ns3_program(
                    params.0,
                    ns3_dir,
                    param,
                    self.retry_limit,
                ));
                // If full, wait for one to finish.
                if tasks.len() >= self.task_concurrent {
                    if let Some(t) = tasks.next().await {
                        match t {
                            Ok((n, t)) => {
                                pb2.inc();
                                if let Some(v) = self.outputs.get_mut(n) {
                                    v.push(t);
                                }
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
            }
        }
        pb1.finish();
        // Wait for the remaining to finish.
        while let Some(t) = tasks.next().await {
            // handle response
            match t {
                Ok((n, t)) => {
                    pb2.inc();
                    if let Some(v) = self.outputs.get_mut(n) {
                        v.push(t);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        pb2.finish();
        progress.await.unwrap();
        Ok(())
    }
}

impl Default for ExecutorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutorBuilder {
    pub fn new() -> Self {
        Self {
            config_path: None,
            ns3_path: None,
            task_concurrent: None,
            retry_limit: None,
        }
    }

    pub fn config_path(mut self, config_path: &str) -> Self {
        self.config_path = Some(config_path.to_string());
        self
    }

    pub fn ns3_path(mut self, ns3_path: &str) -> Self {
        self.ns3_path = Some(ns3_path.to_string());
        self
    }

    pub fn task_concurrent(mut self, task_concurrent: usize) -> Self {
        self.task_concurrent = Some(task_concurrent);
        self
    }

    pub fn retry_limit(mut self, retry_limit: u32) -> Self {
        self.retry_limit = Some(retry_limit);
        self
    }

    pub fn build<'de, T: Default + BuildParam<P> + serde::de::Deserialize<'de>, P: BuildCmd>(
        self,
    ) -> Result<Executor<T, P>, Error> {
        let mut config_path = self
            .config_path
            .unwrap_or_else(|| "config.toml".to_string());
        let mut ns3_path = self.ns3_path.unwrap_or_else(|| "/".to_string());
        let task_concurrent = self.task_concurrent.unwrap_or_else(num_cpus::get);
        let retry_limit = self
            .retry_limit
            .unwrap_or_else(|| DEFAULT_RETRY_LIMIT as u32);
        // Check config file
        let config_file_path = match Path::new(&config_path).canonicalize() {
            Ok(path) => path,
            Err(e) => {
                return Err(Error::FileNotFound(format!(
                    "Can not locate config file: {:?}.",
                    e
                )));
            }
        };
        match config_file_path.extension() {
            Some(t) => {
                if t != "toml" {
                    return Err(Error::InvalidConfig(
                        "Config file must be a toml file.".to_string(),
                    ));
                }
            }
            None => {
                return Err(Error::InvalidConfig(
                    "Config file must have a valid file extension.".to_string(),
                ));
            }
        }
        config_path = config_file_path.display().to_string();
        // check ns3 directory
        let ns3_dir_path = match Path::new(&ns3_path).join("waf").canonicalize() {
            Ok(path) => path,
            Err(e) => {
                return Err(Error::FileNotFound(format!(
                    "Can not locate ns3 dir: {:?}.",
                    e
                )));
            }
        };
        ns3_path = ns3_dir_path.parent().unwrap().display().to_string();
        let configuration = match std::fs::read_to_string(config_file_path) {
            Ok(c) => c,
            Err(e) => {
                return Err(Error::InvalidConfig(format!(
                    "Config file cannot be opened at {}. Err: {:?}.",
                    &config_path, e
                )))
            }
        };
        let configs: toml::value::Table = match toml::from_str(&configuration) {
            Ok(t) => t,
            Err(e) => {
                return Err(Error::InvalidTomlFormat(format!(
                    "Config file is not a valid toml file. Err: {:?}.",
                    e
                )));
            }
        };

        let configs: HashMap<String, T> = configs
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned().try_into().unwrap()))
            .collect();
        let outputs: HashMap<String, Vec<Task<P>>> = configs
            .iter()
            .map(|(k, _)| (k.to_owned(), vec![]))
            .collect();

        Ok(Executor {
            config_path,
            ns3_path,
            task_concurrent,
            retry_limit,
            configs,
            outputs,
        })
    }
}

impl<P: BuildCmd> Task<P> {
    pub fn read_raw(&self) -> (Vec<u8>, Vec<u8>) {
        let stdout = self.output.stdout.clone();
        let stderr = self.output.stderr.clone();
        (stdout, stderr)
    }

    pub fn read_raw_stdout(&self) -> Vec<u8> {
        self.output.stdout.clone()
    }

    pub fn read_raw_stderr(&self) -> Vec<u8> {
        self.output.stderr.clone()
    }

    pub fn read(&self) -> (&str, &str) {
        (&self.stdout, &self.stderr)
    }

    pub fn read_stdout(&self) -> &str {
        &self.stdout
    }

    pub fn read_stderr(&self) -> &str {
        &self.stderr
    }
}

async fn execute_ns3_program<'a, P: BuildCmd>(
    name: &'a str,
    ns3_dir: &Path,
    param: P,
    retry_limit: u32,
) -> Result<(&'a str, Task<P>), Error> {
    let waf_path = ns3_dir.join("waf");
    let argument = param.build_cmd();
    let mut cnt = 1;
    let mut output = match Command::new(waf_path.as_os_str())
        .arg("--run")
        .arg(&argument)
        .current_dir(&ns3_dir)
        .output()
        .await
    {
        Ok(output) => output,
        Err(e) => {
            return Err(Error::ExecuteFail(format!(
                "Failed to execute NS3 program. Err: {:?}.",
                e
            )));
        }
    };
    while !output.status.success() && cnt <= retry_limit {
        cnt += 1;
        if cnt > retry_limit {
            return Err(Error::RetryLimitExceed);
        }
        output = match Command::new(waf_path.as_os_str())
            .arg("--run")
            .arg(&argument)
            .current_dir(&ns3_dir)
            .output()
            .await
        {
            Ok(output) => output,
            Err(e) => {
                return Err(Error::ExecuteFail(format!(
                    "Failed to execute NS3 program. Err: {:?}.",
                    e
                )));
            }
        };
    }
    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();
    Ok((
        name,
        Task {
            param,
            output,
            stdout,
            stderr,
        },
    ))
}
