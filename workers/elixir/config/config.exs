import Config

config :grpc, start_server: true

import_config "#{Mix.env()}.exs"
