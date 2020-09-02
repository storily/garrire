<?php

namespace App\Controllers\Command;

class Roll extends \App\Controller
{
  public function post(): void
  {
    if (!preg_match('/(\d*)d(\d+)/i', $this->argument(), $matches)) {
      $this->redirect('/command/help');
    }

    $rolls = ((int) $matches[1]) ?: 1;
    $sides = ((int) $matches[2]) ?: 6;

    $throws = [];
    for ($i = 0; $i < $rolls; $i += 1) {
      $throws[] = rand(1, $sides);
    }

    $total = array_sum($throws);
    $this->reply_once(implode(' ', array_map(
      fn ($throw) => "**$throw**",
      $throws
    )) . ' = ' . $total);
  }
}
