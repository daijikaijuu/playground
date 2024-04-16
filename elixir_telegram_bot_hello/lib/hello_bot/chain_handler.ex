defmodule HelloBot.ChainHandler do
  @moduledoc false

  use Telegex.Chain.Handler

  pipeline([
    HelloBot.RespStartChain,
    HelloBot.CallHelloChain
  ])
end
