pub enum ReservedVariable {
    // "confirmation_depth"
    ConfirmationDepth,
    // "dry_run"
    DryRun,
    // "network"
    Network,
    // "operations"
    Operations,
    // "preprocess_metadata"
    PreprocessMetadata,
    // "suggested_fee"
    SuggestedFee,
    // "transaction"
    Transaction,
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
