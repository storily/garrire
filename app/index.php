<?php

declare(strict_types=1);
require_once('bootstrap.php');

$method = strtolower($_SERVER['REQUEST_METHOD']);
$path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);

try {
	if (str_starts_with($path, '/command/')) {
	Models\Command::handle($method, preg_replace('|^/command|', '', $path));
else
	http_response_code(404);
} catch (\Exceptions\End $end) {
	if (!headers_sent()) http_response_code($end->status);
	exit;
} catch (\Throwable $err) {
	dump($err);
	http_response_code(500);
	die("$err");
}
