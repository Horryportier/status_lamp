> tldr: cli for stutus lamp
# TODO:
- [x] set up cil commands  
- [x] read json data with animation and execute it  
- [ ] figure out pipe overfolw bug 
    -  find and way to do it with using  (stty -F %s -hup -crtscts)
        hacky way that overflows serial file but,
        allows arduino to not restart at opening of the port
    -  run without it and make this tool an backrond task with socket for coms (might not solve overflow issue)
        would have to run all the time but allows for control of the flow of data 
        checking for correct data and clamiping thins like minimal delay        
        Would be fun to do. 
- [ ]  create socket lisener @(Current Goal)
    
