# frozen_string_literal: true

module VaultCoh
  module Commands
    # Data object representing information parsed from a
    # battlegroup ability usage command.
    class UseBattlegroupAbility
      # This value is the tick at which the command was
      # found while parsing the replay, which represents
      # the time in the replay at which it was executed.
      # Because CoH3's engine runs at 8 ticks per second,
      # you can divide this value by 8 to get the number
      # of seconds since the replay began, which will tell
      # you when this command was executed.
      #
      # @return [Integer] unsigned, 32 bits
      def tick; end

      # Internal ID that uniquely identifies the battlegroup
      # ability used. This value can be matched to CoH3
      # attribute files in order to determine the battlegroup
      # ability being used. Note that, while rare, it is
      # possible that this value may change between patches
      # for the same battlegroup.
      #
      # @return [Integer] unsigned, 32 bits
      def pbgid; end

      # Returns a hash representation of the object.
      #
      # @return [Hash]
      def to_h; end
    end
  end
end
