use serde_json::Value;

use crate::domain::{data::data_functions::array_from_input_data, error::Error};

use super::repeat_client::RepeatClient;

// INPUT DATA

pub fn input_for_repeat(
    client : &RepeatClient,
    prompt : &str,
    context : &Value
) -> Result<Vec<String>, Error>
{
    let mut ret = vec![];

    for input in &client.input
    {
        ret.append( &mut array_from_input_data( prompt, context, input )? );
    }

    Ok( ret )
}

// END INPUT DATA