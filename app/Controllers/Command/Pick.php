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
/// `flower`, `food`, `colour`, `animal`, `season`, `gemstone`, `card`.
/// Plurals and some alternative spellings are also accepted.

declare(strict_types=1);
namespace Controllers\Command;

class Pick extends \Controllers\Controller
{
	private const SPECIALS = [
		'animal' => 'animal',
		'animals' => 'animal',
		'card' => 'card',
		'cards' => 'card',
		'suit' => 'card',
		'color' => 'colour',
		'colors' => 'colour',
		'colour' => 'colour',
		'colours' => 'colour',
		'feed' => 'food',
		'flower' => 'flower',
		'flowers' => 'flower',
		'food' => 'food',
		'foods' => 'food',
		'gem' => 'gemstone',
		'gemstone' => 'gemstone',
		'gemstones' => 'gemstone',
		'month' => 'month',
		'months' => 'month',
		'season' => 'season',
		'seasons' => 'season',
		// 'fruit' => 'fruit',
		// 'fruits' => 'fruit',
		// 'plant' => 'plant',
		// 'plants' => 'plant',
		// 'sport' => 'sport',
		// 'sports' => 'sport',
	];

	public function post(): void
	{
		$this->help();
		if (empty($arg = strtolower($this->argument()))) $this->show_help();
		$arg = trim(str_replace([',', ';'], ' ', $arg));

		if ($special = static::SPECIALS[$arg] ?? false) {
			$arg = "1 $special";
		}

		$matches = [];
		if (!preg_match_all(
			'/(?P<from>\d+|an?)(?:(?P<op> |\.{2,3}|-)(?P<to>\d+))?(?:\s+(?P<special>[a-z]+))?\b/i'
		, $arg, $matches)) {
			$this->reply('no match', null, true);
			return;
		}

		$pickers = [];
		foreach (array_keys($matches[0]) as $n) {
			$pickers[] = (object) [
				'from' => (int) preg_replace('/^an?$/', '1', $matches['from'][$n]),
				'op' => $matches['op'][$n],
				'to' => ($matches['to'][$n] !== '') ? ((int) $matches['to'][$n]) : null,
				'special' => static::SPECIALS[$matches['special'][$n]] ?? $matches['special'][$n],
			];
		}

		$picks = [];

		// TODO: optimise to at most one query
		foreach ($pickers as $p) {
			if ($p->to !== null) {
				$sorted = [$p->from, $p->to];
				sort($sorted);
				[$from, $to] = $sorted;
				if ($p->op == '..') $to -= 1;

				$n = rand($from, $to);
			} else {
				if ($p->special) {
					$n = $p->from;
				} else {
					$n = rand(1, $p->from);
				}
			}

			if (!empty($p->special)) {
				$model = '\\Models\\Pick'.ucfirst($p->special);
				if (class_exists($model)) {
					$q = $model::query();
				} else {
					$this->reply("unsupported category: `{$p->special}`");
					return;
				}

				foreach ($q->inRandomOrder()->take($n)->get() as $spec) {
					$picks[] = $spec->text;
				}
			} else {
				$picks[] = $n;
			}
		}

		$this->reply(implode(', ', array_map(fn ($pick) => "**$pick**", $picks)), null, true);
	}
}
