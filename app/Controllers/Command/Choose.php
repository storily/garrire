<?php

/// choose - Choose between several items (separated by `or`).
///
/// Separate items by the word `or`. One will be chosen at random.

declare(strict_types=1);
namespace Controllers\Command;

class Choose extends \Controller
{
  public function post(): void
  {
    $this->help();
    if (empty($arg = $this->argument())) $this->show_help();

    $items = array_map(
      fn ($item) => trim($item),
      preg_split('/\s+or\s+/i', $arg)
    );

    $this->reply($items[array_rand($items)], null, true);
  }
}
