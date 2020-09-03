<?php

namespace App\Controllers\Command;

class Roll extends \App\Controller
{
  public function post(): void
  {
    $all = [];
    foreach (preg_split('/\s+/', $this->argument()) as $arg) {
      if (!preg_match('/(\d*)d(\d+)/i', $arg, $matches)) {
        $this->redirect('/command/help');
        return;
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
      )) . ' = ' . $total;
    }

    $this->reply(implode("\n", $all), null, true);
  }
}
