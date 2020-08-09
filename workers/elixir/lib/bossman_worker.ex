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
        %Bossman.Protobuf.V1alpha1.Options.Env{
          _env:
            {:_value,
             Bossman.Protobuf.V1alpha1.Options.EnvValue.new(%{name: "hello", value: "world"})}
        },
        %Bossman.Protobuf.V1alpha1.Options.Env{
          _env:
            {:_valueFrom,
             Bossman.Protobuf.V1alpha1.Options.EnvFrom.new(%{
               name: "praveen",
               valueFrom:
                 {:secretKeyRef,
                  Bossman.Protobuf.V1alpha1.Options.SecretKeyRef.new(%{name: "1", key: "@"})}
             })}
        }
      ]
    )
    |> IO.inspect(label: "Structs")
    |> Bossman.Protobuf.V1alpha1.Options.encode()
    |> IO.inspect(label: "Binary")
    |> Bossman.Protobuf.V1alpha1.Options.decode()
  end
end
