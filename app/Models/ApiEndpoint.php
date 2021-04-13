<?php

declare(strict_types=1);
namespace Models;

class ApiEndpoint extends Model
{
	protected array $params = [];

	public function initiate(): \Controllers\ApiController
	{
		if ($this->redirect) {
			http_response_code($this->redirect_code ?? 307);
			header("Location: {$this->redirect}");
			exit;
		}

		if (!$this->controller)
			throw new \Exception("No redirect, no controller on {$this->id}");

		$controller = '\\Controllers\\ApiEndpoint\\' . $this->controller;
		if (!class_exists($controller))
			throw new \Exception("Controller {$this->controller} doesn’t exist for {$this->id}");

		return new $controller;
	}

	public static function handle(string $method, string $path)
	{
		$command = self::query()
			->where('path', '=', strtolower($path))
			->first();

		if (!$command) throw new \Exceptions\End(404);

		try {
			$instance = $command->initiate();
		} catch (\Throwable $err) {
			if (ENVIRONMENT == PRODUCTION) error_log($err->getMessage().' '.$err->getTraceAsString());
			else dump($err);
			throw new \Exceptions\End(404);
		}

		$instance->$method();
	}
}
