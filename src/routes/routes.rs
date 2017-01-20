use rocket_contrib::JSON;

#[derive(Serialize)]
pub struct ModelMessage<T> {
    model: Option<T>,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    status: String,
    reason: Option<String>,
}

pub fn create_model_message<T>(_model: Option<T>, _error: Option<String>) -> JSON<ModelMessage<T>> {
    JSON(ModelMessage{
            model: _model,
            error: _error
        })
}

pub fn create_response_message(_status: String, _reason: Option<String>) -> JSON<ResponseMessage> {
    JSON(ResponseMessage{
            status: _status,
            reason: _reason
        })
}
