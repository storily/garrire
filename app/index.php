<?php

require_once('../vendor/autoload.php');

function error_dump($arg): void
{
  error_log(var_export($arg, true));
}

$path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
$method = strtolower($_SERVER['REQUEST_METHOD']);

const ALIASES = [
  'die' => 'roll',
  'dice' => 'roll',
];

$segs = array_filter(explode('/', strtolower($path)));
$ksegs = array_keys($segs);

$attempts = [];
foreach ($ksegs as $i) {
  $parts = array_slice($segs, 0, $i);
  $attempts[] = implode('\\', array_map(fn ($seg) => ucfirst($seg), $parts));
}

$attempts = array_reverse($attempts);

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

  $attempts = array_merge($attempts, array_reverse($sub_attempts));
}

foreach ($attempts as $attempt) {
  $controller = '\\App\\Controllers\\' . $attempt;
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
} catch (\App\End $end) {
  exit;
} catch (\Throwable $err) {
  error_dump("$err");
  http_response_code(500);
  echo "$err";
}
