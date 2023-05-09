# frozen_string_literal: true

require_relative 'lib/vault_coh/version'

Gem::Specification.new do |spec|
  spec.name = 'vault_coh'
  spec.version = VaultCoh::VERSION
  spec.authors = ['ryantaylor']
  spec.email = ['2320507+ryantaylor@users.noreply.github.com']

  spec.summary = 'CoH3 replay parsing in Ruby with Rust'
  spec.description = 'Company of Heroes replay parsing in Ruby using the vault parsing library via a Rust native extension.'
  spec.homepage = 'https://github.com/ryantaylor/vault-rb'
  spec.license = 'MIT'
  spec.required_ruby_version = '>= 3.1.0'
  spec.required_rubygems_version = '>= 3.3.11'

  spec.metadata['allowed_push_host'] = 'https://rubygems.org'

  spec.metadata['homepage_uri'] = spec.homepage
  spec.metadata['source_code_uri'] = spec.homepage
  spec.metadata['changelog_uri'] = spec.homepage

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|circleci)|appveyor)})
    end
  end
  spec.bindir = 'exe'
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']
  spec.extensions = ['ext/vault_coh/Cargo.toml']
  spec.metadata['rubygems_mfa_required'] = 'true'
end
