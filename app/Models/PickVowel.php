<?php

declare(strict_types=1);
namespace Models;

class PickVowel extends HardCodedPick
{
	static protected function picks(): array
	{
		return ['A', 'E', 'I', 'O', 'U', 'Y'];
	}
}
