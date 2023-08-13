# frozen_string_literal: true

module VaultCoh
  # Base class that all command wrappers extend.
  class Command
    # Data object that contains information specific to the
    # type of command it represents.
    #
    # @return [Commands::BuildSquad|Commands::SelectBattlegroup|Commands::Unknown]
    def value; end
  end
end
