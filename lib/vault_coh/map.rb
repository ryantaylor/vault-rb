# frozen_string_literal: true

module VaultCoh
  # Representation of all map-related information that can be parsed
  # from a Company of Heroes 3 replay.
  class Map
    # This is a “filename” in the sense that its structure resembles
    # one, but it doesn’t actually point to any file on the file
    # system. The final “token” in this string (if you split by slash)
    # generally corresponds to the map name returned by the CoH3 stats
    # API. The string is UTF-8 encoded.
    #
    # @return [String]
    def filename; end

    # Entity ID that corresponds to a localization string that
    # represents the localized name of the map. Conventionally these
    # IDs do not change between patches, but that isn’t guaranteed.
    # The string is UTF-16 encoded.
    #
    # @return [String]
    def localized_name_id; end

    # Entity ID that corresponds to a localization string that
    # represents the localized description of the map. Conventionally
    # these IDs do not change between patches, but that isn’t
    # guaranteed. The string is UTF-16 encoded.
    #
    # @return [String]
    def localized_description_id; end

    # Returns a hash representation of the object.
    #
    # @return [Hash]
    def to_h; end
  end
end
