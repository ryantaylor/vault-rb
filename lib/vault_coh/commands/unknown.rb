# frozen_string_literal: true

module VaultCoh
  module Commands
    # Data object representing an unknown (i.e. not yet
    # handled) command.
    class Unknown
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

      # This value identifies the type of the command
      # (build, move, stop, etc.). Commands with similar
      # functionality can be grouped by this value.
      #
      # @return [Integer] unsigned, 8 bits
      def action_type; end

      # Returns a hash representation of the object.
      #
      # @return [Hash]
      def to_h; end
    end
  end
end
