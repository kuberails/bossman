defmodule ProtobufTest do
  use ExUnit.Case, async: true
  alias Bossman.Protobuf.V1alpha1.Options

  test "options work" do
    struct =
      Options.new(
        timeout: %{value: 1},
        retries: %{value: 2},
        env_from: [%{env_from: {:secret_key_ref, %{name: "1"}}}],
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
              {:value_from,
               %{
                 name: "praveen",
                 value_from: {:secret_key_ref, %{name: "1", key: "@"}}
               }}
          },
          %{
            env:
              {:value_from,
               %{
                 name: "praveen",
                 value_from: {:config_map_key_ref, %{name: "1", key: "@"}}
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
