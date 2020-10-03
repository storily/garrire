<?php

/// pick - Select a number from a range, or find random items.
///
/// You can pick between two numbers using one of the following forms:
/// `1-10`, `2 20`, `3..31` (top-exclusive), `4...40` (top-inclusive).
/// If you use the forms without spaces you can specify multiple ranges
/// at once to pick simultaneously, e.g. `1-60 1-60 1-24`. If you give
/// only one number, the range starts from 1 to that number.
///
/// Specials: if you add any of the following words after a range or number,
/// or on its own, you will obtain random items matching the category: `month`,
/// `flower`, `food`, `colour`, `fruit`, `plant`, `animal`, `sport`, `season`.
/// Plurals and some alternative spellings are also accepted.

declare(strict_types=1);
namespace Controllers\Command;

class Pick extends \Controller
{
	private const SPECIALS = [
		'month' => 'month',
		'months' => 'month',
		'flower' => 'flower',
		'flowers' => 'flower',
		'food' => 'food',
		'foods' => 'food',
		'feed' => 'food',
		'colour' => 'colour',
		'colours' => 'colour',
		'color' => 'colour',
		'colors' => 'colour',
		'fruit' => 'fruit',
		'fruits' => 'fruit',
		'plant' => 'plant',
		'plants' => 'plant',
		'animal' => 'animal',
		'animals' => 'animal',
		'sport' => 'sport',
		'sports' => 'sport',
		'season' => 'season',
		'seasons' => 'season',
	];

	public function post(): void
	{
		$this->help();
		if (empty($arg = strtolower($this->argument()))) $this->show_help();

		if ($special = static::SPECIALS[$arg] ?? false) {
			$arg = "1 $special";
		}

		$matches = [];
		if (!preg_match_all(
			'/(?P<from>\d+|an?)(?:(?P<op> |\.{2,3}|-)(?P<to>\d+))?(?:\s+(?P<special>[a-z]+))?\b/i'
		, $arg, $matches)) {
			$this->reply('no match', null, true);
		}

		$pickers = [];
		foreach (array_keys($matches[0]) as $n) {
			$pickers[] = (object) [
				'from' => (int) preg_replace('/^an?$/', '1', $matches['from'][$n]),
				'op' => $matches['op'][$n],
				'to' => ($matches['to'][$n] !== '') ? ((int) $matches['to'][$n]) : null,
				'special' => $matches['special'][$n],
			];
		}

		$picks = [];
		$picks[] = var_export($pickers, true);

		// TODO: optimise to at most one query
		foreach ($pickers as $picker) {
			//
		}

		$this->reply(implode(', ', array_map(fn ($pick) => "**$pick**", $picks)), null, true);
	}
}
