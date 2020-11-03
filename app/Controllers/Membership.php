<?php

declare(strict_types=1);
namespace Controllers;

class Membership extends Controller
{
	public function join(): void
	{
		dump($this->payload);

		$role = (int) \Models\NanowrimoSetting::where('name', 'person-role')->first()->value;

		$this->assign_role(
			$role,
			$this->payload['user']['id'],
			$this->payload['server_id'],
			'Automatic upon entry (garrīre)'
		);
		$this->end();
	}
}
