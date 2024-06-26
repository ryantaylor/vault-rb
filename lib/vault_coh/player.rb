# frozen_string_literal: true

module VaultCoh
  # Game-specific player representation. Includes generally immutable
  # information alongside data specific to the replay being parsed.
  class Player
    # Name of the player at the time the replay was recorded. Note
    # that the player may have changed their name since time of
    # recording. If attempting to uniquely identify players across
    # replay files, look at {#steam_id} and {#profile_id} instead.
    # The string is UTF-16 encoded.
    #
    # @return [String]
    def name; end

    # Whether or not the player was a human or an AI/CPU player.
    #
    # @return [Boolean]
    def human?; end

    # The faction selected by the player in this match.
    #
    # @return [Faction::AMERICANS|Faction::WEHRMACHT|Faction::BRITISH|Faction::AFRIKAKORPS]
    def faction; end

    # The team the player was assigned to. Currently only head-to-head
    # matchups are supported (max two teams).
    #
    # @return [Team::FIRST|Team::SECOND]
    def team; end

    # The pbgid of the battlegroup the player selected, or +nil+ if no
    # battlegroup was selected. For details on what this ID represents
    # please see {Commands::SelectBattlegroup#pbgid}.
    #
    # @return [Integer|NilClass]
    def battlegroup; end

    # The Steam ID of the player, or +nil+ if the player is AI. This ID
    # can be used to uniquely identify a player between replays, and
    # connect them to their Steam profile.
    #
    # @return [Integer|NilClass] unsigned, 64 bits
    def steam_id; end

    # The Relic profile ID of the player, or +nil+ if the player is AI.
    # This ID can be used to uniquely identify a player between replays,
    # and can be used to query statistical information about the player
    # from Relic’s stats API.
    #
    # @return [Integer|NilClass] unsigned, 64 bits
    def profile_id; end

    # A list of all messages sent by the player in the match. Sorted
    # chronologically from first to last.
    #
    # @return [Array<Message>]
    def messages; end

    # A list of all commands executed by the player in the match.
    # Sorted chronologically from first to last.
    #
    # @return [Array<Command>]
    def commands; end

    # A list of only build-related commands executed by the player in
    # the match. A build command is any that enqueues the construction
    # of a new unit or upgrade. Sorted chronologically from first to last.
    #
    # @return [Array<Command>]
    def build_commands; end

    # A list of only battlegroup-related commands executed by the player
    # in the match. A battlegroup command is any that involves the select
    # or use of battlegroups and their abilities. Sorted chronologically
    # from first to last.
    #
    # @return [Array<Command>]
    def battlegroup_commands; end

    # Returns a hash representation of the object.
    #
    # @return [Hash]
    def to_h; end
  end
end
