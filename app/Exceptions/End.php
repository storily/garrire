<?php

declare(strict_types=1);
namespace Exceptions;

class End extends \Exception
{
	public int $status;

	function __construct(int $status = 200)
	{
		parent::__construct('the end', $status);
		$this->status = $status;
	}
}
