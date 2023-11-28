module Msg = struct
type t =  {
        mutable mode: int;
        mutable mask: int;
        mutable red: int;
        mutable green: int;
        mutable blue: int;
        mutable relay: bool;
} [@@deriving yojson, show] 

let of_msg ~mode ~mask ~red ~green ~blue ~relay= 
                {mode; mask; red; green; blue; relay}

let to_string x =  to_yojson x |> Yojson.Safe.to_string
end


