# frozen_string_literal: true

module VaultCoh
  # Identifiers for the Company of Heroes 3 factions.
  class Faction
    AMERICANS = 'Americans'
    BRITISH = 'British'
    WEHRMACHT = 'Wehrmacht'
    AFRIKAKORPS = 'AfrikaKorps'

    # String identifier of the given faction.
    #
    # @see https://docs.rs/vault/3.0.0/vault/enum.Faction.html
    #
    # @return [AMERICANS|BRITISH|WEHRMACHT|AFRIKAKORPS]
    def value; end
  end
end
