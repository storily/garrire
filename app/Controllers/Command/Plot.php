<?php

/// plot (prompt) - Random plots and prompts.

declare(strict_types=1);
namespace Controllers\Command;

class Plot extends \Controller
{
	public function post(): void
	{
		$this->help();
		if (empty($this->argument())) $this->show_help();

		$plot = \Models\Plot::query()
			->inRandomOrder()
			->first();

		$this->reply(
			sprintf("> %s\n— %s [**%s**]", $plot->text, $plot->theme, $plot->author),
			null,
			true,
		);
	}
}
