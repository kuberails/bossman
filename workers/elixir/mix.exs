defmodule BossmanWorker.MixProject do
  use Mix.Project

  def project do
    [
      app: :bossman_worker,
      version: "0.1.0",
      elixir: "~> 1.10",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # grpc
      {:grpc, github: "elixir-grpc/grpc"},
      {:cowlib, "~> 2.9.0", override: true},

      # protobuf
      {:protobuf, "~> 0.7.1"},
      {:google_protos, "~> 0.1"},

      # utils
      {:typed_struct, "~> 0.2.1"},

      # testing
      {:mix_test_watch, "~> 1.0", only: :dev}
    ]
  end
end
