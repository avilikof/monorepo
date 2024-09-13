import gleam/io
import gleam/result.{Result, Ok, Error}
import gleam/erlang
import gleam/string

pub type NatsClient {
  NatsClient(stream: pid)
}

// Function to connect to a NATS server and create a NatsClient
pub fn connect(url: String) -> Result(NatsClient, String) {
  case erlnats:start_link(erlnats:nats_options(url: url)) {
    Ok(pid) -> Ok(NatsClient(pid))
    Error(reason) -> Error(reason |> erlang:term_to_string)
  }
}

// Function to publish a message to a subject
pub fn publish(client: NatsClient, subject: String, data: String) -> Result(Nil, String) {
  let NatsClient(stream) = client
  case erlnats:publish(stream, subject, data) {
    Ok(()) -> Ok(Nil)
    Error(reason) -> Error(reason |> erlang:term_to_string)
  }
}

// Function to subscribe to a subject
pub fn subscribe(client: NatsClient, subject: String, handler: fn(String) -> Nil) -> Result(Nil, String) {
  let NatsClient(stream) = client
  case erlnats:subscribe(stream, subject, fn(_sub, msg) {
    let data = string:from_utf8(msg)
    handler(data)
    Ok(Nil)
  }) {
    Ok(()) -> Ok(Nil)
    Error(reason) -> Error(reason |> erlang:term_to_string)
  }
}