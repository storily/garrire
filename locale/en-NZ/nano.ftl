count = {$username}: { $count ->
    [0] zero words
    [1] a single lonely word
    *[more] **{$count}** words
}

error = {$username}: hmm, I didn’t manage to fetch that :(
