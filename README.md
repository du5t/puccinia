# puccinia

*Puccinia allii is the taxonomic name for the species of Rust blight that
 infects onions.*

## summary

Rust program that consumes the API exposed through Tor control port. Requires
nightly (1.5.0) for now.

See:

- https://stem.torproject.org/api/control.html
- https://gitweb.torproject.org/torspec.git/tree/control-spec.txt

## technical notes

- testing

  In order to `cargo test` you will need to ensure there is an empty `torsocket`
  file in the current directory, only reachable by the tor/vidalia user (your
  user). Tor (or something else) must also be running and listening on that
  socket for the domain socket tests to succeed.
