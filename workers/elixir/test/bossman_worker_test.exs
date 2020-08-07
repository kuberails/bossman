defmodule BossmanWorkerTest do
  use ExUnit.Case
  doctest BossmanWorker

  test "greets the world" do
    assert BossmanWorker.hello() == :world
  end
end
