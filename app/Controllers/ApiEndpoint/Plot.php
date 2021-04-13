<?php

declare(strict_types=1);
namespace Controllers\ApiEndpoint;

class Plot extends \Controllers\ApiController
{
	public function get(): void
	{
		$id = intval($_GET['id']) ?? null;
		if ($id === null) throw new \Exceptions\End(404);

		$plot = \Models\Plot::query()->where('id', '=', $id)->first();
		if ($plot === null) throw new \Exceptions\End(404);

		$this->reply([
			"id" => $plot->id,
			"text" => $plot->text,
			"author" => $plot->author,
			"theme" => $plot->theme,
		]);
	}
}
