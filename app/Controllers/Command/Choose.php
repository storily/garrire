<?php

/// choose - Choose between several items (separated by `or`).
///
/// Separate items by the word `or`, or with commas. One will be chosen at random.

declare(strict_types=1);
namespace Controllers\Command;

class Choose extends \Controllers\Controller
{
  public function post(): void
  {
    $this->help();
    if (empty($arg = $this->argument())) $this->show_help();

    $items = array_map(
      fn ($item) => trim($item),
      preg_split('/\s+or\s+/i', $arg)
    );

	if (str_contains($arg, ",") && count($items) == 1) {
		$items = array_map(
		  fn ($item) => trim($item),
		  preg_split('/,\s*/i', $arg)
		);
	}

	if (in_array('write', $items)) $items[] = 'write';

	if (count($items) == 1) {
		$this->reply((rand(0, 1) ? 'yes' : 'no'), null, true);
	} else {
	    $this->reply($items[array_rand($items)], null, true);
	}
  }
}
