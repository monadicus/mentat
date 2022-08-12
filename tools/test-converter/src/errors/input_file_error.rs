CreateErrorType! {
  InputFileError

  new unknown_input_file_extension {
    args: (ext),
    error_msgs: [
        "Unknown config file extension `{ext}`.",
    ],
    suggestions: [
        "The supported input file format is toml."
    ],
  }
}
