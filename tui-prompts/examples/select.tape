Output "target/select_prompt.gif"

Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600

Hide
Type "cargo -q run --example=select -- --debug"
Enter
Show
# Wait at focused_index: 0 than move to focused_index: 1
Sleep 1s
Down
Enter
Sleep 1s
# Wait at focused_index: 0 than move to focused_index: 2
Down 2
Enter
Sleep 2s
Show
Sleep 2s
Type "This is the option selection example. The selected index changes with arrow keys and the focus state changes on selection."
Sleep 2s
Hide
Sleep 2s

