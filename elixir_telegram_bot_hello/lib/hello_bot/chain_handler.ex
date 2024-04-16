defmodule HelloBot.ChainHandler do
  @moduledoc false

  use Telegex.Chain.Handler

  pipeline([
    HelloBot.Chain.Response.Start,
    HelloBot.Chain.Response.Ping,
    HelloBot.Chain.Hello,
    HelloBot.Chain.EchoText
  ])
end
