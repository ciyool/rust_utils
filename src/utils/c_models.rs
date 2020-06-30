#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MsgErrCommon {
    pub code: i32,
    pub msg: String,
}

impl MsgErrCommon{
    pub fn new(code: i32,msg: &str)->MsgErrCommon{
        MsgErrCommon{
            code,
            msg: msg.to_string()
        }
    }
}