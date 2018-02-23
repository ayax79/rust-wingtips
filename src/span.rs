use std::collections::HashMap;
use std::str::FromStr;
use errors::TracingError;
use helpers::GetAs;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpanPurpose {
    Server,
    Client,
    LocalOnly,
    Unknown,
}

impl FromStr for SpanPurpose {
    type Err = TracingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SERVER" => Ok(SpanPurpose::Server),
            "CLIENT" => Ok(SpanPurpose::Client),
            "LOCAL_ONLY" => Ok(SpanPurpose::LocalOnly),
            _ => Ok(SpanPurpose::Unknown)
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    pub trace_id: String,
    pub parent_span_id: String,
    pub span_id: String,
    pub span_name: String,
    pub sampleable: bool,
    pub user_id: String,
    pub span_purpose: SpanPurpose,
    pub span_start_time_epoch_micros: u64,
    pub duration_nanos: u64,
}

impl Span {
    fn from_key_value_str(key_values: &str) -> Result<Span, TracingError> {
        let kv_map: HashMap<&str, &str> = key_values.split(',') // split by comma
            .flat_map(|pair: &str| {
                // split by = sign and then trim, giving us a collection of key value pairs
                let kv: Vec<&str> = pair.split("=").map(|s| s.trim())
                    .collect();
                // split into a tuple if there are two items
                // wrap in a vec and allow flap map to filter out the entries with
                // less than two items
                if kv.len() == 2 {
                    vec![(kv[0], kv[1])]
                } else {
                    vec![]
                }
            })
            .collect();


        Ok(Span{
            trace_id: kv_map.get_as_string("trace_id")?,
            parent_span_id: kv_map.get_as_string("parent_span_id")?,
            span_id: kv_map.get_as_string("span_id")?,
            span_name: kv_map.get_as_string("span_name")?,
            sampleable: kv_map.get_as_bool("sampleable")?,
            user_id: kv_map.get_as_string("user_id")?,
            span_purpose: kv_map.get_as_str("span_purpose").and_then(|v| FromStr::from_str(v))?,
            span_start_time_epoch_micros: kv_map.get_as_u64("span_start_time_epoch_micros")?,
            duration_nanos: kv_map.get_as_u64("duration")?
        })
    }
}



