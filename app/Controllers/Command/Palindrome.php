<?php

/// palindrome (pal) - Find the nearest greater palindrome number.
///
/// A palindrome number is one where digits repeat symmetrically, like
/// `12321` or `619916`. NZNano tradition is to attempt to finish a day's
/// writing on a palindrome number. Tell this command your current word
/// count and you'll see how close you are to the next one.

// TODO: With no argument, pull user's wordcount from nano.org.

namespace App\Controllers\Command;

class Palindrome extends \App\Controller
{
  public function post(): void
  {
    $this->help();
    if (empty($arg = $this->argument())) $this->show_help();

    $current = (int) $arg;
    $palindrome = static::next($current);
    $diff = $palindrome - $current;

    $this->reply("**$palindrome** – $current = $diff", null, true);
  }

  public static function is_pal(int $number): bool
  {
    $digits = str_split((string) $number);
    return $digits == array_reverse($digits);
  }

  public static function next(int $current): int
  {
    while (!static::is_pal(++$current)) {}
    return $current;
  }
}
