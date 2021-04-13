<?php

declare(strict_types=1);
namespace Controllers\ApiEndpoint;

class Plot extends \Controllers\ApiController
{
	public function get(): void
	{
		$q = \Models\Plot::query();

		$theme = $_GET['theme'] ?? null;
		if ($theme !== null) {
			$theme = strtolower(trim($theme));
			$q = $q->where('theme', 'LIKE', "%{$theme}%");
		}

		// number of plots: minimum 1, maximum 5, default 1
		$count = max(1, min(5, intval($_GET['n']) ?? 1));
		$plots = $q->inRandomOrder()->take($count)->get();

		$result = [];
		foreach ($plots as $plot) {
			$result[] = [
				"id" => $plot->id,
				"text" => $plot->text,
				"author" => $plot->author,
				"theme" => $plot->theme,
			];
		}

		$this->reply($result);
	}
}
