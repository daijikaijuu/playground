defmodule ElixirTelegramBotHelloTest do
  use ExUnit.Case
  doctest ElixirTelegramBotHello

  test "greets the world" do
    assert ElixirTelegramBotHello.hello() == :world
  end
end
