# frozen_string_literal: true

module VaultCoh
  # Representation of a user-sent chat message in a Company of Heroes
  # 3 replay. Messages are collected during command parsing and then
  # associated with the {Player} instance that sent them. To access,
  # see {Player#messages}.
  class Message
    # This value is the tick at which the message was found while
    # parsing the replay, which represents the time in the replay at
    # which it was sent. Because CoH3’s engine runs at 8 ticks per
    # second, you can divide this value by 8 to get the number of
    # seconds since the replay began, which will tell you when this
    # message was sent.
    #
    # @return [Integer] unsigned, 32 bits
    def tick; end

    # UTF-16 encoded representation of the message sent by the player.
    #
    # @return [String]
    def message; end
  end
end
