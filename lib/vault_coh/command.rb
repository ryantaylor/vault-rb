# frozen_string_literal: true

module VaultCoh
  # Base class that all command wrappers extend.
  class Command
    # Data object that contains information specific to the
    # type of command it represents.
    #
    # @return [Commands::BuildSquad |
    #          Commands::SelectBattlegroup |
    #          Commands::SelectBattlegroupAbility |
    #          Commands::UseBattlegroupAbility |
    #          Commands::Unknown]
    def value; end

    # Returns a hash representation of the object.
    #
    # @return [Hash]
    def to_h; end
  end
end
