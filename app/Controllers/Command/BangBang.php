<?php

/// w - Your current status. (shorthand: `!!`)
///
/// (WIP) Gives out both your current wordcount ~~and the status
/// of any wordwars ongoing or starting soon~~ (tbd), in one easy command.

declare(strict_types=1);
namespace Controllers\Command;

class BangBang extends \Controllers\Controller
{
	const HELP_NAME = 'w';

	public function post(): void
	{
		$this->command ??= $this->payload['content'][0];
		$this->help();

		$userid = $_SERVER['HTTP_ACCORD_AUTHOR_ID'] ?? $_SERVER['HTTP_ACCORD_USER_ID'] ?? null;
		if (!$userid) throw new \Exception('no user id, cannot proceed?!');

		$novel = \Models\Novel::where('discord_user_id', $userid)->first();
		if (!$novel) {
			$this->reply('work in progress (aka you have no novels)', null, true);
			return;
		}

		$title = $novel->title();
		$count = $novel->wordcount();

		$this->reply("“{$title}”: **{$count}** words", null, true);
	}
}
