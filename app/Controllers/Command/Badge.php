<?php

/// badge (badges, role, roles) - Confers or removes badges (roles) on request.
///
/// Two types of badges are available: locations and pronouns.
/// Get a list with `badge list`, or look around to what people have.
///
/// To get a badge, call `badge get badgename`, and to remove a
/// badge call `badge leave badgename`.

declare(strict_types=1);
namespace Controllers\Command;

class Badge extends \Controllers\Controller
{
	public function post(): void
	{
		$this->help();
		if (empty($arg = $this->argument())) $this->show_help();

		$args = preg_split('/\s+/', strtolower($arg));

		$action = $args[0] ?? null;
		$badgename = $args[1] ?? null;

		switch ($action) {
			case 'list':
				$this->reply(
					\Models\Badge::query()
					->orderBy('name', 'asc')
					->get()
					->groupBy('kind')
					->map(fn ($badges, $kind) => "{$kind}s: " . $badges
						->map(fn ($b) => "**{$b->name}**")
						->implode(', ')
					)
					->implode("\n")
				);
			break;

			case 'get':
			case 'add':
				$badge = \Models\Badge::where('name', 'LIKE', "%{$badgename}%")->first();
				if ($badge) {
					$this->assign_role(
						$badge->role_id,
						$this->payload['author']['user']['id'],
						507442119724630036 ?? $this->payload['server_id'],
						'Asked to get a badge (garrīre)',
					);
					$this->reply('You got it!');
				} else {
					$this->reply('No such badge!');
				}
			break;

			case 'leave':
			case 'rm':
			case 'remove':
				$badge = \Models\Badge::where('name', 'LIKE', "%{$badgename}%")->first();
				if ($badge) {
					$this->remove_role(
						$badge->role_id,
						$this->payload['author']['user']['id'],
						507442119724630036 ?? $this->payload['server_id'],
						'Asked to remove a badge (garrīre)',
					);
					$this->reply('I got it.');
				} else {
					$this->reply('No such badge!');
				}
			break;

			default:
				$this->show_help();
		}

		$this->end();
	}
}
