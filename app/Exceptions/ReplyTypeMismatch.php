<?php

declare(strict_types=1);
namespace Exceptions;

class ReplyTypeMismatch extends \Exception
{
  public function __construct(string $sent, string $wanted)
  {
    parent::__construct("Requested $wanted but $sent has already been sent");
  }
}
