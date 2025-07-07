mod util;
mod tests;

use crate::util::hash_bytes;
use crate::util::append_as_bytes;
use serde_json::Value;

#[derive(Debug)]
struct JsonData {
    fragments: Vec<JsonFragment>,
}

struct JsonFragment {
    data: Vec<u8>,
    flags: u128,
    index: u8,
    parent: Option<u8>,
    hash: String,
}

pub(crate) fn parse_json(json: &Value) -> serde_json::Result<JsonData> {
    let mut fragments = vec![];
    let mut index = 0;
    json_tree_walker(json, None, &mut fragments, &mut index)?;
    Ok(JsonData { fragments })
}

fn json_tree_walker(
    value: &Value,
    parent: Option<u8>,
    fragments: &mut Vec<JsonFragment>,
    index: &mut u8,
) -> serde_json::Result<()> {
    let data = serde_json::to_vec(&value)?;
    let flags = util::get_flags(&value);
    let hash = hash!(&data, flags);

    let current_index = *index;
    *index += 1;
    let current_parent_id = Some(current_index);
    let current = JsonFragment {
        data,
        flags,
        parent,
        index: current_index,
        hash,
    };
    fragments.push(current);


    match value {
        Value::Object(map) => {
            for (_k, v) in map {
                json_tree_walker(v,current_parent_id, fragments, index)?;
            }
        }
        Value::Array(arr) => {
            for v in arr {
                json_tree_walker(v, current_parent_id, fragments, index)?;
            }
        }
        _ => {}
    }

    Ok(())
}