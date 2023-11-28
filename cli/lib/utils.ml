(*
let is_debug = false;;
let debug ?(prefix="") x = if is_debug then Printf.printf "%s %s\n" prefix x;;

let def_delay = 0.5;;
let off_all = Protcol.Msg.of_msg ~mask:0 ~mode: 3 ~red: 0 ~green: 0 ~blue: 0 ~relay: false;;
let off_ring = Protcol.Msg.of_msg ~mask:0 ~mode: 2 ~red: 0 ~green: 0 ~blue: 0 ~relay: false;;
let off_status = Protcol.Msg.of_msg ~mask:0 ~mode: 1 ~red: 0 ~green: 0 ~blue: 0 ~relay: false;;
let set_status r g b = Writer.write (Protcol.Msg.of_msg ~mask: 0 ~mode: 1 ~red: r ~green: g ~blue: b ~relay: false)
let set_ring r g b = Writer.write (Protcol.Msg.of_msg ~mask: 0 ~mode: 2 ~red: r ~green: g ~blue: b ~relay: false)
let set_ring_greyscale s = Writer.write (Protcol.Msg.of_msg ~mask: 0 ~mode: 2 ~red: s ~green: s ~blue: s ~relay: false)

let rec char_list_of_string x =  
        match x with
        | [] -> []
        | x :: [] -> x :: (char_list_of_string [])
        | x :: tl -> x :: (char_list_of_string tl)

let def_color = (20,  20,  20);;

let def_if_no_color (r, g, b) = 
        match r, g, b with
                | 0, 0, 0 -> def_color
                | _, _, _ -> (r, g, b)
        *)

let n_sized_list_of_item a size =  
        let rec inner a size acum =  
        match size with 
        | 0 -> acum 
        | s -> inner a (s - 1) ( a :: acum)in
        inner  a size []
