let no_restart port = Printf.sprintf  "stty -F %s -hup -crtscts" port |> Unix.open_process;;

let write ?(port="/dev/ttyUSB0") ?(baut_rate=9600) msg = 
        let _ = no_restart port in
        let module SerialConfig = struct
                let port = port 
                let baud_rate = baut_rate
        end in
        let module LampSerial = Serial.Make (SerialConfig) in
        Lwt_main.run (LampSerial.write_line (Protcol.Msg.to_string msg));;


