import glats
import gleam/erlang/process
import gleam/result

pub fn main() {
  use conn <- result.then(glats.connect("192.168.32.161", 4222, []))

  let subject = process.new_subject()

  // Subscribe to "some.topic".
  // Messages will be delivered to the erlang subject passed in.
  let assert Ok(sid) = glats.subscribe(conn, subject, "some.topic", [])

  // Publish a single message to "some.topic".
  let assert Ok(Nil) = glats.publish(conn, "some.topic", "hello world!", [])

  // Receive from erlang subject.
  let assert Ok(glats.ReceivedMessage(
    conn: _conn,
    sid: _sid,
    status: _status,
    message: glats.Message(
      topic: _topic,
      headers: _headers,
      reply_to: _reply_to,
      body: _body,
    ),
  )) = process.receive(subject, 1000)

  // Unsubscribe from the subscription.
  let assert Ok(Nil) = glats.unsubscribe(conn, sid)

  Ok(Nil)
}
