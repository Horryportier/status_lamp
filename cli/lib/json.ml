(*
module JsonMsg = struct 
        type t = { 
                delay: float;
                repeating: bool;
                animation: Protcol.Msg.t list;
        } [@@deriving yojson, show]

let run json_msg =  
        Utils.debug "Run func:";
        let (animation, delay, repeating) = (json_msg.animation, json_msg.delay, json_msg.repeating) in
        match repeating with 
        | true -> while true do List.iter (fun  x -> Writer.write x; Unix.sleepf delay ) animation done 
        | false -> List.iter (fun  x -> (Utils.debug ?prefix: (Some " current msg: ") (Protcol.Msg.to_string x)); Writer.write x; Unix.sleepf delay ) animation
end
        *)
