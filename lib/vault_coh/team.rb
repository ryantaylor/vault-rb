# frozen_string_literal: true

module VaultCoh
  # Identifiers for a player's team membership.
  class Team
    FIRST = 0
    SECOND = 1

    # Integer representation of the assigned team.
    #
    # @see https://docs.rs/vault/3.0.0/vault/enum.Team.html
    #
    # @return [FIRST|SECOND]
    def value; end
  end
end
