module completions {

  # Manage `.org` files' tags from the CLI
  export extern tgr [
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Print the number of tags that matching a pattern
  export extern "tgr count" [
    pattern?: string          # Pattern to search for tags
    --file(-f): path          # File where to search for tags
    --include(-i): string     # Override config by including files that match <PATTERN>
    --exclude(-e): string     # Override config by excluding files that match <PATTERN>
    --help(-h)                # Print help
  ]

  # Locate the files that contain a tag matching a pattern
  export extern "tgr locate" [
    pattern: string           # Pattern to search for tags
    --strict(-s)              # Match the pattern strictly or loosely
    --include(-i): string     # Override config by including files that match <PATTERN>
    --exclude(-e): string     # Override config by excluding files that match <PATTERN>
    --help(-h)                # Print help
  ]

  # Refile org trees that have tags that match a pattern
  export extern "tgr refile" [
    pattern: string           # Pattern to find Org trees to refile
    --no-pager(-n)            # Print the contents to stdout instead of pager
    output_file?: string      # Name of the output file. If not given, output is paged or printed to the standard output
    --strict(-s)              # Match the pattern strictly or loosely
    --include(-i): string     # Override config by including files that match <PATTERN>
    --exclude(-e): string     # Override config by excluding files that match <PATTERN>
    --help(-h)                # Print help
  ]

  # Search tags in Org directory or file
  export extern "tgr search" [
    pattern: string           # Pattern used to search for tags
    --file(-f): path          # File where to search for tags
    --pager(-p)               # Force the output to a pager
    --include(-i): string     # Override config by including files that match <PATTERN>
    --exclude(-e): string     # Override config by excluding files that match <PATTERN>
    --help(-h)                # Print help
  ]

  # A wrapper around the `sed` cli utility to safely manipulate tags
  export extern "tgr sed" [
    tag: string               # The tag to be substituted. Note that this command will perform a substitution only on an exact match
    replacement: string       # The replacement
    --verbose(-v)             # Print additional information about substitutions
    --help(-h)                # Print help
  ]

  # Print tags to stdout or to pager
  export extern "tgr tags" [
    --file(-f): path          # Optional file to search instead of searching in the whole Org directory
    --pager(-p)               # Force the output to a pager
    --print(-P)               # Print all the tags into a .txt file
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "tgr help" [
  ]

  # Print the number of tags that matching a pattern
  export extern "tgr help count" [
  ]

  # Locate the files that contain a tag matching a pattern
  export extern "tgr help locate" [
  ]

  # Refile org trees that have tags that match a pattern
  export extern "tgr help refile" [
  ]

  # Search tags in Org directory or file
  export extern "tgr help search" [
  ]

  # A wrapper around the `sed` cli utility to safely manipulate tags
  export extern "tgr help sed" [
  ]

  # Print tags to stdout or to pager
  export extern "tgr help tags" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "tgr help help" [
  ]

}

export use completions *
