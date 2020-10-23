<?php

/// plot (prompt) - Random plots and prompts.
///
/// You can optionally filter by category. Currently available:
/// `general`, `fantasy`, `sci-fi`, `crime`.

declare(strict_types=1);
namespace Controllers\Command;

class Plot extends \Controller
{
	public function post(): void
	{
		$this->help();

		$plot = \Models\Plot::query()
			->inRandomOrder()
			->first();

		$this->reply(
			sprintf("> %s\n— %s [**%s**]", $plot->text, $plot->author, $plot->theme),
			null,
			true,
		);
	}
}
