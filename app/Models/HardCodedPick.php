<?php

declare(strict_types=1);
namespace Models;

class HardCodedPick
{
	static protected function picks(): array
	{
		return [];
	}

	static public function query(): static
	{
		return new static;
	}

	public function inRandomOrder(): static
	{
		$this->selection = array_values(static::picks());
		shuffle($this->selection);
		return $this;
	}

	public function take(int $n): static
	{
		$this->selection = array_slice($this->selection, 0, $n);
		return $this;
	}

	public function get(): array
	{
		return array_map(fn ($p) => (object) ['text' => $p], $this->selection);
	}
}

