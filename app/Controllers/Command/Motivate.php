<?php

/// motivate (motivation, advice) - Motivational messages as a service.

declare(strict_types=1);
namespace Controllers\Command;

class Motivation extends \Controllers\Controller
{
	public function post(): void
	{
		$this->help();

		$motive = \Models\Motivation::query()
			->inRandomOrder()
			->first();

		$this->reply($motive->text, null, true);
	}
}
