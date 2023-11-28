(*
open Arg
type arguments = {
        input_file: string;
        msg: Protcol.Msg.t;
        preset: string;
} [@@deriving yojson, show]


let to_string x = arguments_to_yojson x |> Yojson.Safe.pretty_to_string 
let print x = x |> Printf.printf "%s"


let usage_msg  = "status_lamp [-input] <file> [-mask] <mask> [-mode] <mode> [-r] <red> [-g] <green> [-b] <blue> [-preset] <preset>"

let mask = ref "0"
let mode = ref "0"
let red = ref "0"
let green = ref "0"
let blue = ref "0"
let relay = ref false
let input_file = ref ""
let preset = ref ""

let anon_fun file_name = input_file := file_name

let spec_list = 
        [
                ("-mask", Set_string mask, "bitmask for ring strip");
                ("-mode", Set_string mode, "mode deciading how to handle data");
                ("-red", Set_string red, "red part of color");
                ("-green", Set_string green, "green part of color");
                ("-blue", Set_string blue, "blue part of color");
                ("-blue", Set_string blue, "blue part of color");
                ("-relay", Set relay, "relay on or of mode 5 for relayA mode 6 for relayB");
                ("-input", Set_string input_file, "input file containing animation");
                ("-preset", Set_string preset, "one of predefined animations exp. \"off\", \"binary\""); 
        ]

let to_arguments input_file preset mask mode red green blue relay = 
        let msg = Protcol.Msg.of_msg  ~mask ~mode ~red ~green ~blue ~relay in 
        { input_file ; msg; preset}

let parse = parse spec_list anon_fun usage_msg; 
        let ifs = int_of_string in
        to_arguments !input_file !preset (ifs !mask) (ifs !mode) (ifs !red) (ifs !green) (ifs !blue) !relay;;

exception NOT_VALID_PRESSET;;
exception NOT_VALID_JSON of string;;

let eval arguments = 
        Utils.debug ?prefix: (Some "arguments: ") (to_string arguments); 
        if "" != arguments.input_file then
                let file = Json.JsonMsg.of_yojson (Yojson.Safe.from_file arguments.input_file) in
                match file with
                | Ok msg -> (Utils.debug ?prefix: (Some "file content: ") (Yojson.Safe.to_string ( Json.JsonMsg.to_yojson msg))); (Json.JsonMsg.run msg)
                | Error err -> raise (NOT_VALID_JSON err)  
        else if "" != arguments.preset  then
               let (r, g, b) = Utils.def_if_no_color (arguments.msg.red, arguments.msg.green, arguments.msg.blue) in
               match arguments.preset with
               | "off" -> Writer.write Utils.off_all
               | "bin" -> Animations.bin_counter r g b
               | "10" -> Utils.set_ring_greyscale 25 
               | "20" -> Utils.set_ring_greyscale 51
               | "30" -> Utils.set_ring_greyscale 76
               | "40" -> Utils.set_ring_greyscale 102
               | "50" -> Utils.set_ring_greyscale 127
               | "60" -> Utils.set_ring_greyscale 153
               | "70" -> Utils.set_ring_greyscale 178
               | "80" -> Utils.set_ring_greyscale 204
               | "90" -> Utils.set_ring_greyscale 229
               | "100" -> Utils.set_ring_greyscale 255
               | _ -> raise NOT_VALID_PRESSET
        else
           Writer.write arguments.msg;;
        *)
