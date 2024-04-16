defmodule HelloBot do
  @moduledoc false

  def work_mode, do: Application.get_env(:hello_bot, :work_mode)
end
