mod module;

use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Stdio;

use tokio::io::AsyncReadExt;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let home_path = env::var("IOTDB_HOME").unwrap();
    let config_path = home_path.to_string() + "/conf";
    let metrics_config_path = config_path.to_string() + "/iotdb-metric.yml";
    // write metrics config
    let metrics_config = module::MetricsConfig {
        enable_metric: true,
        metric_reporter_list: vec!["PROMETHEUS".to_string()],
        monitor_type: "MICROMETER".to_string(),
        metric_level: "ALL".to_string(),
        predefined_metrics: vec![
            "JVM".to_string(),
            "LOGBACK".to_string(),
        ],
        push_period_in_second: 0,
        prometheus_exporter_port: 9091
    };
    let metrics_config_content = serde_yaml::to_string(&metrics_config).unwrap();
    let mut file = File::create(metrics_config_path).unwrap();
    write!(file, "{}", metrics_config_content).expect("file write failed");
    // start iotdb
    let script_path = home_path + "/mate/scripts/start-iotdb-standalone.sh";
    let fail_msg = "failed to execute process";
    let mut cmd = Command::new("/bin/bash")
        .arg(script_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(fail_msg);

    let stdout = &mut String::new();
    cmd.stdout.take().unwrap().read_to_string(stdout).await.unwrap();
    println!("stdout : {:?}", stdout);

    let stderr = &mut String::new();
    cmd.stderr.take().unwrap().read_to_string(stderr).await.unwrap();
    println!("stderr : {:?}", stderr);
}
