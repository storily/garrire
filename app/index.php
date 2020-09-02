<?php

require_once('../vendor/autoload.php');

function error_dump($arg): void
{
  error_log(var_export($arg, true));
}

$path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
$method = strtolower($_SERVER['REQUEST_METHOD']);

// TODO: navigation-style resolving, rewriting, and restricting
$controller = '\\App\\Controllers' . implode('\\', array_map(fn ($seg) => ucfirst(strtolower($seg)), explode('/', $path)));

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
