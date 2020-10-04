<?php

declare(strict_types=1);
require_once('bootstrap.php');

$path = parse_url($_SERVER['REQUEST_URI'] ?? '/roll/d6', PHP_URL_PATH);
$method = strtolower($_SERVER['REQUEST_METHOD'] ?? 'POST');

const ALIASES = [
  'die' => 'Roll',
  'dice' => 'Roll',
  '8ball' => 'EightBall',
  'color' => 'Colour',
  'motivation' => 'Motivate',
  'advice' => 'Motivate',
  'pal' => 'Palindrome',
];

$lcpath = strtolower($path);
$prefix = array_values(array_filter(explode('/', $lcpath)))[0] ?? null;
if (!$prefix) {
	http_response_code(404);
	exit;
}

$commands = Models\Command::query()
	->where(fn ($q) => $q
		->where('mode', '=', 'exact')
		->where(function ($q) use ($path, $lcpath) {
			$q->where('path', '=', $path);
			if ($lcpath !== $path)
				$q->orWhere('path', '=', $lcpath);
		})
	)
	->orWhere(fn ($q) => $q
		->where('mode', '=', 'glob')
		->where('path', 'LIKE', "/{$prefix}/%")
	)
	->get();

$command = null;
foreach ($commands as $command) {
	if ($command->exact() || $command->glob($path) || ($path !== $lcpath ? $command->glob($lcpath) : false)) {
		break;
	}
}

if (!$command) {
	http_response_code(404);
	exit;
}

try {
  $instance = $command->initiate();
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
