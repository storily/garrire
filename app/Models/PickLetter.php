<?php

declare(strict_types=1);
namespace Models;

class PickLetter extends HardCodedPick
{
	static protected function picks(): array
	{
		return range('A', 'Z');
	}
}
