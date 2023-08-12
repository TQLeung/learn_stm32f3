target remote :3333
set print asm-demangle on
set print pretty on
set style sources off
monitor tpiu config internal itm.txt uart off 8000000
monitor itm port 0 on
load
define hook-quit
  set confirm off
end
break main
break DefaultHandler
break HardFault
continue
