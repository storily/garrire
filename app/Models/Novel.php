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
		$res = $client->get("/projects/{$this->novel}?include=project-challenges");

		$data = json_decode($res->getBody()->getContents(), true);
		$project = $data['data']['attributes'] ?? null;
		if (!is_array($project)) throw new \Exception('invalid data returned from project');

		$project['goals'] = array_map(
			fn ($goal) => $goal['attributes'],
			array_filter(
				$data['included'] ?? [],
				fn ($item) => ($item['type'] ?? null) == 'project-challenges'
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
		return ($this->project_data()['unit-count'] ?? 0) - $this->accounted_words();
	}

	private static function dateInTz(string $date, \DateTimeZone $tz): \DateTimeImmutable
	{
		return (new \DateTimeImmutable($date))
			->setTimezone($tz)
			->setTime(0, 0, 0, 0)
			->setTimezone($tz);
	}

	public function period(): object
	{
		$goal = $this->current_goal() ?? [
			'starts-at' => '2021-11-01',
			'ends-at' => '2021-11-30',
		];

		$tz = new \DateTimeZone('Pacific/Auckland');

		$start = static::dateInTz($goal['starts-at'], $tz);
		$finish = static::dateInTz($goal['ends-at'], $tz)->modify('+1 day');
		$now = (new \DateTimeImmutable)->setTimezone($tz);
		$today = $now->setTime(0, 0, 0, 0)->setTimezone($tz);

		$length = $start->diff($finish);
		$gone = $start->diff($now);
		$left = $now->diff($finish);
		$started = $start <= $now;

		// Account for DST
		if ($length->h < 0) {
			$normalised_finish = $finish->add(new \DateInterval('PT' . abs($length->h) . 'H'));
			$length = $start->diff($normalised_finish);
			$left = $now->diff($normalised_finish);
		}

		$over = !!$left->invert;

		return (object) compact('start', 'finish', 'now', 'today', 'length', 'gone', 'left', 'over', 'started');
	}

	public function current_goals(): array
	{
		$goals = $this->project_data()['goals'];
		if (empty($goals)) return [];

		$active = array_filter($goals, fn ($goal) => static::goal_is_current($goal));
		if (!empty($active)) return $active;

		usort($goals, fn ($a, $b) => $a['starts-at'] <=> $b['starts-at']);
		return [$goals[0]];
	}

	public function current_goal(): ?array
	{
		$goals = $this->current_goals();
		usort($goals, fn ($a, $b) => $a['starts-at'] <=> $b['starts-at']);
		return $goals[0] ?? null;
	}

	private static function goal_is_current($goal): bool
	{
		if (empty($goal['starts-at']) || empty($goal['ends-at'])) return true;

		$start = (new \DateTime($goal['starts-at']))->format('Y-m-d');
		$end = (new \DateTime($goal['ends-at']))->format('Y-m-d');
		$now = (new \DateTime)->format('Y-m-d');

		return ($now >= $start && $now <= $end);
	}

	public function goal(): int
	{
		return $this->goal_override ?? $this->default_goal();
	}

	public function default_goal(): int
	{
		return $this->current_goal()['goal'] ?? 50000;
	}

	// words already accounted for in past goals
	public function accounted_words(): int
	{
		$goals = $this->project_data()['goals'];
		if (empty($goals)) return 0;

		$past_goals = array_filter($goals, fn ($goal) => !static::goal_is_current($goal));
		if (count($past_goals) == count($goals)) {
			usort($past_goals, fn ($a, $b) => $a['starts-at'] <=> $b['starts-at']);
			array_shift($past_goals);
		}

		return array_sum(array_map(
			fn ($goal) => $goal['current-count'],
			$past_goals
		));
	}

	public function progress(): object
	{
		$count = $this->wordcount();
		$goal = $this->goal();
		$period = $this->period();

		$per_day = ($goal / $period->length->days);
		$goal_today = $period->started ? ((int) round($per_day * min($period->gone->days + 1, $period->length->days))) : 0;

		$secs = $period->now->getTimestamp() - $period->today->getTimestamp();
		$day_secs = 60*60*24;
		$goal_live = $period->started ? ((int) round($goal_today - $per_day * (max(0, $day_secs - $secs) / $day_secs))) : 0;

		$goal_live = min($goal_live, $goal_today);

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
