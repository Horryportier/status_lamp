module Msg = struct
        type color_data = {
                mutable r: int; 
                mutable g: int; 
                mutable b: int; 
        } [@@deriving yojson, show]
        let color_data r g b = {r; g; b}

        type  pin_data = {
                mutable pin: int; 
                mutable value: int; 
                mutable analog: bool; 
        } [@@deriving yojson, show]
        let pin_data ?(analog=false) pin value   = {pin; value; analog}


        type msg_kind = 
                | Pixel_Strip of color_data list * int
                | Center_Led of color_data
                | Pin of pin_data [@@deriving yojson, show]

        let def_msg_kind = function
                | "ps" -> Pixel_Strip ((color_data 20 20 20) :: [], 16)
                | "cl" -> Center_Led (color_data 20 20 20)
                | "p" -> Pin (pin_data 2 1)
                | _ -> Center_Led (color_data 20 0 0)

        
type t =  {
                mutable msg: msg_kind; 
} [@@deriving yojson, show] 

let of_msg msg = {msg}

let to_string x =  to_yojson x |> Yojson.Safe.to_string

end


