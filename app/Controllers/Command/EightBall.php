<?php

/// 8ball - Ask a question, get an answer.
///
/// Answers are provided “as is,” without warranty of any kind, express or
/// implied, including but not limited to the warranties of merchantibility,
/// fitness for a or any particular purpose, making some kind of sense,
/// consistency, repeatability, and general seriousness. In no event shall
/// the answers or providers thereof be liable for any claim, damages, psychic
/// trauma, character death or maiming, or other issues for which you may or
/// may not have a contingency plan already, whether in an action, inaction, or
/// side-effect arising from, out of, or in connection with the answer or the
/// use of or other dealings with the answer.

namespace App\Controllers\Command;

class EightBall extends \App\Controller
{
  const HELP_NAME = '8ball';

  const ANSWERS = [
    'It is decidedly so.',
    'Without a doubt.',
    'Yes – definitely.',
    'You may rely on it.',
    'As I see it, yes.',
    'Most likely.',
    'Outlook good.',
    'Signs point to yes.',
    'Yes.',
    'Reply hazy, try again.',
    'Ask again later.',
    'Better not tell you now.',
    'Cannot predict now.',
    'Concentrate and ask again.',
    'Don’t count on it.',
    'My reply is no.',
    'My sources say no.',
    'Outlook not so good.',
    'Very doubtful.',
    'It is certain.',
  ];

  public function post(): void
  {
    $this->help();
    if (empty($this->argument())) $this->show_help();

    $this->reply(static::ANSWERS[array_rand(static::ANSWERS)], null, true);
  }
}
