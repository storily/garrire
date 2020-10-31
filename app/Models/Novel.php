<?php

declare(strict_types=1);
namespace Models;

use GuzzleHttp\Client;

class Novel extends Model
{
	protected $fillable = ['discord_user_id', 'novel'];

	private $_project_data = null;
	public function project_data(bool $reload = false): array
	{
		if ($this->_project_data && !$reload) return $this->_project_data;

		$client = NanowrimoSetting::get_client();
		$res = $client->get("/projects/{$this->novel}");
		if (($code = $res->getStatusCode()) > 299) throw new \Exception("project status call failed with status $code");

		$data = json_decode($res->getBody()->getContents(), true);
		$data = $data['data']['attributes'] ?? null;
		if (!is_array($data)) throw new \Exception('invalid data returned from project');

		return $this->_project_data = $data;
	}

	public function title(): string
	{
		return $this->project_data()['title'] ?? ucfirst($this->project_data()['slug']);
	}

	public function wordcount(): int
	{
		return $this->project_data()['unit-count'] ?? 0;
	}
}
