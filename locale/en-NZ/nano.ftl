count = {$username}: { $count ->
    [0] zero words
    [1] a single lonely word
    *[more] **{$count}** words
}{ $palindromic ->
    *[0] {""}
    [1] {" nice!"}
}

count-list = {$username}: `{$counts}`

error = {$username}: sorry! `{$detail}`

no-nano-user =
    Looks like I don’t have your nano user on file!
    Set it up with: `{$prefix}my nano <username>`
