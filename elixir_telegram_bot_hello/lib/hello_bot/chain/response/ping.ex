defmodule HelloBot.Chain.Response.Ping do
  @moduledoc false

  use Telegex.Chain, {:command, :ping}

  @impl true
  def handle(%{chat: chat, text: _text} = _message, context) do
    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "pong"
        }
    }

    {:done, context}
  end
end
