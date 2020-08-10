defmodule ProtobufTest do
  use ExUnit.Case, async: true
  alias Bossman.Protobuf.V1alpha1.Options

  test "options work" do
    struct =
      Options.new(
        timeout: %{value: 1},
        backoff_limit: %{value: 2},
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
          },
          %{
            env:
              {:valueFrom,
               %{
                 name: "praveen",
                 valueFrom: {:configMapKeyRef, %{name: "1", key: "@"}}
               }}
          }
        ]
      )

    decoded_struct =
      struct
      |> Options.encode()
      |> Options.decode()

    assert %Options{} = decoded_struct
  end
end
