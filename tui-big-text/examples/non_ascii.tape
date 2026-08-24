# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/non_ascii.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 400
Hide
Type@0 "cargo run -p tui-big-text --example non_ascii --quiet"
Enter
Sleep 2s
Show
Screenshot "target/non_ascii.png"
Sleep 1s
Hide
Escape
