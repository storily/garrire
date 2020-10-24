<?php

declare(strict_types=1);
namespace Acts;

class Act
{
	protected string $kind;
	protected array $body;

	public function __toString(): string
	{
		return json_encode([ $this->kind => $this->body ]);
	}

	public function send(): void
	{
		echo "$this" . str_repeat(' ', 4096) . "\n";
	}
}
