CreateErrorType! {
  LexerError

  new illegal_negative_decimals {
    args: (number),
    error_msgs: [
        "Negative decimals `{number}` not allowed.",
    ],
    suggestions: [
        "Try using a integer, or a non-negative decimal number."
    ],
  }

  from could_not_lex_decimal_number {
    args: (number),
    error_msgs: [
        "Could not lex decimal number `{number}`."
    ],
    suggestions: [],
  }

  from could_not_lex_signed_number {
    args: (number),
    error_msgs: [
        "Could not lex signed number `{number}`."
    ],
    suggestions: [],
  }

  from could_not_lex_number {
    args: (number),
    error_msgs: [
        "Could not lex number `{number}`."
    ],
    suggestions: [],
  }

  new ident_started_with_a_number {
    args: (),
    error_msgs: [
        "Identifiers can't start with a number.",
    ],
    suggestions: [
        "Remove the leading numbers from the ident."
    ],
  }

  new unexpected_eof {
    args: (),
    error_msgs: [
        "Unexpected EOF.",
    ],
    suggestions: [

    ],
  }

  new expected_comment {
    args: (found),
    error_msgs: [
        "Found `/{found}`.",
    ],
    suggestions: [
      "Expected a follow up / to make a comment.",
    ],
  }

  new unclosed_string {
    args: (found),
    error_msgs: [
        "Unclosed string `\"{found}`.",
    ],
    suggestions: [
      "Expected a \" to close the string.",
    ],
  }

  new unknown_token {
    args: (found),
    error_msgs: [
        "Unknown token `{found}`.",
    ],
    suggestions: [
    ],
  }
}
