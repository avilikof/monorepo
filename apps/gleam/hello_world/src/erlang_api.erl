-module(erlang_api).
-export([fetch/1]).

fetch(Url) ->
    inets:start(),
    case httpc:request(get, {Url, []}, [], []) of
        {ok, {{_, 200, _}, _, Body}} ->
            {ok, binary_to_list(Body)};
        {ok, {{_, StatusCode, _}, _, _}} ->
            {error, "Status code: " ++ integer_to_list(StatusCode)};
        {error, Reason} ->
            {error, "Request failed: " ++ atom_to_list(Reason)}
    end.