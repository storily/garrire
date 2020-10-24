<?php

declare(strict_types=1);
namespace Acts;

class CreateMessage extends Act
{
	function __construct(string $content, int $channel_id = null)
	{
		$this->kind = 'create-message';
		$this->body = array_filter([
			'content' => $content,
			'channel_id' => $channel_id,
		]);
	}
}
