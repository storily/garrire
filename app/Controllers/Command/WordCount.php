<?php

/// wc (wordcount) - Get your wordcount from the nano website.
///
/// You need to set your novel "ID" to setup. When you go to your
/// your nanowrimo pages and open the main page for your project,
/// the URL will look like `https://nanowrimo.org/participants/your-name/projects/some-name`.
/// Your project ID is the `some-name` part. Copy just that and tell
/// me about it with `!wc some-name`.
///
/// Thereafter, get your wordcount with `!wc`.

declare(strict_types=1);
namespace Controllers\Command;

class WordCount extends \Controllers\Controller
{
	public function post(): void
	{
		$this->help();

		$userid = $_SERVER['HTTP_ACCORD_AUTHOR_ID'] ?? $_SERVER['HTTP_ACCORD_USER_ID'] ?? null;
		if (!$userid) throw new \Exception('no user id, cannot proceed?!');

		if (!empty($arg = $this->argument())) {
			\Models\Novel::updateOrCreate(['discord_user_id' => $userid], ['novel' => $arg]);
		}

		$novel = \Models\Novel::where('discord_user_id', $userid)->first();
		if (!$novel) {
			$this->show_help();
			return;
		}

		$title = $novel->title();
		$count = $novel->wordcount();

		$this->reply("“{$title}”: **{$count}** words", null, true);
	}
}
