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

	public function period(): object
	{
		$tz = new \DateTimeZone('Pacific/Auckland');

		$start = (new \DateTimeImmutable('1 Nov 2020'))->setTimezone($tz);
		$finish = (new \DateTimeImmutable('1 Dec 2020'))->setTimezone($tz);
		$now = (new \DateTimeImmutable)->setTimezone($tz);
		$today = $now->setTime(0, 0, 0, 0)->setTimezone($tz);

		$length = $finish->diff($start);
		$gone = $start->diff($now);
		$left = $now->diff($start);
		$over = $left <= 0;

		return (object) compact('start', 'finish', 'now', 'today', 'length', 'gone', 'left', 'over');
	}

	public function goal(): int
	{
		return 50000;
	}

	public function default_goal(): int
	{
		return 50000;
	}

	public function progress(): object
	{
		$count = $this->wordcount();
		$goal = $this->goal();
		$period = $this->period();

		$per_day = ($goal / $period->length->days);
		$goal_today = (int) round($per_day * ($period->gone->days + 1));

		$secs = $period->now->getTimestamp() - $period->today->getTimestamp();
		$day_secs = 60*60*24;
		$goal_live = (int) round($goal_today - $per_day * (($day_secs - $secs) / $day_secs));

		dump([$period->today->format(\DateTime::RFC3339), $secs, $day_secs, $per_day, $goal_today]);

		return (object) [
			'percent' => 100 * $count / $goal,
			'today' => (object) [
				'goal' => $goal_today,
				'diff' => $count - $goal_today,
			],
			'live' => (object) [
				'goal' => $goal_live,
				'diff' => $count - $goal_live,
			],
		];
	}
}
