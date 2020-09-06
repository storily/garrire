<?php

require_once('../vendor/autoload.php');

function error_dump($arg): void
{
  error_log(var_export($arg, true));
}

$path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
$method = strtolower($_SERVER['REQUEST_METHOD']);

$attempts = [];
$segs = array_filter(explode('/', $path));
foreach (array_keys($segs) as $i) {
  $parts = array_slice($segs, 0, $i);
  $attempts[] = implode('\\', array_map(fn ($seg) => ucfirst(strtolower($seg)), $parts));
}

$attempts = array_reverse($attempts);

// TODO: navigation-style resolving (aliases), rewriting, and restricting
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
