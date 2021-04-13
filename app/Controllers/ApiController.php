<?php

declare(strict_types=1);
namespace Controllers;

class ApiController
{
	protected $payload = null;

	public function __construct()
	{
		$body = file_get_contents('php://input');
		if (!empty($body)) {
			$this->payload = json_decode($body, true, 512, JSON_THROW_ON_ERROR);
		}
	}

	private $type_sent = false;
	protected function send_type(string $type): void
	{
		if (!$this->type_sent) {
			header("content-type: $type");
			$this->type_sent = $type;
		} else if ($this->type_sent != $type) {
			throw new \Exceptions\ReplyTypeMismatch($this->type_sent, $type);
		}
	}

	protected function reply(mixed $content): void
	{
		$this->send_type('application/json');
		echo json_encode($content);
	}
}
