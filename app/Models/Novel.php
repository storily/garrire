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
		$res = $client->get("/projects/{$this->novel}?include=challenges");

		$data = json_decode($res->getBody()->getContents(), true);
		$project = $data['data']['attributes'] ?? null;
		if (!is_array($project)) throw new \Exception('invalid data returned from project');

		$project['goals'] = array_map(
			fn ($goal) => $goal['attributes'],
			array_filter(
				$data['included'] ?? [],
				fn ($item) => ($item['type'] ?? null) == 'challenges'
			)
		);

		return $this->_project_data = $project;
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

	public function current_goals(): array
	{
		return array_filter($this->project_data()['goals'], function ($goal) {
			if (empty($goal['starts-at']) || empty($goal['ends-at'])) return true;

			$start = new \DateTime($goal['starts-at']);
			$end = new \DateTime($goal['ends-at']);
			$now = new \DateTime;

			return ($now >= $start && $now <= $end);
		});
	}

	public function goal(): int
	{
		return $this->goal_override ?? $this->default_goal();
	}

	public function default_goal(): int
	{
		$goals = array_map(fn ($goal) => $goal['default-goal'], $this->current_goals());
		rsort($goals);
		return $goals[0] ?? 50000;
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
