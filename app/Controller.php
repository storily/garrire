<?php

namespace App;

class Controller
{
  protected $payload = null;
  protected $command = null;

  public function __construct()
  {
    $body = file_get_contents('php://input');
    if (!empty($body)) {
      $payload = json_decode($body, true);
      if (!empty($payload['command'])) {
        $this->command = $payload['command'];
        $this->payload = $payload['message'];
      } else {
        $this->payload = $payload;
      }
    }
  }

  protected function argument(): ?string
  {
    if (!$this->command) return null;

    $content = $this->payload['content'] ?? null;
    if (!$content) return null;

    // TODO: find instead the first pos of the first command[]
    // and determine the prefix to be anything before, to account
    // for multichar prefixes
    $prefix = $content[0] ?? null;
    if (!$prefix) return null;

    $known = $prefix . implode(' ', $this->command);
    if (strpos($content, $known) !== 0) return null;
    return trim(substr($content, strlen($known)));
  }

  protected function redirect(string $url): void
  {
    http_response_code(302);
    header('location: '.$url);
    throw new End;
  }

  protected function reply_once(string $reply): void
  {
    header('content-type: text/plain');
    echo $reply;
    throw new End;
  }
}
