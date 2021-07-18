<?php

declare(strict_types=1);
namespace Models;

class PickConsonant extends HardCodedPick
{
	static protected function picks(): array
	{
		return [
			"B",
			"C",
			"D",
			"F",
			"G",
			"H",
			"J",
			"K",
			"L",
			"M",
			"N",
			"P",
			"Q",
			"R",
			"S",
			"T",
			"V",
			"W",
			"X",
			"Y",
			"Z",
		];
	}
}
