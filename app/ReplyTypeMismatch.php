<?php

namespace App;

class ReplyTypeMismatch extends \Exception
{
  public function __construct(string $sent, string $wanted)
  {
    parent::__construct("Requested $wanted but $sent has already been sent");
  }
}
