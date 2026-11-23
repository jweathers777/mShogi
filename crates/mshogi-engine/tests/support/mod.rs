use mshogi_engine::{
    ExecutionObservation, ExecutionRequest, ExecutionResponse, PROTOCOL_VERSION, incremental,
};

// Valid requests use production admission. Invalid-request tests inspect the
// child failure without treating it as an admissible host observation.
pub fn decode_response(
    bytes: &[u8],
    request: &ExecutionRequest,
) -> Result<ExecutionResponse, String> {
    if request.valid() {
        ExecutionResponse::decode(bytes, request)
    } else {
        let wire: incremental::Response =
            serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        Ok(ExecutionResponse {
            protocol: PROTOCOL_VERSION,
            observation: ExecutionObservation::new(request.context.clone(), wire.into_result()),
        })
    }
}
