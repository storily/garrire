<?php

declare(strict_types=1);
namespace Controllers;

class Membership extends Controller
{
	public function join(): void
	{
		dump($this->payload);

		// (new \Acts\AssignRole(
		// 	$this->payload['server_id']
		// ))->send();
		$this->end();
	}
}
