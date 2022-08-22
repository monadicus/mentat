CreateErrorType! {
  RulesFileError

  new unknown_input_file_extension {
    args: (ext),
    error_msgs: [
        "Unknown config file extension `{ext}`.",
    ],
    suggestions: [
        "The supported input file format is toml."
    ],
  }

  from could_not_open_config_file {
    args: (file_path),
    error_msgs: [
        "Could not open rules file `{file_path}`."
    ],
    suggestions: [],
  }

  from failed_to_read_rules {
    args: (),
    error_msgs: [
        "Failed to read rules."
    ],
    suggestions: [],
  }
}
