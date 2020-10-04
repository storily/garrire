<?php

declare(strict_types=1);
namespace Models;

class Command extends Model
{
	protected array $params = [];

	public function exact(): bool
	{
		return $this->mode == 'exact';
	}

	public function glob(string $path): bool
	{
		if ($this->mode != 'glob') return false;

		// TODO: match $path to $this->path using globs
		// if matches, fill $this->params with each globbing
		// and recompute $this->redirect if present

		return true;
	}

	public function initiate(): \Controller
	{
		if ($this->redirect) {
			http_response_code($this->redirect_code ?? 307);
			header("Location: {$this->redirect}");
			exit;
		}

		if (!$this->controller)
			throw new \Exception("No redirect, no controller on {$this->id}");

		$controller = '\\Controllers\\Command\\' . $this->controller;
		if (!class_exists($controller))
			throw new \Exception("Controller {$this->controller} doesn’t exist for {$this->id}");

		return new $controller;
	}
}
