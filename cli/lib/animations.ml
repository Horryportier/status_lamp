open Protcol.Msg;;
(* this will now not work fully cuz of linux pipe overfolwing  *)
exception TYPE_NOT_SUPORTED_BY_ANIM of Protcol.Msg.msg_kind
 
let fill_msg msg_kind color = 
        let open Protcol.Msg in
        match msg_kind with
                | Pin p -> raise (TYPE_NOT_SUPORTED_BY_ANIM (Pin p))
                | Center_Led _ ->  Center_Led color
                | Pixel_Strip (_, count) -> Pixel_Strip (Utils.n_sized_list_of_item color count, count)

let all_lights color =
        Writer.write (of_msg (fill_msg (def_msg_kind "ps") color ));
        Writer.write (of_msg (fill_msg (def_msg_kind "cl") color )) 
