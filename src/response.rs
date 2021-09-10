use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub node_version: NodeVersion,
    pub environment: String,
    pub coordinator: bool,
    pub starting: bool,
    pub uptime: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NodeVersion {
    pub version: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResults {
    pub id: String,
    pub info_uri: String,
    pub partial_cancel_uri: Option<String>,
    pub next_uri: Option<String>,
    pub columns: Option<Vec<QueryResultColumn>>,
    pub data: Option<Vec<Value>>,
    pub stats: QueryStats,
    pub warnings: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ColumnTypeSignatureArguments {
    pub kind: String,
    pub value: Value, // TODO: Deserialize based on adjacent tagging
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnTypeSignature {
    pub raw_type: String,
    pub arguments: Vec<ColumnTypeSignatureArguments>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResultColumn {
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub type_name: String,
    pub type_signature: ColumnTypeSignature,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryStats {
    pub state: QueryState,
    pub queued: bool,
    pub scheduled: bool,
    pub nodes: u64,
    pub total_splits: u64,
    pub queued_splits: u64,
    pub running_splits: u64,
    pub completed_splits: u64,
    pub cpu_time_millis: u64,
    pub wall_time_millis: u64,
    pub queued_time_millis: u64,
    pub elapsed_time_millis: u64,
    pub processed_rows: u64,
    pub processed_bytes: u64,
    pub physical_input_bytes: u64,
    pub peak_memory_bytes: u64,
    pub spilled_bytes: u64,
    pub root_stage: Option<QueryStage>,
    pub progress_percentage: Option<f32>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryStage {
    pub stage_id: String,
    pub state: QueryState,
    pub done: bool,
    pub nodes: u64,
    pub total_splits: u64,
    pub queued_splits: u64,
    pub running_splits: u64,
    pub completed_splits: u64,
    pub cpu_time_millis: u64,
    pub wall_time_millis: u64,
    pub processed_rows: u64,
    pub processed_bytes: u64,
    pub physical_input_bytes: u64,
    pub sub_stages: Vec<QueryStage>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueryState {
    Aborted,
    Failed,
    Finished,
    Finishing,
    Flushing,
    Queued,
    Running,
    Scheduled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_initial_response() {
        let res: QueryResults = serde_json::from_str(r#"
            {
              "id": "19991231_000000_00000_00000",
              "infoUri": "http://localhost:8080/ui/query.html?19991231_000000_00000_00000",
              "nextUri": "http://localhost:8080/v1/statement/queued/19991231_000000_00000_00000/.../1",
              "stats": {
                "state": "QUEUED",
                "queued": true,
                "scheduled": false,
                "nodes": 0,
                "totalSplits": 0,
                "queuedSplits": 0,
                "runningSplits": 0,
                "completedSplits": 0,
                "cpuTimeMillis": 0,
                "wallTimeMillis": 0,
                "queuedTimeMillis": 0,
                "elapsedTimeMillis": 0,
                "processedRows": 0,
                "processedBytes": 0,
                "physicalInputBytes": 0,
                "peakMemoryBytes": 0,
                "spilledBytes": 0
              },
              "warnings": []
            }"#).unwrap();
        let exp = QueryResults {
            id: String::from("19991231_000000_00000_00000"),
            info_uri: String::from(
                "http://localhost:8080/ui/query.html?19991231_000000_00000_00000",
            ),
            partial_cancel_uri: None,
            next_uri: Some(String::from(
                "http://localhost:8080/v1/statement/queued/19991231_000000_00000_00000/.../1",
            )),
            columns: None,
            data: None,
            stats: QueryStats {
                state: QueryState::Queued,
                queued: true,
                scheduled: false,
                nodes: 0,
                total_splits: 0,
                queued_splits: 0,
                running_splits: 0,
                completed_splits: 0,
                cpu_time_millis: 0,
                wall_time_millis: 0,
                queued_time_millis: 0,
                elapsed_time_millis: 0,
                processed_rows: 0,
                processed_bytes: 0,
                physical_input_bytes: 0,
                peak_memory_bytes: 0,
                spilled_bytes: 0,
                root_stage: None,
                progress_percentage: None,
            },
            warnings: vec![],
        };
        assert_eq!(res, exp);
    }

    #[test]
    fn deserialize_info_response() {
        let res: Info = serde_json::from_str(
            r#"
            {
              "nodeVersion": {
                "version": "360"
              },
              "environment": "docker",
              "coordinator": true,
              "starting": false,
              "uptime": "1.00m"
            }"#,
        )
        .unwrap();
        let exp = Info {
            node_version: NodeVersion {
                version: String::from("360"),
            },
            environment: String::from("docker"),
            coordinator: true,
            starting: false,
            uptime: String::from("1.00m"),
        };
        assert_eq!(res, exp);
    }
}
