<?php

declare(strict_types=1);
require_once('bootstrap.php');

try {
	$method = strtolower($_SERVER['REQUEST_METHOD']);
	$path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);

	if (str_starts_with($path, '/command/')) {
		Models\Command::handle($method, preg_replace('|^/command|', '', $path));
	} else if (preg_match('|^/server/\d+/join/\d+$|', $path)) {
		(new Controllers\Membership)->join();
	} else if ($path == '/check/double-bang') {
		(new Controllers\Check)->double_bang();
	} else {
		throw new \Exceptions\End(404);
	}
} catch (\Exceptions\End $end) {
	if (!headers_sent()) http_response_code($end->status);
	exit;
} catch (\Throwable $err) {
	http_response_code(500);
	if (ENVIRONMENT === PRODUCTION) error_log($err->getMessage().' '.$err->getTraceAsString());
	else dump($err);
}
