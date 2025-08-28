# Vault

[![Gem Version](https://badge.fury.io/rb/vault_coh.svg)](https://badge.fury.io/rb/vault_coh) [![Documentation](https://img.shields.io/badge/View-Documentation-blue.svg)](https://rubydoc.info/github/ryantaylor/vault-rb/v6.3.0)

A native Ruby client wrapper for the [vault](https://github.com/ryantaylor/vault) Company of Heroes replay parser, integrated via a Rust native extension.

## Installation

First, make sure you have a relatively recent version of the Rust toolchain installed (minimum supported Rust version for `vault` is 1.66.0). You can install Rust easily using [rustup](https://rustup.rs/).

Then you can add to Gemfile:
```
gem 'vault_coh'
```
And run `bundle install`.

## Usage

Currently `vault` accepts a byte array and parses it as a Company of Heroes 3 replay file, returning a parsed object when successful or an error on failure:
```ruby
require 'vault_coh'

bytes = File.read('/path/to/replay.rec').unpack('C*')
replay = VaultCoh::Replay.from_bytes(bytes)
puts replay.version
```
All information available from parsing can be found in the [documentation](https://rubydoc.info/github/ryantaylor/vault-rb/v6.3.0).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ryantaylor/vault-rb.

If you need to make changes to the Rust code, you can compile the extension by running `rake compile` or run tests with `cargo test`.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
