defmodule HelloBot.Applicaton do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    updates_handler = HelloBot.UpdatesPoller

    children = [
      updates_handler
    ]

    opts = [strategy: :one_for_one, name: HelloBot.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
