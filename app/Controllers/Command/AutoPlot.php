<?php

/// autoplot (ap) - Randomly-generated plots.
///
/// Use `!autoplot GENRE` where `GENRE` is one of `drama`, `fantasy`, `mystery`, `romance`, `sci-fi`
/// for plots in the style of "It's a story about X with Y and Z characters."
///
/// Use `!autoplot triple` for "pick three elements" style plots.
///
/// Put a number in front of your selection like `!autoplot 3 sci-fi` to get more than one plot.
/// Call with nothing to get three plots in random styles.

declare(strict_types=1);
namespace Controllers\Command;

class AutoPlot extends \Controllers\Controller
{
	public function post(): void
	{
		$this->help();
		$arg = $this->argument();

		$plots = [];
		if (empty($arg)) {
			$n = 3;
			$g = 'random';
		} else if (is_numeric($arg)) {
			$n = (int) $arg;
			$g = 'random';
		} else {
			$items = array_map(
			  fn ($item) => trim($item),
			  preg_split('/,\s*/i', $arg)
			);
			[$n, $g] = match (count($items)) {
				1 => [1, $items[0]],
				2 => [(int) $items[0], $items[1]],
				default => $this->show_help(),
			};
		}

		if ($n < 1) $n = 1;
		if ($n > 10) $n = 10;

		foreach (range(1, $n) as $n) {
			$plot = match ($g) {
				'random' => static::random(),
				'triple' => static::triple(),
				default => static::storied($g),
			};

			$this->reply($plot);
		}
	}

	public static function random(): string
	{
		$set = [
			['storied', 'drama'],
			['storied', 'fantasy'],
			['storied', 'mystery'],
			['storied', 'romance'],
			['storied', 'sci-fi'],
			// ['triple', null],
		];
		shuffle($set);
		[$fn, $arg] = $set[0];

		if ($arg) return static::$fn($arg);
		else return static::$fn();
	}

	public static function storied(string $genre): string
	{
		$protag = static::storied_cat($genre, "protagonists");
		$second = static::storied_cat($genre, "secondary_character");
		$who_one = static::storied_cat($genre, "who_one");
		$who_two = static::storied_cat($genre, "who_two");
		$agenre = static::storied_cat($genre, "genre");
		$about = static::storied_cat($genre, "about");
		$begins = static::storied_cat($genre, "begins");
		$plot = static::storied_cat($genre, "plot");
		$note = static::storied_cat($genre, "note");
		$twist = static::storied_cat($genre, "twist");

		return implode("\n", [
			"**Protagonist**: {$protag}, who {$who_one}",
			"**Secondary character**: {$second}, who {$who_two}",
			"**Plot**: It's {$agenre} story about {$about} It kicks off {$begins} with {$plot}",
			"_(Note that: {$note})_",
			"And there's a **twist**! {$twist}",
		]);
	}

	private static function storied_cat(string $genre, string $cat): string
	{
		return \Models\PlotStoried::query()
			->where("genre", $genre)
			->where("category", $cat)
			->inRandomOrder()
			->first()
			->value;
	}

	public static function triple(): string
	{
		$genre = "generic";
		return implode(' ', [
			static::triple_one($genre, "place"),
			static::triple_one($genre, "class"),
			static::triple_one($genre, "time"),
		]);
	}

	private static function triple_one(string $genre, string $kind): string
	{
		return \Models\PlotTriple::query()
			->where("genre", $genre)
			->where("kind", $kind)
			->inRandomOrder()
			->first()
			->value;
	}
}
