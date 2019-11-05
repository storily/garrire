count = {$username}: { $count ->
    [0] kahore kupu
    [1] kupu matua
    *[more] **{$count}** kupu
}

count-list = {$username}: `{$counts}`

error = {$username}: mihi ki te raru! `{$detail}`

no-nano-user =
    Ahau kaore to ingoa nanowrimo!
    Hei whakatu i to ingoa, mea atu: `{$prefix}my nano <username>`
