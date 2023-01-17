use mentat_tokenizer::Span;
use serde_json::Value;

pub struct Workflow {
    workflows: Vec<WorkFlowName>,
}

/// <workflow name>(<concurrency>) { ...scenarios }
pub struct WorkFlowName {
    name: String,
    concurrency: u8,
    scenarios: Vec<Scenario>,
    span: Span,
}

/// <scenario name> {
///   <output path> = <action type>(<input>);
/// }
pub struct Scenario {
    name: String,
    output_path: String,
    action_type: ActionType,
    input: Value,
    span: Span,
}

pub enum ActionType {
    // "derive"
    Derive,
    // "find_balance"
    FindBalance,
    // "generate_key"
    GenerateKey,
    // "get_blob"
    GetBlob,
    // "http_request"
    HttpRequest,
    // "load_env"
    LoadEnv,
    // 1 + {{fee}}
    Math,
    // "print_message"
    PrintMessage,
    // "save_account"
    SaveAccount,
    // "set_blob"
    SetBlob,
    // Empty string
    SetVariable,
    // "random_string"
    RandomString,
    // "random_number"
    RandomNumber,
}
