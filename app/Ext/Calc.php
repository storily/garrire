<?php

declare(strict_types=1);
namespace Ext;

class Calc
{
	function __construct() {
		$this->calc = \FFI::load(__DIR__ . "/calc.h");
	}

	public function eval(string $expr): string
	{
		$out = $this->calc->eval($expr);

		$string = \FFI::string($out);
		$this->calc->free_output($out);

		if (\str_starts_with($string, 'err:')) {
			throw new \Exceptions\ExtCalc(substr($string, 4));
		} else {
			return $string;
		}
	}

	private static $instance;
	public static function init(): self
	{
		return static::$instance ??= new static;
	}
}
