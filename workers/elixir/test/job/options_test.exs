defmodule Bossman.Job.OptionsTest do
  use ExUnit.Case, async: true
  alias Bossman.Job.Options
  alias Bossman.Protobuf.V1alpha1.Options, as: ProtoOptions

  describe "encode" do
    test "encodes empty options properly" do
      res =
        Options.new()
        |> Options.encode()
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
  end
end
