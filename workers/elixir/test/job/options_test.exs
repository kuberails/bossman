defmodule Bossman.Job.OptionsTest do
  use ExUnit.Case, async: true
  alias Bossman.Job.Options
  alias Bossman.Protobuf.V1alpha1.Options, as: ProtoOptions

  describe "encode" do
    test "encodes empty options properly" do
      res =
        Options.new()
        |> Options.encode!()
        |> ProtoOptions.encode()
        |> ProtoOptions.decode()

      assert res == %Bossman.Protobuf.V1alpha1.Options{
               annotations: %{},
               args: [],
               command: [],
               completions: %Google.Protobuf.Int32Value{value: 1},
               env: [],
               env_from: [],
               image_pull_secrets: nil,
               namespace: nil,
               parallelism: %Google.Protobuf.Int32Value{value: 1},
               retries: %Google.Protobuf.Int32Value{value: 5},
               timeout: nil
             }
    end

    test "encodes with env value" do
      options = %{namespace: "not-default", env: [%{name: "TEST", value: "test1"}]}

      res =
        options
        |> Options.new()
        |> Options.encode!()
        |> ProtoOptions.encode()
        |> ProtoOptions.decode()

      assert res == %Bossman.Protobuf.V1alpha1.Options{
               annotations: %{},
               args: [],
               command: [],
               completions: %Google.Protobuf.Int32Value{value: 1},
               env: [
                 %Bossman.Protobuf.V1alpha1.Options.Env{
                   env:
                     {:value,
                      %Bossman.Protobuf.V1alpha1.Options.Env.EnvValue{
                        name: "TEST",
                        value: "test1"
                      }}
                 }
               ],
               env_from: [],
               image_pull_secrets: nil,
               namespace: %Google.Protobuf.StringValue{value: "not-default"},
               parallelism: %Google.Protobuf.Int32Value{value: 1},
               retries: %Google.Protobuf.Int32Value{value: 5},
               timeout: nil
             }
    end

    test "encode and decode with default options" do
      res =
        Options.new()
        |> Options.encode!()
        |> ProtoOptions.encode()
        |> ProtoOptions.decode()
        |> Options.decode!()

      assert res == Options.new()
    end

    test "encode and decode with envs and env vara" do
      options = %{
        timeout: 800,
        retries: 10,
        namespace: "not-default",
        env: [
          %{name: "TEST", value: "test1"},
          %{
            type: :secret_key_ref,
            name: "ENV_NAME",
            secret_name: "secret-key",
            secret_key: "SECRET_ENV_NAME"
          },
          %{
            type: :config_map_key_ref,
            name: "ENV_NAME",
            config_map_name: "secret-key",
            config_map_key: "SECRET_ENV_NAME"
          }
        ],
        env_from: [
          %{type: :secret_key_ref, name: "secret-key-ref-name"},
          %{type: :config_map_key_ref, name: "config-map-key-ref-name"}
        ]
      }

      res =
        options
        |> Options.new()
        |> Options.encode!()
        |> ProtoOptions.encode()
        |> ProtoOptions.decode()
        |> Options.decode!()

      assert res == Options.new(options)
    end
  end
end
