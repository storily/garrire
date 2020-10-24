<?php

declare(strict_types=1);
namespace Controllers;

class Check extends Controller
{
	public function double_bang(): void
	{
		// TODO: get prefix from env/config and use that
		if (in_array($this->payload['content'] ?? null, ['~~', '!!'])) {
			$this->redirect("/command/w");
		} else {
			$this->end(404);
		}
	}
}
