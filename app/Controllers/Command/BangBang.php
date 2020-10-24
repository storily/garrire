<?php

/// w - Your current status. (shorthand: `!!`)
///
/// (WIP) Gives out both your current wordcount and the status
/// of any wordwars ongoing or starting soon, in one easy command.

declare(strict_types=1);
namespace Controllers\Command;

class BangBang extends \Controllers\Controller
{
	const HELP_NAME = 'w';

	public function post(): void
	{
		$this->command ??= $this->payload['content'][0];
		$this->help();

		$this->reply('work in progress', null, true);
	}
}
