defmodule BossmanWorker do
  @moduledoc """
  Documentation for `BossmanWorker`.
  """

  @doc """
  Run
  """
  def run do
    Bossman.Protobuf.V1alpha1.Options.new(
      backoff_limit: {:_backoff_limit, 2},
      env: [
        %{
          env: {:value, %{name: "hello", value: "world"}}
        },
        %{
          env: {:value, %{name: "hello", value: "world"}}
        },
        %{
          env: {:value, %{name: "hello", value: "world"}}
        },
        %{
          env:
            {:valueFrom,
             %{
               name: "praveen",
               valueFrom: {:secretKeyRef, %{name: "1", key: "@"}}
             }}
        }
      ]
    )
    |> IO.inspect(label: "Structs")
    |> Bossman.Protobuf.V1alpha1.Options.encode()
    |> IO.inspect(label: "Binary")
    |> Bossman.Protobuf.V1alpha1.Options.decode()
  end
end
