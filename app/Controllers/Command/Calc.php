<?php

/// calc (=) - Do some math.
///
/// Does integer maths if you only pass integers, which can
/// be a little surprising. Generally this manifests when
/// you do `1 / 5` and expect `0.2`, but I'll tell you that's
/// just plain `0` instead. Surprise! Use `1.0 / 5` instead.

declare(strict_types=1);
namespace Controllers\Command;

class Calc extends \Controllers\Controller
{
	public function post(): void
	{
		$this->help();
		if (empty($arg = $this->argument())) $this->show_help();

		try {
			$out = \Ext\Calc::init()->eval($arg);
		} catch (\Exceptions\ExtCalc $err) {
			$out = $err->getMessage();
		} catch (\Throwable $err) {
			$out = 'unexpected error';
		}

		$this->reply($out, null, true);
	}
}
