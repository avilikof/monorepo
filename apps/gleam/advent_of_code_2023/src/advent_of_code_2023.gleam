import gleam/io
import gleam/result.{Error, Ok}

type IoDevi
 fn open(
  path: String,
  mode: String,
) -> Result(IoDevice, String) =
  "file" "open"

pub external fn read_line(
  device: IoDevice
) -> Result(String, String) =
  "file" "read_line"

pub external fn close(
  device: IoDevice
) -> Result(Nil, String) =
  "file" "close"

pub fn read_file_line_by_line(path: String) -> Result(List(String), String) {
  case open(path, "r") {
    Ok(device) -> read_lines(device, [])
    Error(reason) -> Error(reason)
  }
}

fn read_lines(device: IoDevice, lines: List(String)) -> Result(List(String), String) {
  case read_line(device) {
    Ok(line) ->
      if line == eof() {
        Ok(List.reverse(lines))
      } else {
        read_lines(device, [line | lines])
      }
    Error(reason) -> Error(reason)
  }
}

fn eof() -> String {
  "eof"
}
