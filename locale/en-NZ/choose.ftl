both = { CHOOSE(5) ->
    *[1] both
    [2] yes
    [3] all of the above
    [4] not super sure, actually
    [5] Gryffindor!
}

yes = { CHOOSE(5) ->
    *[1] yes
    [2] yep
    [3] this one
    [4] absolutely
    [5] go for it
}

no = { CHOOSE(5) ->
    *[1] no
    [2] nope
    [3] no way
    [4] nah
    [5] yeah nah
}