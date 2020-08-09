defmodule BossmanWorker do
  @moduledoc """
  Documentation for `BossmanWorker`.
  """

  @doc """
  Run
  """
  def run do
    Bossman.Protobuf.V1alpha1.Options.new(
      env: [
        Bossman.Protobuf.V1alpha1.Options.EnvValue.new(%{name: "hello", value: "world"}),
        Bossman.Protobuf.V1alpha1.Options.EnvFrom.new(%{
          name: "praveen",
          valueFrom:
            {:secretKeyRef,
             Bossman.Protobuf.V1alpha1.Options.SecretKeyRef.new(%{name: "1", key: "@"})}
        }),
        Bossman.Protobuf.V1alpha1.Options.EnvFrom.new(%{
          name: "praveen",
          valueFrom: {:configMapKeyRef, %{name: "1", key: "@"}}
        })
      ]
    )
    |> IO.inspect()
    |> Bossman.Protobuf.V1alpha1.Options.encode()
  end
end
