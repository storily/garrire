<?php

/// wc (wordcount) - Get your wordcount from the nano website.
///
/// You need to set your novel to setup. The most straightforward
/// way is with the project URL: when you go to your nanowrimo pages
/// and open the main page for your novel, the URL will look like
/// `https://nanowrimo.org/participants/your-name/projects/some-name`.
/// Copy it and tell me about it with `!wc add novel URL`.
///
/// **Your novel needs to be public for me to see it.**
///
/// Thereafter, get your wordcount and stats with `!wc`.
///
/// If you're writing multiple things at once you can add your other
/// novels with more `!wc add novel NOVEL`, and remove some with
/// `!wc remove novel NOVEL`, or clear everything with `!wc clear`.
///
/// The goal of novels assigned to the November event is not editable
/// on the site. You can override it here to get correct stats with
/// `!wc set goal NOVEL WORDS`.
///
/// In all commands above, the `NOVEL` identifier can either be the
/// full URL or the last part of it after the slash or any part of
/// the title which is unambiguous.


declare(strict_types=1);
namespace Controllers\Command;

use Models\Novel;

class WordCount extends \Controllers\Controller
{
	public function post(): void
	{
		$this->help();

		$userid = $_SERVER['HTTP_ACCORD_AUTHOR_ID'] ?? $_SERVER['HTTP_ACCORD_USER_ID'] ?? null;
		if (!$userid) throw new \Exception('no user id, cannot proceed?!');

		$debug = false;

		if (!empty($arg = $this->argument())) {
			$args = preg_split('/\s+/', $arg);
			switch (strtolower(trim("{$args[0]} {$args[1]}"))) {
			case '':
				break;

			case 'set goal':
			case 'add goal':
				$novel = Novel::where('discord_user_id', $userid)->first();
				if (!$novel) {
					$this->reply('🛑 no novel set', null, true);
					return;
				}

				$goal = (int) str_replace('k', '000', $args[2] ?? '');

				if ($goal) {
					$novel->goal_override = $goal;
					$novel->save();
				} else {
					$this->reply('that doesn’t look like a number to me', null, true);
					return;
				}
				break;

			case 'unset goal':
			case 'remove goal':
				$novel = Novel::where('discord_user_id', $userid)->first();
				if (!$novel) {
					$this->reply('🛑 no novel set', null, true);
					return;
				}

				$novel->goal_override = null;
				$novel->save();
				break;

			case 'set novel':
			case 'add novel':
				$novel = $args[2];
				if (str_starts_with($novel, 'https:')) $novel = last(explode('/', $novel));
				Novel::updateOrCreate(['discord_user_id' => $userid], ['novel' => $novel]);
				break;

			case 'debug':
				$debug = true;
				break;

			default:
				$this->show_help();
			}
		}

		$novel = Novel::where('discord_user_id', $userid)->first();
		if (!$novel) $this->show_help();

		try {
			$title = $novel->title();
			$count = $novel->wordcount();
			$goal = $novel->goal();
			$progress = $novel->progress();

			$deets = implode(', ', array_filter([
				round($progress->percent, 2) . '% done',
				($progress->percent >= 100 ? null : static::on_track($progress->today->diff ?? null, ' today')),
				($progress->percent >= 100 ? null : static::on_track($progress->live->diff ?? null, ' live')),
				($goal == 50000 ? null : (static::numberk($goal).' goal')),
				(Palindrome::is_pal($count) ? null : ((Palindrome::next($count) - $count) . ' to next pal')),
			]));

			if ($debug) {
				$period = $novel->period();
				$period = [
					'start' => $period->start->format(\DateTime::RFC3339),
					'finish' => $period->finish->format(\DateTime::RFC3339),
					'now' => $period->now->format(\DateTime::RFC3339),
					'today' => $period->today->format(\DateTime::RFC3339),

					'length_days' => $period->length->days * ($period->length->invert ? -1 : 1),
					'gone_days' => $period->gone->days * ($period->gone->invert ? -1 : 1),
					'left_days' => $period->left->days * ($period->left->invert ? -1 : 1),

					'length_hours' => $period->length->h,

					'length_inverted' => !!$period->length->invert,
					'gone_inverted' => !!$period->gone->invert,
					'left_inverted' => !!$period->left->invert,

					'over' => $period->over,
				];
				$deets .= "\n\n```\n".var_export(compact('title', 'count', 'goal', 'progress', 'period'), true)."\n```";
			}

			$count = static::pretty($count, $progress->percent ?? null);
			$this->reply("“{$title}”: **{$count}** words ($deets)", null, true);
		} catch (\GuzzleHttp\Exception\ClientException $err) {
			$res = $err->getResponse();
			$this->reply("⚠️ Error: {$res->getStatusCode()} {$res->getReasonPhrase()}", null, true);
		}
	}

	private static function numberk(int $count): string
	{
		if ($count < 1000) {
			return "{$count}";
		} else if ($count < 10000) {
			return round($count / 1000, 1).'k';
		} else {
			return round($count / 1000).'k';
		}
	}

	private static function on_track(?int $diff, string $append = ''): ?string
	{
		if (is_null($diff)) return null;

		if ($diff == 0) {
			return 'on track';
		} else if ($diff < 0) {
			$diff = abs($diff);
			$state = 'behind';
		} else {
			$state = 'ahead';
		}

		return static::numberk($diff) . " {$state}{$append}";
	}

	public static function pretty($count, ...$args): string
	{
		[$deco, $oced] = static::effects($count, ...$args);
		return "{$deco}{$count}{$oced}";
	}

	public static function effects(int $count, ?float $percent = null): array
	{
		$single_digit = strlen("$count") == 1;

		$deco = '';
		if (preg_match('/^(\d)\1+$/', "$count")) $deco .= '🌉';
		if (preg_match('/^(\d)(\d)\2+\1$/', "$count")) $deco .= '🌆';
		if (!$single_digit && Palindrome::is_pal($count)) $deco .= '✨';
		else if (preg_match('/(\d)\1.*(\d)\2/', "$count")) $deco .= '👀';
		else if (preg_match('/(\d{2}).*\1/', "$count")) $deco .= '💞';
		if (preg_match('/^\d0+$/', "$count")) $deco .= '💫';
		if (preg_match('/^\d+0{2,}$/', "$count")) $deco .= '🌻';
		if (!$single_digit && static::is_incrnum($count)) $deco .= '🌌';
		if (round(log($count, 2)) == log($count, 2)) $deco .= '🤖';
		if (static::is_prime($count)) $deco .= '🥇';
		if (static::is_fibonacci($count)) $deco .= '🤌';
		if (static::is_weird($count)) $deco .= '👾';
		if (static::is_square($count)) $deco .= '🆒';
		if (static::is_perfect($count)) $deco .= '💯';
		if (static::is_now($count)) $deco .= '🕓';
		if (!is_null($percent) and $percent >= 100) $deco .= '🎆';
		if ($count <= 0) $deco = '';

		$oced = implode('', array_reverse(mb_str_split($deco)));
		return [$deco, $oced];
	}

	private static function is_incrnum(int $count): bool
	{
		$revrs = (int) implode('', array_reverse(str_split("$count")));
		return static::is_incrnum_single($count, 0)
			|| static::is_incrnum_single($count, 1)
			|| static::is_incrnum_single($count, 2)
			|| static::is_incrnum_single($count, 3)
			|| static::is_incrnum_single($count, 4)
			|| static::is_incrnum_single($count, 5)
			|| static::is_incrnum_single($count, 6)
			|| static::is_incrnum_single($count, 7)
			|| static::is_incrnum_single($count, 8)
			|| static::is_incrnum_single($revrs, 8)
			|| static::is_incrnum_single($revrs, 7)
			|| static::is_incrnum_single($revrs, 6)
			|| static::is_incrnum_single($revrs, 5)
			|| static::is_incrnum_single($revrs, 4)
			|| static::is_incrnum_single($revrs, 3)
			|| static::is_incrnum_single($revrs, 2)
			|| static::is_incrnum_single($revrs, 1)
			|| static::is_incrnum_single($revrs, 0);
	}

	private static function is_incrnum_single(int $count, int $offset = 0): bool
	{
		if (strlen("$count") == 1) return false;
		foreach (str_split("$count") as $i => $n) {
			if (((int) $n) !== (($i + 1 + $offset) % 10)) return false;
		}

		return true;
	}

	private static function is_prime(int $count): bool
	{
        $intsqrt = floor(sqrt($count));
        for ($i = 2; $i <= $intsqrt; $i += 1)
			if ($count % $i == 0) return false;

		return true;
	}

	private static function is_square(int $n): bool
	{
		return pow(floor(sqrt($n)), 2) == $n;
	}

	private static function is_perfect(int $count): bool
	{
		// https://oeis.org/A000396
		return in_array($count, [6, 28, 496, 8128]);
	}

	private static function is_fibonacci(int $count): bool
	{
		# https://en.wikipedia.org/wiki/Fibonacci_number#Identification
		return static::is_square(5 * pow($count, 2) + 4) || static::is_square(5 * pow($count, 2) - 4);
	}

	private static function is_weird(int $count): bool
	{
		// https://oeis.org/A006037
		return in_array($count,
			[70,836,4030,5830,7192,7912,9272,10430,10570,
			 10792,10990,11410,11690,12110,12530,12670,13370,
			 13510,13790,13930,14770,15610,15890,16030,16310,
			 16730,16870,17272,17570,17990,18410,18830,18970,
			 19390,19670]
		);
	}

	private static function is_now(int $count): bool
	{
		// probably impossible to hit, so surely someone will in a few years
		return $count == (
			(new \DateTime)->getTimestamp() -
			(new \DateTime('today'))
				->setTimezone(new \DateTimeZone('Pacific/Auckland'))
				->getTimestamp()
		);
	}
}
