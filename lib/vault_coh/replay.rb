# frozen_string_literal: true

module VaultCoh
  # A complete representation of all information able to be parsed from a
  # Company of Heroes 3 replay. Note that parsing is not yet exhaustive,
  # and iterative improvements will be made to pull more information from
  # replay files over time.
  class Replay
    # Takes a byte slice, parses it as a CoH3 replay, and returns a
    # representation of the parsed information. Any failures during parsing
    # or conversion will return an error.
    #
    # @param bytes [Array<Byte>] Byte array representation of a CoH3 replay file (required).
    #
    # @return [Replay]
    #
    # @example
    #   require 'vault_coh'
    #
    #   bytes = File.read('/path/to/replay.rec').unpack('C*')
    #   replay = VaultCoh::Replay.from_bytes(bytes)
    #   puts replay.version
    def self.from_bytes(bytes); end

    # The Company of Heroes 3 game version this replay was recorded on.
    # Note that this is probably more accurately described as the build
    # version, and represents the final segment of digits you see in the
    # game version on the game’s main menu. Every time the game is patched,
    # this version will change, and replays are generally only viewable on
    # the same game version they were recorded on.
    #
    # @return [Integer] unsigned, 16 bits
    def version; end

    # A UTF-16 representation of the recording user’s local time when the
    # replay was recorded. Note that value may contain non-standard
    # characters and is not guaranteed to be parsable into an accurate
    # date/time format.
    #
    # @return [String]
    def timestamp; end

    # The ID used by Relic to track this match on their internal servers.
    # This ID can be matched with an ID of the same name returned by
    # Relic’s CoH3 stats API, enabling linkage between replay files and
    # statistical information for a match.
    #
    # @return [Integer] unsigned, 64 bits
    def matchhistory_id; end

    # Map information for this match.
    #
    # @return [Map]
    def map; end

    # Filename of the map this match was played on. See {Map#filename}
    # for more information.
    #
    # @return [String]
    def map_filename; end

    # Localization ID of the map’s name. See {Map#localized_name_id}
    # for more information.
    #
    # @return [String]
    def map_localized_name_id; end

    # Localization ID of the map’s description. See
    # {Map#localized_description_id} for more information.
    #
    # @return [String]
    def map_localized_description_id; end

    # A list of all players who participated in this match.
    #
    # @return [Array<Player>]
    def players; end

    # A simple count of the number of ticks that were executed in this
    # match. Because CoH3’s engine runs at 8 ticks per second, you can
    # divide this value by 8 to get the duration of the match in seconds.
    #
    # @return [Integer]
    def length; end
  end
end
