help =
    {$commandCount ->
        [one] Command
        *[other] Commands
    }: {$commandList}.
    Also use `{$prefix}<command> help` to get help for any command.

help-syntax = In individual command help, `<something>` means a mandatory word/thing, `[another]` means it's optional, and `...` means there can be more, in the same pattern.

eightball = Usage: `{$prefix}eightball [question]`
calc = Usage: `{$prefix}calc <math...>`
choose = Usage: `{$prefix}choose <first thing> or <second thing> ...`
colour = Usage: `{$prefix}colour [amount]`
motivate = Usage: `{$prefix}motivate`
pick = Usage: `{$prefix}pick <start> <end>` — Picks a number between start and end
ping = Usage: `{$prefix}ping [message]`
plot = Usage: `{$prefix}plot` — Get a random plot
roll =
    Usage: `{$prefix}roll <die> ...` – `<die>` is of the form `<n>d<m>±<o>`, where `n` is the amount to roll (can be omitted), `m` is the sides, and `o` is the offset to add/remove (can be omitted).
    E.g. `3d6` rolls three six-sided dice, `d100+100` gets a roll between 101 and 200, `d24 d7` could be used to roll for an hour and day of the week. “Partial” rolls and offsets are possible: `0.7d20` rolls a d20 and multiplies it by 0.7, `4.3d9+8.6` rolls five d9 and multiplies the last one by 0.3, then adds 8.6 to each roll.
wordcount =
    Usage: `{$prefix}wc <your nano username>`
    Counts for all your events: `{$prefix}wc <nano username> list`
    Your username is the one in the URL when you visit your profile, after `participants/`.
    The bot cannot yet remember your nano username (you might remember that from last year); that's coming... soonish.
