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
