<?php

declare(strict_types=1);
require_once('bootstrap.php');

$path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
$method = strtolower($_SERVER['REQUEST_METHOD']);

const ALIASES = [
  'die' => 'Roll',
  'dice' => 'Roll',
  '8ball' => 'EightBall',
  'color' => 'Colour',
  'motivation' => 'Motivate',
  'advice' => 'Motivate',
  'pal' => 'Palindrome',
];

$segs = array_filter(explode('/', strtolower($path)));
$ksegs = array_keys($segs);

$attempts = [];
foreach ($ksegs as $i) {
  $parts = array_slice($segs, 0, $i);
  $attempts[] = implode('\\', array_map(fn ($seg) => ucfirst($seg), $parts));
}

// Run through the segments, each time replacing one of them with the aliasing.
foreach ($ksegs as $k) {
  $subbed_segs = array_map(function ($j) use (&$segs) {
    $sub = $segs[$j];
    return ALIASES[$sub] ?? $sub;
  }, $ksegs);

  $sub_attempts = [];
  foreach ($ksegs as $i) {
    $parts = array_slice($subbed_segs, 0, $i);
    $sub_attempts[] = implode('\\', array_map(fn ($seg) => ucfirst($seg), $parts));
  }

  $attempts = array_unique(array_merge($attempts, $sub_attempts));
}

usort($attempts, fn ($a, $b) => strlen($a) <=> strlen($b));

foreach (array_reverse($attempts) as $attempt) {
  $controller = '\\Controllers\\' . $attempt;
  if (class_exists($controller)) break;
}

try {
  $instance = new $controller;
} catch (\Throwable $err) {
  error_dump("$err");
  http_response_code(404);
  exit;
}

try {
  $instance->$method();
} catch (\Exceptions\End $end) {
  exit;
} catch (\Throwable $err) {
  error_dump("$err");
  http_response_code(500);
  echo "$err";
}
