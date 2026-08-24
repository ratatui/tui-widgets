# VHS Tape (see https://github.com/charmbracelet/vhs)
# Non-ASCII font demo
Output "target/non_ascii.gif"
Set Theme "Aardvark Blue"
Set Width 1000
Set Height 420
Hide
Type@0 "cargo run -p tui-big-text --example non_ascii --quiet"
Enter
Sleep 2s
Show
Screenshot "target/non_ascii.png"
Sleep 1s
Hide
Escape
