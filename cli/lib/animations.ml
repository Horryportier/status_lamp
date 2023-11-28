(* this will now not work fully cuz of linux pipe overfolwing  *)
let bin_counter r g b = 
        let open Utils in
        let paylod = Protcol.Msg.of_msg ~mask: 0 ~mode: 0 ~red: r ~green: g ~blue: b  ~relay: false in
                for i = 0 to 65536 do 
                        paylod.mask <- i;
                        Writer.write paylod;
                        set_status r g b;
                        Unix.sleepf def_delay;
                        Writer.write off_all;
                done
                
