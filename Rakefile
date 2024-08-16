# frozen_string_literal: true

require 'bundler/gem_tasks'
require 'rspec/core/rake_task'

RSpec::Core::RakeTask.new(:spec)

require 'rubocop/rake_task'

RuboCop::RakeTask.new

require 'rake/extensiontask'

desc 'build gem'
task build: :compile

Rake::ExtensionTask.new('vault_coh') do |ext|
  ext.lib_dir = 'lib/vault_coh'
end

task default: %i[compile spec rubocop]
