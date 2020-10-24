<?php

declare(strict_types=1);
namespace Acts;

class AssignRole extends Act
{
	function __construct(int $role_id, int $user_id, ?int $server_id = null, ?string $reason = null)
	{
		$this->kind = 'assign-role';
		$this->body = array_filter([
			'role_id' => $role_id,
			'user_id' => $user_id,
			'server_id' => $server_id,
			'reason' => $reason,
		]);
	}
}
