<?php

/// roll (dice, die) - Roll some dice by tabletop notation (e.g. `3d20`).
///
/// Notation is `[N]d[M]`, where `[N]` is the number of rolls (default 1)
/// to make with an `[M]`-sided die (default 6 sides). You can roll
/// multiple dice at once, e.g. `1d20 2d10 3d5`. Sides don't have to make
/// physical sense: a `d2` is possible, as is a `d1`, as is a `d1927362`.
///
/// You can append `#` followed by some label to have your label attached
/// to the output (so you remember what it's for).

declare(strict_types=1);
namespace Controllers\Command;

class Roll extends \Controllers\Controller
{
  public function post(): void
  {
    $this->help();

	$all = [];

	[$args, $comment] = explode('#', $this->argument() ?: '', 2);
	$args = trim($args ?: 'd');
	$comment = trim($comment ?? '');

    foreach (preg_split('/\s+/', trim($args ?: 'd')) as $arg) {
      if (!preg_match('/(\d*)d(\d*)/i', $arg, $matches)) {
        $this->show_help();
      }

      $rolls = ((int) $matches[1]) ?: 1;
      $sides = ((int) $matches[2]) ?: 6;

      $throws = [];
      for ($i = 0; $i < $rolls; $i += 1) {
        $throws[] = rand(1, $sides);
      }

      $total = array_sum($throws);
      $all[] = implode(' ', array_map(
        fn ($throw) => "**$throw**",
        $throws
      )) . (count($throws) > 1 ? (' = ' . $total) : '');
    }

	if ($comment) {
		if (count($all) == 1) {
			$all = ["{$all[0]} `{$comment}`"];
		} else {
			array_unshift($all, "`{$comment}`");
		}
	}

    $this->reply(implode("\n", $all), null, true);
  }
}
