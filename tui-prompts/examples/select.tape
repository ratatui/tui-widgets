# charm/vhs script to create a gif of the select example
# run with `vhs select.tape` then `vhs publish select_prompt.gif`
# for docs run `vhs new docs.tape`

Output "target/select_prompt.gif"

Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600

Hide
Type "cargo -q run --example=select -- --debug"
Enter
Wait+Screen /Select a fruit:/
Show
Sleep 1s
Down
Sleep 400ms
Down
Enter
Wait+Screen /Selected: Cherry/
Sleep 2s
Hide
Escape
Wait+Line@5s />$/
