use serde::{Deserialize, Serialize};

type PortNumber = u32;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    id: Option<String>,
    name: Option<String>,
    auth_key: Option<String>,
    login: Option<String>,
    password: Option<String>,
    #[serde(rename = "procParams")]
    process_params: Option<ProcessParams>,

    #[serde(rename = "sslParams")]
    ssl_params: Option<SSLParams>,

    #[serde(rename = "configsvrs")]
    config_servers: Option<Vec<ProcessParams>>,
    routers: Option<Vec<ProcessParams>>,
    shards: Option<Vec<ShardConfig>>,

    members: Option<Vec<ReplicaSetMember>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessParams {
    dbpath: Option<String>,
    ipv6: Option<bool>,
    logappend: Option<bool>,
    logpath: Option<String>,
    journal: Option<bool>,
    port: Option<PortNumber>,
    #[serde(rename = "storageEngine")]
    storage_engine: Option<String>,

    noprealloc: Option<bool>,
    nssize: Option<u64>,
    smallfiles: Option<bool>,

    bind_ip: Option<String>,

    #[serde(default)]
    #[serde(rename = "setParameter")]
    set_parameter: serde_json::Value,

    #[serde(rename = "oplogSize")]
    oplog_size: Option<u64>,

    shardsvr: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SSLParams {
    #[serde(rename = "sslCAFile")]
    ca_file: Option<String>,
    #[serde(rename = "sslOnNormalPorts")]
    on_normal_ports: Option<bool>,
    #[serde(rename = "sslPEMKeyFile")]
    pem_key_file: Option<String>,
    #[serde(rename = "sslWeakCertificateValidation")]
    weak_certification_validation: Option<bool>,

    #[serde(rename = "sslAllowInvalidCertificates")]
    ssh_allow_invalid_certificates: Option<bool>,
}

use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReplicaSetParams {
    priority: Option<f64>,

    #[serde(default)]
    tags: HashMap<String, String>,
    #[serde(rename = "arbiterOnly")]
    arbiter_only: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReplicaSetMember {
    #[serde(rename = "procParams")]
    process_params: ProcessParams,

    #[serde(rename = "rsParams")]
    replica_set_params: Option<ReplicaSetParams>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShardParams {
    #[serde(rename = "procParams")]
    process_params: Option<ProcessParams>,

    #[serde(default)]
    members: Vec<ReplicaSetMember>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShardConfig {
    id: String,
    #[serde(rename = "shardParams")]
    shard_params: Option<ShardParams>,
}
