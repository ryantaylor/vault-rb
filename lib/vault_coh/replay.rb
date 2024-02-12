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

    # The type of game this replay represents. Note that this information
    # is parsed on a best-effort basis and therefore may not always be
    # correct. Also note that it's currently not known if there's a way to
    # differentiate between automatch and custom games for replays recorded
    # before the replay system release in patch 1.4.0. Games played before
    # that patch will be marked as either +skirmish+ (for local AI games) or
    # +multiplayer+ (for networked custom or automatch games). Games recorded
    # on or after patch 1.4.0 will properly differentiate between +custom+
    # and +automatch+ games.
    #
    # @return [GameType::SKIRMISH|GameType::MULTIPLAYER|GameType::AUTOMATCH|GameType::CUSTOM]
    def game_type; end

    # The ID used by Relic to track this match on their internal servers.
    # This ID can be matched with an ID of the same name returned by
    # Relic’s CoH3 stats API, enabling linkage between replay files and
    # statistical information for a match. When the game type is +skirmish+,
    # there is no ID assigned by Relic, so this will be +nil+.
    #
    # @return [Integer|NilClass] unsigned, 64 bits
    def matchhistory_id; end

    # The UUID of the base game mod this replay ran on. If no mod was used,
    # this will be a nil UUID (all zeroes).
    #
    # @return [String]
    def mod_uuid; end

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

    # Returns a hash representation of the object.
    #
    # @return [Hash]
    def to_h; end
  end
end
