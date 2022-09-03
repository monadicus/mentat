CreateErrorType! {
  ContextError

  new unexpected_token {
    args: (received, expected, span),
    error_msgs: [
        "{span}",
        "Received token `{received}` but expected `{expected}`.",
    ],
    suggestions: [],
  }
}
