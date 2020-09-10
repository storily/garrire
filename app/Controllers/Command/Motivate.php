<?php

/// motivate (motivation, advice) - Motivational messages as a service.

namespace App\Controllers\Command;

class Motivation extends \App\Controller
{
  const MOTIVES = [
    'Reality cannot destroy you.',
    'One day you will find the right words.',
    'Always be a poet, even in prose.',
    'Write to keep civilisation from destroying itself.',
    'Mmmm… I love deadlines. I like the whooshing sound they make as they fly by.',
    'Someone is sitting in the shade today because they planted a tree a long time ago… and then worked hard to keep it alive.',
    'Write. If it’s good, you’ll find out. If it’s not, throw it out of the window.',
    'Take a chance. It may be bad, but it’s the only way you can do anything really good.',
    'Wake up. Kick ass. Repeat.',
    'You’re gonna make it happen!',
    'Your words are magic. Recharge! Then cast again',
    'Imagine signing a copy of your own book. Now finish writing it.',
    'Someone out there _needs_ your story.',
  ];

  public function post(): void
  {
    $this->help();
    if (empty($this->argument())) $this->show_help();

    $this->reply(static::MOTIVES[array_rand(static::MOTIVES)], null, true);
  }
}
